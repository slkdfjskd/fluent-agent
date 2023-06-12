use chrono::Local;

use crate::common::error::Result;
use crate::common::types::ReqType;
use crate::domain::proto::{entity, param};
use crate::domain::tonic::service::request;
use crate::infra::persistence::proto::po;

pub fn to_project_po(project: &entity::Project) -> Result<po::Project> {
    let proto_file = project
        .proto_file
        .as_ref()
        .map_or(Ok("".to_string()), |s| serde_json::to_string(&s))?;
    Ok(po::Project {
        id: project.id,
        name: project.name.clone(),
        req_type: project.req_type.clone(),
        proto_file,
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

pub fn ref_to_project_ref_po<'a>(project: &'a entity::ProjectRef) -> Result<po::ProjectRef<'a>> {
    let proto_file = project
        .proto_file
        .as_ref()
        .map_or(Ok("".to_string()), |s| serde_json::to_string(&s))?;
    Ok(po::ProjectRef {
        id: project.id,
        name: &project.name,
        req_type: &project.req_type,
        proto_file,
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

pub fn to_project_entity(project: po::Project) -> Result<entity::Project> {
    let proto_file = if !project.proto_file.is_empty() {
        serde_json::from_str(&project.proto_file)?
    } else {
        None
    };
    Ok(entity::Project {
        id: project.id,
        name: project.name,
        req_type: project.req_type,
        proto_file,
    })
}

pub fn to_nav_project_entity(nav_project: po::NavProject) -> Result<entity::NavProject> {
    let mut p = entity::NavProject {
        id: nav_project.id,
        project_name: nav_project.project_name,
        project_id: nav_project.project_id,
        req_type: nav_project.req_type,
        order_no: nav_project.order_no,
        services: None,
        requests: None,
    };

    match &p.req_type {
        ReqType::GRPC => {
            p.services = if !nav_project.services.is_empty() {
                Some(serde_json::from_str(&nav_project.services)?)
            } else {
                None
            }
        }
        ReqType::HTTP => {
            p.requests = if !nav_project.services.is_empty() {
                Some(serde_json::from_str(&nav_project.services)?)
            } else {
                None
            }
        }
    }
    Ok(p)
}

pub fn batch_to_nav_project_entity(
    nav_projects: Vec<po::NavProject>,
) -> Result<Vec<entity::NavProject>> {
    let mut result = vec![];
    for p in nav_projects {
        result.push(to_nav_project_entity(p)?)
    }
    return Ok(result);
}

pub fn to_nav_project_ref_po(project: &entity::NavProject) -> Result<po::NavProjectRef> {
    let services = match &project.req_type {
        ReqType::GRPC => project
            .services
            .as_ref()
            .map_or(Ok("".to_string()), |s| serde_json::to_string(&s))?,
        ReqType::HTTP => project
            .requests
            .as_ref()
            .map_or(Ok("".to_string()), |s| serde_json::to_string(&s))?,
    };

    Ok(po::NavProjectRef {
        id: project.id,
        project_name: &project.project_name,
        project_id: project.project_id,
        req_type: &project.req_type,
        services,
        order_no: 0,
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

pub fn ref_to_nav_project_ref_po<'a>(
    project: &'a entity::NavProjectRef,
) -> Result<po::NavProjectRef<'a>> {
    let services = match &project.req_type {
        ReqType::GRPC => project
            .services
            .as_ref()
            .map_or(Ok("".to_string()), |s| serde_json::to_string(&s))?,
        ReqType::HTTP => project
            .requests
            .as_ref()
            .map_or(Ok("".to_string()), |s| serde_json::to_string(&s))?,
    };

    Ok(po::NavProjectRef {
        id: project.id,
        project_name: project.project_name,
        project_id: project.project_id,
        req_type: project.req_type,
        services,
        order_no: 0,
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

// pub fn to_request_po(request: &entity::Request) -> po::Request {
//     let req_json = request.req_json.as_ref().map_or("".to_string(), |s| s.to_string());
//     let resp_json = request.resp_json.as_ref().map_or("".to_string(), |s| s.to_string());
//     po::Request {
//         id: request.id,
//         name: request.name.clone(),
//         project_id: request.project_id,
//         url: request.url.clone(),
//         req_type: request.req_type.clone(),
//         service_name: request.service_name.clone(),
//         method: request.method.clone(),
//         headers: request.headers.clone(),
//         params: request.params.clone(),
//         req_json,
//         resp_json,
//         created_at: Local::now(),
//         updated_at: Local::now(),
//     }
// }

pub fn ref_to_request_ref_po<'a>(request: &'a entity::RequestRef) -> Result<po::RequestRef<'a>> {
    let req_json = request
        .req_json
        .as_ref()
        .map_or("".to_string(), |s| s.to_string());
    let resp_json = request
        .resp_json
        .as_ref()
        .map_or("".to_string(), |s| s.to_string());
    let r = po::RequestRef {
        id: request.id,
        name: request.name,
        project_id: request.project_id,
        url: request.url,
        req_type: request.req_type,
        service: request.service,
        method: request.method,
        headers: serde_json::to_string(&request.headers)?,
        params: serde_json::to_string(&request.params)?,
        req_json,
        resp_json,
        created_at: Local::now(),
        updated_at: Local::now(),
    };
    return Ok(r);
}

pub fn param_to_request_ref_po<'a>(request: &param::RequestRef<'a>) -> Result<po::RequestRef<'a>> {
    let req_json = request
        .req_json
        .as_ref()
        .map_or("".to_string(), |s| s.to_string());
    let resp_json = request
        .resp_json
        .as_ref()
        .map_or("".to_string(), |s| s.to_string());
    let r = po::RequestRef {
        id: request.id,
        name: request.name,
        project_id: 0,
        url: request.url,
        req_type: &ReqType::HTTP,
        service: "",
        method: request.method,
        headers: serde_json::to_string(&request.headers)?,
        params: serde_json::to_string(&request.params)?,
        req_json,
        resp_json,
        created_at: Local::now(),
        updated_at: Local::now(),
    };
    return Ok(r);
}

pub fn batch_to_request_entity(requests: Vec<po::Request>) -> Result<Vec<entity::Request>> {
    let mut result = vec![];
    for req in requests {
        result.push(to_request_entity(Some(req))?.unwrap());
    }
    return Ok(result);
}

pub fn to_request_entity(request: Option<po::Request>) -> Result<Option<entity::Request>> {
    let r = match request {
        None => None,
        Some(r) => {
            let req_json = if r.req_json.is_empty() {
                None
            } else {
                Some(serde_json::from_str(&r.req_json)?)
            };
            let resp_json = if r.resp_json.is_empty() {
                None
            } else {
                Some(serde_json::from_str(&r.resp_json)?)
            };
            Some(entity::Request {
                id: r.id,
                name: r.name,
                project_id: r.project_id,
                url: r.url,
                req_type: r.req_type,
                service: r.service,
                method: r.method,
                headers: serde_json::from_str(&r.headers)?,
                params: serde_json::from_str(&r.params)?,
                req_json,
                resp_json,
            })
        }
    };
    return Ok(r);
}

pub fn batch_ref_to_request_ref_po<'a>(
    requests: &'a Vec<entity::RequestRef>,
) -> Result<Vec<po::RequestRef<'a>>> {
    let mut result = Vec::new();
    for req in requests {
        result.push(ref_to_request_ref_po(req)?)
    }
    return Ok(result);
}
