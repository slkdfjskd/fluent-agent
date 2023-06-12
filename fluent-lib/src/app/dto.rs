use crate::common;
use crate::common::error;
use crate::common::error::Code;
use crate::common::types::ReqType;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ResultDTO {
    pub code: Code,
    pub msg: String,
}

impl From<common::error::Result> for ResultDTO {
    fn from(r: common::error::Result) -> Self {
        match r {
            Ok(_) => ResultDTO {
                code: Code::OK,
                msg: "".to_string(),
            },
            Err(e) => ResultDTO {
                code: e.code,
                msg: e.msg,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct LibInfo {
    pub version: String,
    pub build_num: i64,
    pub build_at: String,
    pub commit_hash: String,
    pub code: Code,
    pub req_type: Option<ReqType>,
}

#[derive(Debug, Clone)]
pub struct ProtoService {
    pub name: String,
    pub methods: Vec<ProtoMethod>,
}

#[derive(Debug, Clone)]
pub struct ProtoMethod {
    pub name: String,
    pub req_json: String,
}

#[derive(Debug, Clone)]
pub struct NavProjectDTO {
    pub id: i64,
    pub project_id: i64,
    pub project_name: String,
    pub req_type: ReqType,
    pub services: Vec<NavServiceDTO>, // GPRC
    pub requests: Vec<NavRequestDTO>, // HTTP
    pub order_no: i32,
}

#[derive(Debug, Clone)]
pub struct NavServiceDTO {
    pub name: String,
    pub requests: Vec<NavRequestDTO>,
}

#[derive(Debug, Clone)]
pub struct NavRequestDTO {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RequestDTO {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub url: String,
    pub req_type: ReqType,
    pub service: String,
    pub method: String,
    pub headers: Vec<EntryDTO>,
    pub params: Vec<EntryDTO>,
    pub req_json: String,
    pub resp_json: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntryDTO {
    pub name: String,
    pub value: String,
}

pub struct GetRequestResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<RequestDTO>,
}

#[derive(Debug, Clone)]
pub struct ProtoFileDTO {
    pub proto_files: Vec<String>,
    pub import_paths: Vec<String>,
}

pub struct ProtoFileResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<ProtoFileDTO>,
}

#[derive(Debug, Clone)]
pub struct ListNavProjectResult {
    pub code: Code,
    pub msg: String,
    pub data: Vec<NavProjectDTO>,
}

#[derive(Debug, Clone)]
pub struct CreateProjectResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<NavProjectDTO>,
}

#[derive(Debug, Clone)]
pub struct ImportProtoResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<NavProjectDTO>,
}

#[derive(Debug, Clone)]
pub struct GetConfigResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GetBatchConfigResult {
    pub code: Code,
    pub msg: String,
    pub data: Vec<EntryDTO>,
}

#[derive(Debug, Clone)]
pub struct SendRequestResult {
    pub code: Code,
    pub msg: String,
    pub info: SendRequestInfo,
    pub resp: Option<ResponseDTO>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendRequestInfo {
    pub infos: Vec<EntryDTO>,
}

#[derive(Debug, Clone)]
pub struct ResponseDTO {
    pub headers: Vec<EntryDTO>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct EnvVariableDTO {
    pub id: i64,
    pub env_name: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct EnvironmentDTO {
    pub env_name: String,
    pub list: Vec<EnvVariableDTO>,
}

#[derive(Debug, Clone)]
pub struct ListEnvironmentResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<Vec<EnvironmentDTO>>,
}

#[derive(Debug, Clone)]
pub struct CreateEnvironmentResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<EnvVariableDTO>,
}

#[derive(Debug, Clone)]
pub struct ListRequestLogResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<ListRequestLogData>,
}

#[derive(Debug, Clone)]
pub struct ListRequestLogData {
    pub keyword: String,
    pub last_id: i64,
    pub page_size: u16,
    pub first_id: i64,
    pub request_logs: Vec<RequestLogDTO>,
}

#[derive(Debug, Clone)]
pub struct RequestLogDTO {
    pub id: i64,
    pub request_id: i64,
    pub base_url: String,
    pub path: String,
    pub error: Option<error::Error>,
    pub info: SendRequestInfo,
    pub request: RequestLogData,
    pub response: Option<RequestLogData>,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct RequestLogData {
    pub metadata: Vec<EntryDTO>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct UniqueIdResult {
    pub code: Code,
    pub msg: String,
    pub data: i64,
}

#[derive(Debug, Clone)]
pub struct GetRequestLogResult {
    pub code: Code,
    pub msg: String,
    pub data: Option<RequestLogDTO>,
}
