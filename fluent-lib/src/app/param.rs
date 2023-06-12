use crate::app::dto::EntryDTO;
use crate::common::types::ReqType;

#[derive(Debug, Clone)]
pub struct UpdateRequest {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub method: String,
    pub headers: Vec<EntryDTO>,
    pub params: Vec<EntryDTO>,
    pub req_json: String,
    pub resp_json: String,
}

#[derive(Debug, Clone)]
pub struct SendRequest {
    pub request_id: i64,
    pub url: String,
    pub req_type: ReqType,
    pub headers: Vec<EntryDTO>,
    pub params: Vec<EntryDTO>,
    pub req_json: String,
    pub env_name: String,
}

#[derive(Debug, Clone)]
pub struct UpdateEnvironment {
    pub id: i64,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct CreateEnvironment {
    pub env_name: String,
    pub name: String,
    pub value: String,
}
