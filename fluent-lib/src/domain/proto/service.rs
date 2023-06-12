use std::collections::HashMap;
use std::fmt::format;
use std::path::Path;
use std::sync::Arc;

use crate::common;
use crate::common::error::Result;
use crate::common::types::ReqType;
use crate::common::{error, uniqueid};
use crate::domain::proto;
use crate::domain::proto::repo::ProtoRepo;
use crate::domain::proto::{entity, param, PROTO_REPO_WRAP};

use super::protobuf;

pub fn create_project(project_name: String, req_type: ReqType) -> Result<entity::NavProject> {
    let mut nav_project = entity::NavProject {
        id: common::uniqueid::generate()?,
        project_name,
        project_id: 0,
        req_type,
        order_no: 0,
        services: None,
        requests: None,
    };
    let project = entity::ProjectRef {
        id: common::uniqueid::generate()?,
        name: &nav_project.project_name,
        req_type: &nav_project.req_type,
        proto_file: None,
    };
    nav_project.project_id = project.id;
    PROTO_REPO_WRAP
        .get()
        .create_project(&project, &nav_project)?;
    return Ok(nav_project);
}

pub fn import_proto(
    project_id: i64,
    proto_files: Vec<String>,
    import_paths: Vec<String>,
) -> Result<entity::NavProject> {
    debug!(
        "import_proto project_id: {}, proto_files:{:?},  import_paths:{:?}",
        project_id, &proto_files, &import_paths
    );
    let proto_project = parse_proto(&proto_files, &import_paths)?;
    let mut project = PROTO_REPO_WRAP.get().find_project(project_id)?;
    let mut nav_project = PROTO_REPO_WRAP
        .get()
        .find_nav_project_by_project_id(project_id)?;
    let mut requests = vec![];
    let mut update_requests = vec![];
    let mut update_req_key_map = HashMap::new();
    let mut nav_services = vec![];
    let origin_req_map = get_request_map(project_id)?;
    for service in &proto_project.services {
        let mut nav_requests = vec![];
        for method in &service.methods {
            let req_key = req_key(&service.name, &method.name);
            let origin_req = origin_req_map.get(&req_key);
            let mut req_id;
            if let Some(origin_req) = origin_req {
                let req = build_update_request(method, origin_req);
                req_id = req.id;
                update_requests.push(req);
                update_req_key_map.insert(req_key.to_string(), ());
            } else {
                let req = build_create_request(project_id, service, method)?;
                req_id = req.id;
                requests.push(req);
            }
            let nav_req = entity::NavRequest {
                id: req_id,
                name: method.name.clone(),
            };

            nav_requests.push(nav_req);
        }
        let nav_service = entity::NavService {
            name: service.name.clone(),
            requests: nav_requests,
        };
        nav_services.push(nav_service);
    }

    let mut delete_req_ids = vec![];
    for (key, val) in &origin_req_map {
        if !update_req_key_map.contains_key(key) {
            delete_req_ids.push(val.id);
        }
    }

    nav_project.services = Some(nav_services);
    let proto_file =
        PROTO_REPO_WRAP
            .get()
            .copy_proto_file(project.id, &proto_files, &import_paths)?;
    project.proto_file = Some(proto_file);
    PROTO_REPO_WRAP.get().batch_create_request(
        &nav_project,
        &project,
        &requests,
        &update_requests,
        &delete_req_ids,
    )?;
    proto::add_proto_project_map(project_id, proto_project);
    return Ok(nav_project);
}

fn transform_to_entry_ref(entries: &Vec<entity::Entry>) -> Vec<entity::EntryRef> {
    let mut result = vec![];
    for e in entries {
        result.push(entity::EntryRef {
            name: &e.name,
            value: &e.value,
        })
    }
    return result;
}

fn build_update_request<'a>(
    method: &'a Arc<entity::Method>,
    origin_req: &'a entity::Request,
) -> entity::RequestRef<'a> {
    entity::RequestRef {
        id: origin_req.id,
        name: &origin_req.name,
        project_id: origin_req.project_id,
        url: &origin_req.url,
        req_type: &origin_req.req_type,
        service: &origin_req.service,
        method: &origin_req.method,
        headers: transform_to_entry_ref(&origin_req.headers),
        params: transform_to_entry_ref(&origin_req.params),
        req_json: Some(&method.request.json),
        resp_json: Some(&method.response.json),
    }
}

