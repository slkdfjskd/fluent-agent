use crate::common::error::Result;
use crate::domain::log::entity;
use crate::domain::log::entity::RequestLog;
use crate::infra::persistence::log::{assembler, db};
use crate::infra::persistence::CONN;

pub trait LogRepo {
    fn create_request_log(&self, log: &entity::RequestLog) -> Result;

    fn list_next_request_log(
        &self,
        last_id: i64,
        keyword: &str,
        page_size: u16,
    ) -> Result<Vec<entity::RequestLog>>;
    fn list_pre_request_log(
        &self,
        first_id: i64,
        keyword: &str,
        page_size: u16,
    ) -> Result<Vec<RequestLog>>;

    fn get_latest_request_log(&self, request_id: i64) -> Result<Option<RequestLog>>;

    fn deallocate_request_log(&self, max_retain_count: i32) -> Result;
}
