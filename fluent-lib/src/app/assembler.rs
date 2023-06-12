use crate::app::dto::{
    EntryDTO, EnvVariableDTO, EnvironmentDTO, ListRequestLogData, ListRequestLogResult,
    NavProjectDTO, NavRequestDTO, NavServiceDTO, RequestLogDTO, RequestLogData, ResponseDTO,
    SendRequestInfo,
};
use crate::app::param;
use crate::app::service::unique_id;
use bytes::BufMut;
use linked_hash_map::LinkedHashMap;
use serde_json::Map;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::dto;
use crate::common::error::Result;
use crate::common::time;
use crate::domain::config::entity;
use crate::domain::{proto, tonic};
use crate::{common, domain};

pub fn to_nav_project_list_dto(
    nav_projects: Vec<proto::entity::NavProject>,
) -> Vec<dto::NavProjectDTO> {
    let mut result = vec![];
    for p in nav_projects {
        let new_nav_project = to_nav_project_dto(p);
        result.push(new_nav_project);
    }
    return result;
}

pub fn to_nav_project_dto(nav_project: proto::entity::NavProject) -> dto::NavProjectDTO {
    let services = nav_project.services.map_or_else(|| vec![], |p| p);
    let mut new_services = vec![];
    for s in services {
        let mut new_requests = vec![];
        for r in s.requests {
            let new_request = NavRequestDTO {
                id: r.id,
                name: r.name,
            };
            new_requests.push(new_request);
        }
        let new_service = NavServiceDTO {
            name: s.name,
            requests: new_requests,
        };
        new_services.push(new_service);
    }
    return NavProjectDTO {
        id: nav_project.id,
        project_id: nav_project.project_id,
        project_name: nav_project.project_name,
        req_type: nav_project.req_type,
        services: new_services,
        requests: vec![],
        order_no: nav_project.order_no,
    };
}

pub fn to_proto_file_dto(proto_file: Option<proto::entity::ProtoFile>) -> dto::ProtoFileDTO {
    let proto_file = proto_file.map_or(
        proto::entity::ProtoFile {
            proto_files: vec![],
            import_paths: vec![],
            local_proto_files: vec![],
            local_import_paths: vec![],
        },
        |p| p,
    );
    return dto::ProtoFileDTO {
        proto_files: proto_file.proto_files,
        import_paths: proto_file.import_paths,
    };
}

pub fn to_request_dto(request: Option<proto::entity::Request>) -> Option<dto::RequestDTO> {
    return match request {
        Some(r) => {
            let req_json = r.req_json.map_or("".to_string(), |j| j.to_string());
            let resp_json = r.resp_json.map_or("".to_string(), |j| j.to_string());
            Some(dto::RequestDTO {
                id: r.id,
                project_id: r.project_id,
                name: r.name,
                url: r.url,
                req_type: r.req_type,
                service: r.service,
                method: r.method,
                headers: to_entry_dto(r.headers),
                params: to_entry_dto(r.params),
                req_json,
                resp_json,
            })
        }
        None => None,
    };
}

pub fn to_entry_dto(entries: Vec<proto::entity::Entry>) -> Vec<EntryDTO> {
    let mut result = vec![];
    for e in entries {
        result.push(dto::EntryDTO {
            name: e.name,
            value: e.value,
        })
    }
    return result;
}

pub fn to_entry_ref_entity(entries: &Vec<EntryDTO>) -> Vec<proto::entity::EntryRef> {
    let mut result = vec![];
    for e in entries {
        result.push(proto::entity::EntryRef {
            name: &e.name,
            value: &e.value,
        });
    }
    return result;
}

pub fn to_entry_entity(entries: Vec<EntryDTO>) -> Vec<proto::entity::Entry> {
    let mut result = vec![];
    for e in entries {
        result.push(proto::entity::Entry {
            name: e.name,
            value: e.value,
        });
    }
    return result;
}

pub fn to_request_param(param: &param::UpdateRequest) -> Result<proto::param::RequestRef> {
    let req_json = if param.req_json.is_empty() {
        None
    } else {
        Some(serde_json::from_str(&param.req_json)?)
    };
    let resp_json = if param.resp_json.is_empty() {
        None
    } else {
        Some(serde_json::from_str(&param.resp_json)?)
    };
    return Ok(proto::param::RequestRef {
        id: param.id,
        name: &param.name,
        url: &param.url,
        method: &param.method,
        headers: to_entry_ref_entity(&param.headers),
        params: to_entry_ref_entity(&param.params),
        req_json,
        resp_json,
    });
}

pub fn to_tonic_request(
    param: param::SendRequest,
    req: proto::entity::Request,
    method: Arc<proto::entity::Method>,
) -> Result<tonic::entity::Request> {
    let path = format!("/{}/{}", req.service, req.method);
    Ok(tonic::entity::Request {
        url: param.url,
        path,
        metadata: to_entry_entity(param.headers),
        req_json: serde_json::from_str(&param.req_json)?,
        req_msg: method.request.clone(),
        resp_msg: method.response.clone(),
        env_name: param.env_name,
    })
}

