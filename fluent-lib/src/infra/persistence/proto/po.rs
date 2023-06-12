use chrono::{DateTime, Local};

use crate::common::types::ReqType;

// #[derive(Debug, Clone)]
// pub struct Map {
//     pub id: i64,
//     pub key: String,
//     pub value: String,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
// }
//
// #[derive(Debug, Clone)]
// pub struct RequestLog {
//     pub id: i64,
//     pub project_id: i64,
//     pub req_id: i64,
//     pub req_json: String,
// }

#[derive(Debug, Clone)]
pub struct NavProject {
    pub id: i64,
    pub project_name: String,
    pub project_id: i64,
    pub req_type: ReqType,
    pub services: String,
    pub order_no: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct NavProjectRef<'a> {
    pub id: i64,
    pub project_name: &'a str,
    pub project_id: i64,
    pub req_type: &'a ReqType,
    pub services: String,
    pub order_no: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub req_type: ReqType,
    pub proto_file: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct ProjectRef<'a> {
    pub id: i64,
    pub name: &'a str,
    pub req_type: &'a ReqType,
    pub proto_file: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub id: i64,
    pub name: String,
    pub project_id: i64,
    pub url: String,
    pub req_type: ReqType,
    pub service: String,
    pub method: String,
    pub headers: String,
    pub params: String,
    pub req_json: String,
    pub resp_json: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct RequestRef<'a> {
    pub id: i64,
    pub name: &'a str,
    pub project_id: i64,
    pub url: &'a str,
    pub req_type: &'a ReqType,
    pub service: &'a str,
    pub method: &'a str,
    pub headers: String,
    pub params: String,
    pub req_json: String,
    pub resp_json: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
