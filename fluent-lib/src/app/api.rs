use crate::app::dto::{
    CreateEnvironmentResult, CreateProjectResult, EntryDTO, GetRequestLogResult,
    ListNavProjectResult, ListRequestLogResult, ResultDTO, SendRequestInfo,
};
use crate::app::{dto, info, param, service};
use crate::common::error::Code;
use crate::common::log;
use crate::common::types::ReqType;
use crate::infra::persistence;
use chrono::Local;

use super::assembler;

#[allow(dead_code)]
pub fn init() -> ResultDTO {
    let r = log::init();
    if r.is_err() {
        return r.into();
    }
    persistence::init_db().into()
}

#[allow(dead_code)]
pub fn import_proto(
    project_id: i64,
    proto_files: Vec<String>,
    import_paths: Vec<String>,
) -> dto::ImportProtoResult {
    return service::import_proto(project_id, proto_files, import_paths);
}

#[allow(dead_code)]
pub fn get_proto_file(project_id: i64) -> dto::ProtoFileResult {
    return service::get_proto_file(project_id);
}

#[allow(dead_code)]
pub fn list_nav_project() -> ListNavProjectResult {
    return service::list_nav_project();
}

#[allow(dead_code)]
pub fn create_project(name: String, req_type: ReqType) -> CreateProjectResult {
    return service::create_project(name, req_type);
}

#[allow(dead_code)]
pub fn get_config(key: String) -> dto::GetConfigResult {
    return service::get_config(key);
}

pub fn get_batch_config(keys: Vec<String>) -> dto::GetBatchConfigResult {
    return service::get_batch_config(keys);
}

#[allow(dead_code)]
pub fn put_config(key: String, value: String) -> dto::ResultDTO {
    return service::put_config(key, value);
}

pub fn delete_configs(keys: Vec<String>) -> dto::ResultDTO {
    return service::delete_configs(keys);
}

#[allow(dead_code)]
pub fn delete_project(project_id: i64) -> dto::ResultDTO {
    return service::delete_project(project_id);
}

#[allow(dead_code)]
pub fn update_project_name(project_id: i64, new_name: String) -> dto::ResultDTO {
    return service::update_project_name(project_id, new_name);
}

#[allow(dead_code)]
pub fn get_request(request_id: i64) -> dto::GetRequestResult {
    return service::get_request(request_id);
}

#[allow(dead_code)]
pub fn update_request(request: param::UpdateRequest) -> ResultDTO {
    return service::update_request(request);
}

#[allow(dead_code)]
pub fn send_request(param: param::SendRequest) -> dto::SendRequestResult {
    let result = service::send_request(param);

    match result {
        Ok((r, info)) => dto::SendRequestResult {
            code: Code::OK,
            msg: "".to_string(),
            info,
            resp: Some(assembler::to_response_dto(r)),
        },
        Err(e) => dto::SendRequestResult {
            code: e.code,
            msg: e.msg,
            info: SendRequestInfo { infos: vec![] },
            resp: None,
        },
    }
}

#[allow(dead_code)]
pub fn create_env(param: param::CreateEnvironment) -> CreateEnvironmentResult {
    service::create_env(param)
}

#[allow(dead_code)]
pub fn update_env_variable(param: param::UpdateEnvironment) -> ResultDTO {
    service::update_env_variable(param)
}

#[allow(dead_code)]
pub fn update_env_name(new_name: String, old_name: String) -> ResultDTO {
    service::update_env(new_name, old_name)
}

#[allow(dead_code)]
pub fn delete_env_variable(id: i64) -> ResultDTO {
    service::delete_env_variable(id)
}

#[allow(dead_code)]
pub fn delete_env(env_name: String) -> ResultDTO {
    service::delete_env(env_name)
}

#[allow(dead_code)]
pub fn list_env() -> dto::ListEnvironmentResult {
    service::list_env()
}

#[allow(dead_code)]
pub fn unique_id() -> dto::UniqueIdResult {
    service::unique_id()
}

/// 获取 lib 版本信息
#[allow(dead_code)]
pub fn lib_info() -> dto::LibInfo {
    info::lib_info()
}

#[allow(dead_code)]
pub fn list_next_request_log(
    last_id: i64,
    keyword: String,
    page_size: u16,
) -> ListRequestLogResult {
    service::list_next_request_log(last_id, keyword, page_size)
}

#[allow(dead_code)]
pub fn list_pre_request_log(
    first_id: i64,
    keyword: String,
    page_size: u16,
) -> ListRequestLogResult {
    service::list_pre_request_log(first_id, keyword, page_size)
}

#[allow(dead_code)]
pub fn get_latest_request_log(request_id: i64) -> GetRequestLogResult {
    return service::get_latest_request_log(request_id);
}
