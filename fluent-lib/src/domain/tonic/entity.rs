use crate::domain::proto::entity;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub path: String,
    pub metadata: Vec<entity::Entry>,
    pub req_json: serde_json::Value,
    pub req_msg: Arc<entity::Message>,
    pub resp_msg: Arc<entity::Message>,
    pub env_name: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub metadata: Vec<entity::Entry>,
    pub body: serde_json::Value,
}
