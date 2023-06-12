use super::entity;
use crate::common::error::Result;
use crate::domain::proto::param;

pub trait ProtoRepo {
    fn list_nav_project(&self) -> Result<Vec<entity::NavProject>>;

    fn create_project(
        &self,
        project: &entity::ProjectRef,
        nav_project: &entity::NavProject,
    ) -> Result;

    fn find_project(&self, project_id: i64) -> Result<entity::Project>;

    fn find_nav_project_by_project_id(&self, project_id: i64) -> Result<entity::NavProject>;

    fn copy_proto_file(
        &self,
        project_id: i64,
        proto_files: &Vec<String>,
        import_paths: &Vec<String>,
    ) -> Result<entity::ProtoFile>;

    fn batch_create_request(
        &self,
        nav_project: &entity::NavProject,
        project: &entity::Project,
        requests: &Vec<entity::RequestRef>,
        update_requests: &Vec<entity::RequestRef>,
        delete_req_ids: &Vec<i64>,
    ) -> Result;

    fn get_requests_by_project_id(&self, project_id: i64) -> Result<Vec<entity::Request>>;

    fn get_request(&self, request_id: i64) -> Result<Option<entity::Request>>;

    fn delete_project(&self, project_id: i64) -> Result;

    fn update_project_name(&self, project_id: i64, new_name: &str) -> Result;

    fn update_request(&self, request: &param::RequestRef) -> Result;
}
