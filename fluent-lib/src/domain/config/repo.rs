use crate::common::error::Result;
use crate::domain::config::entity;
use crate::domain::config::entity::{Environment, EnvironmentRef};
use crate::infra::persistence::config::db;
use crate::infra::persistence::CONN;

pub trait ConfigRepo {
    fn create_env(&self, env: &EnvironmentRef) -> Result;

    fn update_env_variable(&self, env: &EnvironmentRef) -> Result;

    fn update_env(&self, new_name: &str, old_name: &str) -> Result;

    fn list_env(&self) -> Result<Vec<Environment>>;

    fn get_env(&self, env_name: &str, variable: &str) -> Result<Option<Environment>>;

    fn delete_env_variable(&self, id: i64) -> Result;

    fn delete_env(&self, env_name: &str) -> Result;

    fn get_config(&self, key: &str) -> Result<Option<entity::Config>>;

    fn get_batch_config(&self, keys: &Vec<String>) -> Result<Vec<entity::Config>>;

    fn put_config(&self, config: &entity::ConfigRef) -> Result;
    fn delete_configs(&self, keys: &Vec<String>) -> Result;
}
