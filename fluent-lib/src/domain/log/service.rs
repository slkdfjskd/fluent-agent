use crate::common::error::Result;
use crate::domain::log::entity::RequestLog;
use crate::domain::log::repo::LogRepo;
use crate::domain::log::{entity, LogRepoWrap, LOG_REPO_WRAP};
use std::thread;

pub fn create_request_log(log: &entity::RequestLog) -> Result {
    thread::spawn(move || {
        let result = LOG_REPO_WRAP.get().deallocate_request_log(5000);
        if result.is_err() {
            error!("Failed to deallocate request log: {:?}", result);
        }
    });
    LOG_REPO_WRAP.get().create_request_log(&log)
}

pub fn list_next_request_log(
    last_id: i64,
    keyword: &str,
    page_size: u16,
) -> Result<Vec<entity::RequestLog>> {
    LOG_REPO_WRAP
        .get()
        .list_next_request_log(last_id, keyword, page_size)
}

pub fn list_pre_request_log(
    first_id: i64,
    keyword: &str,
    page_size: u16,
) -> Result<Vec<entity::RequestLog>> {
    LOG_REPO_WRAP
        .get()
        .list_pre_request_log(first_id, keyword, page_size)
}

pub fn get_latest_request_log(request_id: i64) -> Result<Option<RequestLog>> {
    LOG_REPO_WRAP.get().get_latest_request_log(request_id)
}
