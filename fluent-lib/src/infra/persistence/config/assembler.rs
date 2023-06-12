use crate::common::error::Result;
use crate::domain::config::entity;
use crate::infra::persistence::config::po;
use chrono::Local;

pub fn to_config_po<'a>(config: &'a entity::ConfigRef) -> po::ConfigRef<'a> {
    return po::ConfigRef {
        id: config.id,
        key: config.key,
        value: config.value,
        created_at: Local::now(),
        updated_at: Local::now(),
    };
}

pub fn to_config_entity(config: Option<po::Config>) -> Option<entity::Config> {
    if let Some(c) = config {
        return Some(entity::Config {
            id: c.id,
            key: c.key,
            value: c.value,
        });
    }
    return None;
}

pub fn batch_to_config_entity(configs: Vec<po::Config>) -> Result<Vec<entity::Config>> {
    let mut result = vec![];
    for config in configs {
        result.push(entity::Config {
            id: config.id,
            key: config.key,
            value: config.value,
        })
    }
    return Ok(result);
}

pub fn to_env_po<'a>(env: &'a entity::EnvironmentRef) -> Result<po::EnvironmentRef<'a>> {
    Ok(po::EnvironmentRef {
        id: env.id,
        env_name: env.env_name,
        name: env.name,
        value: env.value,
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

pub fn batch_to_env_po<'a>(
    env_list: &'a Vec<entity::EnvironmentRef>,
) -> Result<Vec<po::EnvironmentRef<'a>>> {
    let mut result = vec![];
    for env in env_list {
        result.push(to_env_po(env)?)
    }
    return Ok(result);
}

pub fn to_env_entity(env: po::Environment) -> Result<entity::Environment> {
    Ok(entity::Environment {
        id: env.id,
        env_name: env.env_name,
        name: env.name,
        value: env.value,
    })
}

pub fn batch_to_env_entity(env_list: Vec<po::Environment>) -> Result<Vec<entity::Environment>> {
    let mut result = vec![];
    for env in env_list {
        let new_env = to_env_entity(env)?;
        result.push(new_env);
    }

    Ok(result)
}