pub fn to_log_request(
    request_id: i64,
    info: &dto::SendRequestInfo,
    req: tonic::entity::Request,
    resp: Option<tonic::entity::Response>,
    err: Option<common::error::Error>,
) -> Result<domain::log::entity::RequestLog> {
    let log = domain::log::entity::RequestLog {
        id: common::uniqueid::generate()?,
        request_id,
        base_url: req.url,
        path: req.path,
        error: err,
        info: serde_json::to_string(info)?,
        request: domain::log::entity::Data {
            metadata: req.metadata,
            body: req.req_json,
        },
        response: resp.map(|r| domain::log::entity::Data {
            metadata: r.metadata,
            body: r.body,
        }),
        created_at: Default::default(),
    };
    Ok(log)
}

pub fn to_response_dto(resp: tonic::entity::Response) -> ResponseDTO {
    return ResponseDTO {
        headers: to_entry_dto(resp.metadata),
        body: resp.body.to_string(),
    };
}

pub fn create_to_env_entity(param: &param::CreateEnvironment) -> entity::EnvironmentRef {
    return entity::EnvironmentRef {
        id: 0,
        env_name: &param.env_name,
        name: &param.name,
        value: &param.value,
    };
}

pub fn update_to_env_entity(param: &param::UpdateEnvironment) -> entity::EnvironmentRef {
    return entity::EnvironmentRef {
        id: param.id,
        env_name: "",
        name: &param.name,
        value: &param.value,
    };
}

pub fn batch_update_to_env_entity(
    param: &Vec<param::UpdateEnvironment>,
) -> Vec<entity::EnvironmentRef> {
    return param.into_iter().map(|p| update_to_env_entity(p)).collect();
}

pub fn batch_to_env_dto(env_list: Vec<entity::Environment>) -> Vec<EnvironmentDTO> {
    let mut env_map = LinkedHashMap::new();
    for env in env_list {
        let v_opt = env_map.get_mut(&env.env_name);
        let v = match v_opt {
            Some(v) => v,
            None => {
                env_map.insert(env.env_name.clone(), vec![]);
                env_map.get_mut(&env.env_name).unwrap()
            }
        };
        v.push(EnvVariableDTO {
            id: env.id,
            env_name: env.env_name,
            name: env.name,
            value: env.value,
        });
    }

    let mut result = vec![];
    for (k, v) in env_map {
        result.push(EnvironmentDTO {
            env_name: k,
            list: v,
        })
    }
    return result;
}

pub fn to_env_dto(env: entity::Environment) -> EnvVariableDTO {
    return EnvVariableDTO {
        id: env.id,
        env_name: env.env_name,
        name: env.name,
        value: env.value,
    };
}

pub fn to_request_log_dto(log: domain::log::entity::RequestLog) -> RequestLogDTO {
    let info = serde_json::from_str(&log.info).map_or(SendRequestInfo { infos: vec![] }, |v| v);
    RequestLogDTO {
        id: log.id,
        request_id: log.request_id,
        base_url: log.base_url,
        path: log.path,
        error: log.error,
        info,
        request: RequestLogData {
            metadata: to_entry_dto(log.request.metadata),
            body: log.request.body.to_string(),
        },
        response: log.response.map(|r| RequestLogData {
            metadata: to_entry_dto(r.metadata),
            body: r.body.to_string(),
        }),
        created_at: time::to_string(&log.created_at),
    }
}

pub fn to_next_request_log_dto(
    origin_last_id: i64,
    keyword: String,
    page_size: u16,
    request_logs: Vec<domain::log::entity::RequestLog>,
) -> ListRequestLogData {
    let mut logs = vec![];
    for log in request_logs {
        logs.push(to_request_log_dto(log))
    }
    let mut last_id = -1;
    let mut first_id = 0;
    if !logs.is_empty() {
        last_id = logs.get(logs.len() - 1).map_or(0, |log| log.id);
        if origin_last_id == 0 {
            first_id = logs.get(0).map_or(0, |log| log.id)
        }
    }
    ListRequestLogData {
        keyword,
        last_id,
        page_size,
        first_id,
        request_logs: logs,
    }
}

pub fn to_pre_request_log_dto(
    origin_first_id: i64,
    keyword: String,
    page_size: u16,
    request_logs: Vec<domain::log::entity::RequestLog>,
) -> ListRequestLogData {
    let mut logs = vec![];
    for log in request_logs {
        logs.push(to_request_log_dto(log))
    }
    let mut first_id;
    if logs.is_empty() {
        first_id = origin_first_id;
    } else {
        first_id = logs.get(0).map_or(0, |log| log.id)
    }
    ListRequestLogData {
        keyword,
        first_id,
        page_size,
        request_logs: logs,
        last_id: 0,
    }
}

pub fn to_batch_config_dto(configs: Vec<entity::Config>) -> Vec<EntryDTO> {
    let mut result = vec![];
    for config in configs {
        result.push(EntryDTO {
            name: config.key,
            value: config.value,
        })
    }
    return result;
}
