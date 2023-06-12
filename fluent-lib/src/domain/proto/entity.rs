use crate::common::types::ReqType;
use protobuf::reflect::FileDescriptor;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ProtoProject {
    pub services: Vec<Arc<Service>>,
    pub service_map: HashMap<String, Arc<Service>>,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub methods: Vec<Arc<Method>>,
    pub method_map: HashMap<String, Arc<Method>>,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub request: Arc<Message>,
    pub response: Arc<Message>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub name: String,
    pub dyn_fd: Arc<FileDescriptor>,
    pub json: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub req_type: ReqType,
    pub proto_file: Option<ProtoFile>,
}

#[derive(Debug, Clone)]
pub struct ProjectRef<'a> {
    pub id: i64,
    pub name: &'a str,
    pub req_type: &'a ReqType,
    pub proto_file: Option<&'a ProtoFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProtoFile {
    pub proto_files: Vec<String>,
    pub import_paths: Vec<String>,
    pub local_proto_files: Vec<String>,
    pub local_import_paths: Vec<String>,
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
    pub headers: Vec<Entry>,
    pub params: Vec<Entry>,
    pub req_json: Option<serde_json::Value>,
    pub resp_json: Option<serde_json::Value>,
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
    pub headers: Vec<EntryRef<'a>>,
    pub params: Vec<EntryRef<'a>>,
    pub req_json: Option<&'a serde_json::Value>,
    pub resp_json: Option<&'a serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct NavProject {
    pub id: i64,
    pub project_name: String,
    pub project_id: i64,
    pub req_type: ReqType,
    pub services: Option<Vec<NavService>>, // GPRC
    pub requests: Option<Vec<NavRequest>>, // HTTP
    pub order_no: i32,
}

#[derive(Debug, Clone)]
pub struct NavProjectRef<'a> {
    pub id: i64,
    pub project_name: &'a str,
    pub project_id: i64,
    pub req_type: &'a ReqType,
    pub services: Option<&'a Vec<NavService>>, // GPRC
    pub requests: Option<&'a Vec<NavService>>, // HTTP
    pub order_no: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavService {
    pub name: String,
    pub requests: Vec<NavRequest>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavRequest {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntryRef<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entry {
    pub name: String,
    pub value: String,
}
