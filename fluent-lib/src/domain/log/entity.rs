use crate::common;
use crate::common::error;
use crate::domain::proto::entity;
use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};

// #[derive(Debug, Clone)]
pub struct RequestLog {
    pub id: i64,
    pub request_id: i64,
    pub base_url: String,
    pub path: String,
    pub error: Option<error::Error>,
    pub info: String,
    pub request: Data,
    pub response: Option<Data>,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Data {
    pub metadata: Vec<entity::Entry>,
    pub body: serde_json::Value,
}
