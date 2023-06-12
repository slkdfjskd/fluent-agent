use crate::common::error::Result;
use crate::domain;
use crate::domain::config::entity;
use crate::domain::config::entity::{Config, ConfigRef, Environment, EnvironmentRef};
use crate::infra::persistence::config::{assembler, db};
use crate::infra::persistence::CONN;

pub struct ConfigRepo {}

impl ConfigRepo {
    pub fn new() -> Self {
        return ConfigRepo {};
    }
}

impl domain::config::repo::ConfigRepo for ConfigRepo {
    fn create_env(&self, env: &EnvironmentRef) -> Result {
        db::create_env(&CONN, assembler::to_env_po(env)?)
    }

    fn update_env_variable(&self, env: &EnvironmentRef) -> Result {
        let env = assembler::to_env_po(env)?;
        db::update_env_variable(&CONN, env)
    }

    fn update_env(&self, new_name: &str, old_name: &str) -> Result {
        db::update_env(&CONN, new_name, old_name)
    }

    fn list_env(&self) -> Result<Vec<Environment>> {
        let env_list = db::find_env(&CONN)?;
        return assembler::batch_to_env_entity(env_list);
    }

    fn get_env(&self, env_name: &str, variable: &str) -> Result<Option<Environment>> {
        let env = db::find_env_by_env_variable(&CONN, env_name, variable)?;
        let env = match env {
            Some(e) => Some(assembler::to_env_entity(e)?),
            None => None,
        };
        return Ok(env);
    }

    fn delete_env_variable(&self, id: i64) -> Result {
        db::delete_env_variable(&CONN, id)
    }

    fn delete_env(&self, env_name: &str) -> Result {
        db::delete_env(&CONN, env_name)
    }

    fn get_config(&self, key: &str) -> Result<Option<Config>> {
        let config = db::find_config(&CONN, key)?;
        return Ok(assembler::to_config_entity(config));
    }

    fn get_batch_config(&self, keys: &Vec<String>) -> Result<Vec<Config>> {
        let configs = db::find_configs(&CONN, keys)?;
        return assembler::batch_to_config_entity(configs);
    }

    fn put_config(&self, config: &ConfigRef) -> Result {
        db::create_config(&CONN, &assembler::to_config_po(config))?;
        return Ok(());
    }

    fn delete_configs(&self, keys: &Vec<String>) -> Result {
        db::delete_configs(&CONN, keys)?;
        return Ok(());
    }
}
