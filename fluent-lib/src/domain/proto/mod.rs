use crate::infra::persistence;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod entity;
pub mod param;
mod protobuf;
pub mod repo;
pub mod service;

lazy_static! {
    pub static ref PROTO_PROJECT_MAP: Mutex<HashMap<i64, entity::ProtoProject>> =
        Mutex::new(HashMap::new());
    pub static ref PROTO_REPO_WRAP: ProtoRepoWrap<persistence::proto::repo::ProtoRepo> = {
        return ProtoRepoWrap {
            proto_repo: persistence::proto::repo::ProtoRepo::new(),
        };
    };
}

pub struct ProtoRepoWrap<T>
where
    T: repo::ProtoRepo,
{
    proto_repo: T,
}

impl<T: repo::ProtoRepo> ProtoRepoWrap<T> {
    pub fn get(&self) -> &T {
        return &self.proto_repo;
    }
}

fn add_proto_project_map(project_id: i64, proto_project: entity::ProtoProject) {
    let mut map = PROTO_PROJECT_MAP.lock().unwrap();
    map.insert(project_id, proto_project);
}

fn get_proto_method(
    project_id: i64,
    service_name: &str,
    method_name: &str,
) -> Option<Arc<entity::Method>> {
    let map = PROTO_PROJECT_MAP.lock().unwrap();
    let project = map.get(&project_id);
    if project.is_none() {
        return None;
    }
    let project = project.unwrap();
    let service = project.service_map.get(service_name);
    if service.is_none() {
        return None;
    }
    let service = service.unwrap();
    let method = service.method_map.get(method_name);
    if method.is_none() {
        return None;
    }
    let method = method.unwrap();
    return Some(method.clone());
}
