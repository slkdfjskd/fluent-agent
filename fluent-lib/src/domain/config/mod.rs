pub mod entity;
pub mod param;
pub mod repo;
pub mod service;

use crate::infra::persistence;

lazy_static! {
    pub static ref CONFIG_REPO_WRAP: ConfigRepoWrap<persistence::config::repo::ConfigRepo> = {
        return ConfigRepoWrap {
            config_repo: persistence::config::repo::ConfigRepo::new(),
        };
    };
}

pub struct ConfigRepoWrap<T>
where
    T: repo::ConfigRepo,
{
    config_repo: T,
}

impl<T: repo::ConfigRepo> ConfigRepoWrap<T> {
    pub fn get(&self) -> &T {
        return &self.config_repo;
    }
}
