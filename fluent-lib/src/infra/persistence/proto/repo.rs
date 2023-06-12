use std::fs;
use std::path::Path;

use crate::common::error;
use crate::common::error::Result;
use crate::domain;
use crate::domain::proto::entity::{Project, ProjectRef, RequestRef};
use crate::domain::proto::{entity, param};
use crate::infra::persistence::proto::file;
use crate::infra::persistence::{
    proto::{assembler, db},
    CONN,
};

pub struct ProtoRepo {}

impl ProtoRepo {
    pub fn new() -> Self {
        return ProtoRepo {};
    }
}

impl domain::proto::repo::ProtoRepo for ProtoRepo {
    fn list_nav_project(&self) -> Result<Vec<entity::NavProject>> {
        let nav_projects = db::find_nav_project(&CONN)?;
        assembler::batch_to_nav_project_entity(nav_projects)
    }

    fn create_project(&self, project: &ProjectRef, nav_project: &entity::NavProject) -> Result {
        db::create_project_with_tx(
            &CONN,
            &assembler::ref_to_project_ref_po(project)?,
            &assembler::to_nav_project_ref_po(nav_project)?,
        )
    }

    fn find_project(&self, project_id: i64) -> Result<Project> {
        let project = db::find_project_by_id(&CONN, project_id)?
            .ok_or(error::proto_parse_error_with_str("project not found"))?;
        assembler::to_project_entity(project)
    }

    fn find_nav_project_by_project_id(&self, project_id: i64) -> Result<entity::NavProject> {
        let nav_project = db::find_nav_project_by_project_id(&CONN, project_id)?
            .ok_or(error::proto_parse_error_with_str("nav_project not found"))?;
        assembler::to_nav_project_entity(nav_project)
    }

    fn copy_proto_file(
        &self,
        project_id: i64,
        proto_files: &Vec<String>,
        import_paths: &Vec<String>,
    ) -> Result<entity::ProtoFile> {
        let to_root_path = file::data_dir()?.join(project_id.to_string());
        let mut local_proto_files = Vec::new();
        let mut local_import_paths = Vec::new();
        for proto in proto_files {
            let path = Path::new(proto).to_path_buf();
            file::copy_proto_file(&to_root_path, &path)?;
            let file_name = path.file_name().ok_or(error::proto_parse_error_with_str(
                "copy_proto_file file name is none ",
            ))?;
            let p = to_root_path
                .join(file_name)
                .to_str()
                .ok_or(error::proto_parse_error_with_str(
                    "copy_proto_file file name is none",
                ))?
                .to_string();
            local_proto_files.push(p);
        }

        for import in import_paths {
            let mut reside = false;
            // 包含在proto文件的 import_path 直接移到跟目录
            for proto in proto_files {
                if proto.contains(import) {
                    reside = true;
                    break;
                }
            }
            if reside {
                let read_dir = fs::read_dir(import)?;
                for f in read_dir {
                    let a = f?.path();
                    file::copy_proto_file(&to_root_path, &a)?;
                }
                local_import_paths.push(
                    to_root_path
                        .to_str()
                        .ok_or(error::proto_parse_error_with_str(
                            "copy_proto_file file name is none",
                        ))?
                        .to_string(),
                );
            } else {
                let path = Path::new(import).to_path_buf();
                file::copy_proto_file(&to_root_path, &path)?;
                let end_path = file::end_path(&path)?;
                let p = to_root_path
                    .join(&end_path)
                    .to_str()
                    .ok_or(error::proto_parse_error_with_str(
                        "copy_proto_file file name is none",
                    ))?
                    .to_string();
                local_import_paths.push(p);
            }
        }
        let proto_file = entity::ProtoFile {
            proto_files: proto_files.clone(),
            import_paths: import_paths.clone(),
            local_proto_files,
            local_import_paths,
        };
        return Ok(proto_file);
    }

    fn batch_create_request(
        &self,
        nav_project: &entity::NavProject,
        project: &Project,
        requests: &Vec<RequestRef>,
        update_requests: &Vec<RequestRef>,
        delete_req_ids: &Vec<i64>,
    ) -> Result {
        db::batch_create_request_with_tx(
            &CONN,
            &assembler::to_nav_project_ref_po(nav_project)?,
            &assembler::to_project_po(project)?,
            &assembler::batch_ref_to_request_ref_po(requests)?,
            &assembler::batch_ref_to_request_ref_po(update_requests)?,
            delete_req_ids,
        )
    }

    fn get_requests_by_project_id(&self, project_id: i64) -> Result<Vec<entity::Request>> {
        let r = db::find_requests_by_project_id(&CONN, project_id)?;
        assembler::batch_to_request_entity(r)
    }

    fn get_request(&self, request_id: i64) -> Result<Option<entity::Request>> {
        let r = db::find_request_by_id(&CONN, request_id)?;
        assembler::to_request_entity(r)
    }

    fn delete_project(&self, project_id: i64) -> Result {
        db::delete_project(&CONN, project_id)?;
        let root_path = file::data_dir()?.join(project_id.to_string());
        let result = file::delete_proto_file(&root_path);
        if let Err(e) = result {
            error!("delete_proto_file error:{}", e);
        }
        return Ok(());
    }

    fn update_project_name(&self, project_id: i64, new_name: &str) -> Result {
        return db::update_project_name(&CONN, project_id, new_name);
    }

    fn update_request(&self, request: &param::RequestRef) -> Result {
        db::update_request(&CONN, &assembler::param_to_request_ref_po(request)?)
    }
}
