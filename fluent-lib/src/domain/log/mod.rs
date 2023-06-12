use crate::infra::persistence;

pub mod entity;
pub mod repo;
pub mod service;

lazy_static! {
    pub static ref LOG_REPO_WRAP: LogRepoWrap<persistence::log::repo::LogRepo> = {
        return LogRepoWrap {
            log_repo: persistence::log::repo::LogRepo::new(),
        };
    };
}

pub struct LogRepoWrap<T>
where
    T: repo::LogRepo,
{
    log_repo: T,
}

impl<T: repo::LogRepo> LogRepoWrap<T> {
    pub fn get(&self) -> &T {
        return &self.log_repo;
    }
}
