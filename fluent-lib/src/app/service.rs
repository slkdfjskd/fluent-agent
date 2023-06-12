use std::collections::HashMap;
use std::thread;

use chrono::{Duration, Local};

use crate::app::dto::{
    CreateProjectResult, EntryDTO, GetRequestLogResult, ListNavProjectResult, ListRequestLogResult,
    UniqueIdResult,
};
use crate::app::{dto, param};
use crate::common::error;
use crate::common::error::Result;
use crate::common::error::{error_with_msg, Code};
use crate::common::types::ReqType;
use crate::domain::log::entity::RequestLog;
use crate::domain::proto::entity::{NavProject, NavService};
use crate::domain::{config, proto, tonic};
use crate::{common, domain};

use super::assembler;

pub fn create_project(name: String, req_type: ReqType) -> CreateProjectResult {
    let result = proto::service::create_project(name, req_type);
    match result {
        Ok(nav_project) => dto::CreateProjectResult {
            code: Code::OK,
            msg: "".to_string(),
            data: Some(assembler::to_nav_project_dto(nav_project)),
        },
        Err(e) => dto::CreateProjectResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}

pub fn import_proto(
    project_id: i64,
    proto_files: Vec<String>,
    import_paths: Vec<String>,
) -> dto::ImportProtoResult {
    let result = proto::service::import_proto(project_id, proto_files, import_paths);
    build_import_proto_result(result)
}

fn build_import_proto_result(result: Result<NavProject>) -> dto::ImportProtoResult {
    match result {
        Ok(nav_project) => dto::ImportProtoResult {
            code: Code::OK,
            msg: "".to_string(),
            data: Some(assembler::to_nav_project_dto(nav_project)),
        },
        Err(e) => dto::ImportProtoResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}

pub fn list_nav_project() -> ListNavProjectResult {
    let result = proto::service::list_nav_project();
    match result {
        Ok(nav_projects) => dto::ListNavProjectResult {
            code: Code::OK,
            msg: "".to_string(),
            data: assembler::to_nav_project_list_dto(nav_projects),
        },
        Err(e) => dto::ListNavProjectResult {
            code: e.code,
            msg: e.msg,
            data: vec![],
        },
    }
}

pub fn get_proto_file(project_id: i64) -> dto::ProtoFileResult {
    let result = proto::service::get_proto_file(project_id);
    match result {
        Ok(p) => dto::ProtoFileResult {
            code: Code::OK,
            msg: "".to_string(),
            data: Some(assembler::to_proto_file_dto(p)),
        },
        Err(e) => dto::ProtoFileResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}

pub fn get_config(key: String) -> dto::GetConfigResult {
    let result = config::service::get_config(&key);
    match result {
        Ok(c) => dto::GetConfigResult {
            code: Code::OK,
            msg: "".to_string(),
            data: c.map(|c| c.value),
        },
        Err(e) => dto::GetConfigResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}

pub fn get_batch_config(keys: Vec<String>) -> dto::GetBatchConfigResult {
    let result = config::service::get_batch_config(&keys);
    match result {
        Ok(configs) => dto::GetBatchConfigResult {
            code: Code::OK,
            msg: "".to_string(),
            data: assembler::to_batch_config_dto(configs),
        },
        Err(e) => dto::GetBatchConfigResult {
            code: e.code,
            msg: e.msg,
            data: vec![],
        },
    }
}

pub fn put_config(key: String, value: String) -> dto::ResultDTO {
    let result = config::service::put_config(&key, &value);
    return result.into();
}

pub fn delete_configs(keys: Vec<String>) -> dto::ResultDTO {
    let result = config::service::delete_configs(&keys);
    return result.into();
}

pub fn delete_project(project_id: i64) -> dto::ResultDTO {
    let result = proto::service::delete_project(project_id);
    return result.into();
}

pub fn update_project_name(project_id: i64, new_name: String) -> dto::ResultDTO {
    let result = proto::service::update_project_name(project_id, &new_name);
    return result.into();
}

pub fn get_request(request_id: i64) -> dto::GetRequestResult {
    let result = proto::service::get_request(request_id);
    match result {
        Ok(req) => dto::GetRequestResult {
            code: Code::OK,
            msg: "".to_string(),
            data: assembler::to_request_dto(req),
        },
        Err(e) => dto::GetRequestResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}

pub fn update_request(request: param::UpdateRequest) -> dto::ResultDTO {
    let result = assembler::to_request_param(&request);
    match result {
        Ok(r) => proto::service::update_request(&r).into(),
        Err(e) => dto::ResultDTO {
            code: e.code,
            msg: e.msg,
        },
    }
}

pub fn send_request(
    mut param: param::SendRequest,
) -> Result<(tonic::entity::Response, dto::SendRequestInfo)> {
    let req = proto::service::get_request(param.request_id)?
        .ok_or(error::invalid_argument_error_with_str("request not found"))?;
    let request_id = param.request_id;
    let method = proto::service::get_proto_method(req.project_id, &req.service, &req.method)?;
    let req = assembler::to_tonic_request(param, req, method)?;

    let start = Local::now();
    let result = tonic::service::request(&req);
    let end = Local::now();

    let mut infos = vec![];
    infos.push(EntryDTO {
        name: "Total Response time".to_string(),
        value: format!("{} ms", end.timestamp_millis() - start.timestamp_millis()),
    });
    let info = dto::SendRequestInfo { infos };

    return match result {
        Ok(resp) => {
            create_request_log(request_id, &info, req, Some(resp.clone()), None);
            Ok((resp, info))
        }
        Err(e) => {
            create_request_log(request_id, &info, req, None, Some(e.clone()));
            Err(e)
        }
    };
}

fn create_request_log(
    request_id: i64,
    info: &dto::SendRequestInfo,
    req: tonic::entity::Request,
    resp: Option<tonic::entity::Response>,
    err: Option<error::Error>,
) {
    let result = assembler::to_log_request(request_id, info, req, resp, err);
    match result {
        Ok(log) => {
            if let Err(e) = domain::log::service::create_request_log(&log) {
                error!("domain::log::service::create_request_log error: {}", e)
            }
        }
        Err(e) => {
            error!("assembler::to_log_request error:{}", e)
        }
    }
}

pub fn create_env(param: param::CreateEnvironment) -> dto::CreateEnvironmentResult {
    let mut env = assembler::create_to_env_entity(&param);
    let result = config::service::create_env(&mut env);
    return match result {
        Ok(env) => dto::CreateEnvironmentResult {
            code: Code::OK,
            msg: "".to_string(),
            data: Some(assembler::to_env_dto(env)),
        },
        Err(e) => dto::CreateEnvironmentResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    };
}

pub fn update_env_variable(param: param::UpdateEnvironment) -> dto::ResultDTO {
    let env = assembler::update_to_env_entity(&param);
    let result = config::service::update_env_variable(env);
    result.into()
}

pub fn update_env(new_name: String, old_name: String) -> dto::ResultDTO {
    let result = config::service::update_env(&new_name, &old_name);
    result.into()
}

pub fn delete_env_variable(id: i64) -> dto::ResultDTO {
    let result = config::service::delete_env_variable(id);
    return result.into();
}

pub fn delete_env(env_name: String) -> dto::ResultDTO {
    let result = config::service::delete_env(&env_name);
    return result.into();
}

pub fn list_env() -> dto::ListEnvironmentResult {
    let result = config::service::list_env();
    match result {
        Ok(r) => dto::ListEnvironmentResult {
            code: Code::OK,
            msg: "".to_string(),
            data: Some(assembler::batch_to_env_dto(r)),
        },
        Err(e) => dto::ListEnvironmentResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}

pub fn unique_id() -> dto::UniqueIdResult {
    let result = common::uniqueid::generate();
    match result {
        Ok(id) => UniqueIdResult {
            code: Code::OK,
            msg: "".to_string(),
            data: id,
        },
        Err(e) => UniqueIdResult {
            code: e.code,
            msg: e.msg,
            data: 0,
        },
    }
}

pub fn list_next_request_log(
    last_id: i64,
    keyword: String,
    page_size: u16,
) -> ListRequestLogResult {
    let result = domain::log::service::list_next_request_log(last_id, &keyword, page_size);
    match result {
        Ok(logs) => ListRequestLogResult {
            code: Code::OK,
            msg: "".to_string(),
            data: Some(assembler::to_next_request_log_dto(
                last_id, keyword, page_size, logs,
            )),
        },
        Err(e) => ListRequestLogResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}

pub fn list_pre_request_log(
    first_id: i64,
    keyword: String,
    page_size: u16,
) -> ListRequestLogResult {
    let result = domain::log::service::list_pre_request_log(first_id, &keyword, page_size);
    match result {
        Ok(logs) => ListRequestLogResult {
            code: Code::OK,
            msg: "".to_string(),
            data: Some(assembler::to_pre_request_log_dto(
                first_id, keyword, page_size, logs,
            )),
        },
        Err(e) => ListRequestLogResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}

pub fn get_latest_request_log(request_id: i64) -> GetRequestLogResult {
    let result = domain::log::service::get_latest_request_log(request_id);
    match result {
        Ok(opt_log) => {
            let mut r = GetRequestLogResult {
                code: Code::OK,
                msg: "".to_string(),
                data: None,
            };
            if let Some(log) = opt_log {
                r.data = Some(assembler::to_request_log_dto(log));
            }
            r
        }
        Err(e) => GetRequestLogResult {
            code: e.code,
            msg: e.msg,
            data: None,
        },
    }
}
