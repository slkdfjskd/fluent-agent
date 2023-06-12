use crate::common;
use crate::common::error::Result;
use crate::domain::config::entity::{Config, ConfigRef, Environment};
use crate::domain::config::repo::ConfigRepo;
use crate::domain::config::{entity, CONFIG_REPO_WRAP};
use crate::domain::proto;
use crate::domain::proto::PROTO_REPO_WRAP;
use regex::Regex;
use std::collections::HashMap;

pub fn get_config(key: &str) -> Result<Option<Config>> {
    CONFIG_REPO_WRAP.get().get_config(key)
}

pub fn get_batch_config(keys: &Vec<String>) -> Result<Vec<Config>> {
    CONFIG_REPO_WRAP.get().get_batch_config(keys)
}

pub fn put_config(key: &str, value: &str) -> Result {
    CONFIG_REPO_WRAP.get().put_config(&ConfigRef {
        id: common::uniqueid::generate()?,
        key,
        value,
    })
}

pub fn delete_configs(keys: &Vec<String>) -> Result {
    CONFIG_REPO_WRAP.get().delete_configs(keys)
}

pub fn create_env(env: &mut entity::EnvironmentRef) -> Result<Environment> {
    env.id = common::uniqueid::generate()?;
    CONFIG_REPO_WRAP.get().create_env(env)?;
    Ok(Environment {
        id: env.id,
        env_name: env.env_name.to_string(),
        name: env.name.to_string(),
        value: env.value.to_string(),
    })
}

pub fn update_env_variable(env: entity::EnvironmentRef) -> Result {
    CONFIG_REPO_WRAP.get().update_env_variable(&env)
}

pub fn update_env(new_name: &str, old_name: &str) -> Result {
    CONFIG_REPO_WRAP.get().update_env(new_name, old_name)
}

pub fn delete_env_variable(id: i64) -> Result {
    CONFIG_REPO_WRAP.get().delete_env_variable(id)
}

pub fn delete_env(env_name: &str) -> Result {
    CONFIG_REPO_WRAP.get().delete_env(env_name)
}

pub fn list_env() -> Result<Vec<entity::Environment>> {
    CONFIG_REPO_WRAP.get().list_env()
}

pub fn parse_url_with_env(url: String, env_name: &str) -> Result<String> {
    let reg = Regex::new(r"\{\{[\w-]*\}\}").unwrap();
    let match_vars: Vec<&str> = reg.find_iter(&url).map(|v| v.as_str()).collect();
    let reg = Regex::new(r"[\w-]*").unwrap();
    let mut var_map = HashMap::new();
    for match_var in match_vars {
        let variables: Vec<&str> = reg.find_iter(match_var).map(|v| v.as_str()).collect();
        for var in variables {
            if var.is_empty() {
                continue;
            }
            let env_opt = CONFIG_REPO_WRAP.get().get_env(env_name, var)?;
            if let Some(env) = env_opt {
                var_map.insert(match_var, env);
            }
        }
    }
    let mut result: String = url.clone();
    for (k, v) in var_map {
        result = url.replace(k, &v.value);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::domain::config::service::{get_config, put_config};
    use crate::infra::persistence;
    use regex::Regex;

    #[test]
    fn test_regex() {
        let url = "{{abc}} dkfjdk{{fjd}}#a33";
        let r = Regex::new(r"\{\{[\w-]*\}\}").unwrap();
        let res: Vec<&str> = r.find_iter(url).map(|mat| mat.as_str()).collect();
        println!("res: {:?}", res);

        let variable = "{{abc123}}dfsdf{{abc123}}";
        let r = Regex::new(r"[\w-]*").unwrap();
        let res: Vec<&str> = r.find_iter(variable).map(|mat| mat.as_str()).collect();
        println!("res: {:?}", res);

        let variable = "base-url_sfsf123";
        let r = Regex::new(r"^[\w-]{1,50}$").unwrap();
        let res = r.is_match(variable);
        println!("res: {:?}", res);
    }

    #[test]
    fn test_put_config() {
        persistence::init_db().unwrap();
        let key = "test-test-test";
        let value = "{\"123\": \"123\"}";
        put_config(key, value).unwrap();
    }

    #[test]
    fn test_get_config() {
        persistence::init_db().unwrap();
        let key = "test-test-test";
        let value = get_config(key).unwrap();
        println!("value: {:?}", value.unwrap());
    }
}
