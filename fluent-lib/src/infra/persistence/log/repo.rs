use crate::common::error::Result;
use crate::domain;
use crate::domain::log::entity::RequestLog;
use crate::infra::persistence::log::{assembler, db};
use crate::infra::persistence::CONN;

pub struct LogRepo {}

impl LogRepo {
    pub fn new() -> Self {
        return LogRepo {};
    }
}

impl domain::log::repo::LogRepo for LogRepo {
    fn create_request_log(&self, log: &RequestLog) -> Result {
        db::create_request_log(&CONN, &assembler::to_request_log_po(log)?)
    }

    fn list_next_request_log(
        &self,
        last_id: i64,
        keyword: &str,
        page_size: u16,
    ) -> Result<Vec<RequestLog>> {
        let logs = db::find_request_logs(&CONN, last_id, keyword, page_size, true)?;

        assembler::batch_to_request_log_entity(logs)
    }

    fn list_pre_request_log(
        &self,
        first_id: i64,
        keyword: &str,
        page_size: u16,
    ) -> Result<Vec<RequestLog>> {
        let logs = db::find_request_logs(&CONN, first_id, keyword, page_size, false)?;

        assembler::batch_to_request_log_entity(logs)
    }

    fn get_latest_request_log(&self, request_id: i64) -> Result<Option<RequestLog>> {
        let mut logs = db::find_request_log_by_request_id(&CONN, request_id, 1)?;
        if logs.is_empty() {
            return Ok(None);
        }
        assembler::to_request_log_entity(Some(logs.remove(0)))
    }

    fn deallocate_request_log(&self, max_retain_count: i32) -> Result {
        db::deallocate_request_log(&CONN, max_retain_count)
    }
}