fn build_create_request<'a>(
    project_id: i64,
    service: &'a Arc<entity::Service>,
    method: &'a Arc<entity::Method>,
) -> Result<entity::RequestRef<'a>> {
    Ok(entity::RequestRef {
        id: uniqueid::generate()?,
        name: &method.name,
        project_id,
        url: "",
        req_type: &ReqType::GRPC,
        service: &service.name,
        method: &method.name,
        headers: vec![],
        params: vec![],
        req_json: Some(&method.request.json),
        resp_json: Some(&method.response.json),
    })
}

fn req_key(service: &str, method: &str) -> String {
    format!("{}-{}", service, method)
}

fn get_request_map(project_id: i64) -> Result<HashMap<String, entity::Request>> {
    let requests = PROTO_REPO_WRAP
        .get()
        .get_requests_by_project_id(project_id)?;
    let mut result = HashMap::new();
    for req in requests {
        let key = req_key(&req.service, &req.method);
        result.insert(key, req);
    }
    return Ok(result);
}

pub fn get_proto_file(project_id: i64) -> Result<Option<entity::ProtoFile>> {
    let project = PROTO_REPO_WRAP.get().find_project(project_id)?;
    return Ok(project.proto_file);
}

pub fn update_project_name(project_id: i64, new_name: &str) -> Result {
    return PROTO_REPO_WRAP
        .get()
        .update_project_name(project_id, new_name);
}

pub fn delete_project(project_id: i64) -> Result {
    return PROTO_REPO_WRAP.get().delete_project(project_id);
}

pub fn list_nav_project() -> Result<Vec<entity::NavProject>> {
    return PROTO_REPO_WRAP.get().list_nav_project();
}

pub fn get_request(request_id: i64) -> Result<Option<entity::Request>> {
    return PROTO_REPO_WRAP.get().get_request(request_id);
}

pub fn update_request(request: &param::RequestRef) -> Result {
    return PROTO_REPO_WRAP.get().update_request(request);
}

pub fn load_proto(project_id: i64) -> Result {
    let project = PROTO_REPO_WRAP.get().find_project(project_id)?;
    let proto_file = project
        .proto_file
        .ok_or(error::proto_parse_error_with_str("proto file is empty"))?;
    let proto_project = parse_proto(
        &proto_file.local_proto_files,
        &proto_file.local_import_paths,
    )?;
    proto::add_proto_project_map(project_id, proto_project);
    Ok(())
}

pub fn get_proto_method(
    project_id: i64,
    service_name: &str,
    method_name: &str,
) -> Result<Arc<entity::Method>> {
    let method = proto::get_proto_method(project_id, service_name, method_name);
    match method {
        Some(m) => Ok(m),
        None => {
            load_proto(project_id)?;
            proto::get_proto_method(project_id, service_name, method_name)
                .ok_or(error::invalid_argument_error_with_str("method not found"))
        }
    }
}

fn parse_proto(
    proto_files: &Vec<String>,
    import_paths: &Vec<String>,
) -> Result<entity::ProtoProject> {
    let proto_files = proto_files
        .iter()
        .map(|v| Path::new(v).to_path_buf())
        .collect();
    let import_paths = import_paths
        .iter()
        .map(|v| Path::new(v).to_path_buf())
        .collect();
    let proto_project = protobuf::parse(proto_files, import_paths)?;
    return Ok(proto_project);
}

#[cfg(test)]
mod tests {
    use crate::common::types::ReqType;
    use crate::domain::proto::param;
    use crate::domain::proto::service::{
        create_project, delete_project, get_request, import_proto, list_nav_project, update_request,
    };
    use crate::infra::persistence;

    #[test]
    fn test_create_project() {
        persistence::init_db().unwrap();
        let r = create_project("FluentTest".to_string(), ReqType::GRPC).unwrap();
        println!("navProject:{:?}", r);
    }

    #[test]
    fn test_list_nav_project() {
        let nav_projects = list_nav_project().unwrap();
        println!("nav_project:{:?}", nav_projects);
    }

    #[test]
    fn test_delete_project() {
        persistence::init_db().unwrap();
        delete_project(6937610825259683842).unwrap()
    }

    #[test]
    fn test_get_request() {
        persistence::init_db().unwrap();
        let value = get_request(6946667951835713557).unwrap();
        println!("request:{:?}", value)
    }

    #[test]
    fn test_update_request() {
        persistence::init_db().unwrap();

        let req = param::RequestRef {
            id: 6946667951835713557,
            name: "123",
            url: "http://123.com",
            method: "method",
            headers: vec![],
            params: vec![],
            req_json: Some(serde_json::from_str("{}").unwrap()),
            resp_json: Some(serde_json::from_str("{}").unwrap()),
        };

        let value = update_request(&req).unwrap();
        println!("request:{:?}", value)
    }
}
