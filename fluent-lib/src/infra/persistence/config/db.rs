use crate::common::error::Result;
use crate::common::time;
use crate::infra::persistence::config::po;
use crate::infra::persistence::config::po::UniqueId;
use crate::infra::persistence::config::repo::ConfigRepo;
use crate::infra::persistence::proto;
use anyhow::ensure;
use chrono::Local;
use rusqlite::{Connection, Transaction};
use std::sync::Mutex;

pub fn create_env(conn: &Mutex<Connection>, env: po::EnvironmentRef) -> Result {
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO environment (id, env_name, name, value, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        [
            &env.id.to_string(),
            env.env_name,
            env.name,
            env.value,
            &time::to_string_rfc3339(&env.created_at),
            &time::to_string_rfc3339(&env.updated_at),
        ],
    )?;
    return Ok(());
}

pub fn update_env_variable(conn: &Mutex<Connection>, env: po::EnvironmentRef) -> Result {
    let mut conn = conn.lock().unwrap();
    conn.execute(
        "UPDATE environment SET name = ?1, value = ?2, updated_at = ?3 where id = ?4",
        [
            env.name,
            env.value,
            &time::to_string_rfc3339(&env.updated_at),
            &env.id.to_string(),
        ],
    )?;
    return Ok(());
}

pub fn update_env(conn: &Mutex<Connection>, new_name: &str, old_name: &str) -> Result {
    let mut conn = conn.lock().unwrap();
    let updated_at = Local::now();
    conn.execute(
        "UPDATE environment SET env_name = ?1, updated_at = ?2 where env_name = ?3",
        [new_name, &time::to_string_rfc3339(&updated_at), old_name],
    )?;
    return Ok(());
}

pub fn delete_env_variable(conn: &Mutex<Connection>, id: i64) -> Result {
    let conn = conn.lock().unwrap();
    conn.execute("DELETE FROM environment WHERE id = ?1", [id.to_string()])?;
    return Ok(());
}

pub fn delete_env(conn: &Mutex<Connection>, env_name: &str) -> Result {
    let conn = conn.lock().unwrap();
    conn.execute("DELETE FROM environment WHERE env_name = ?1", [env_name])?;
    return Ok(());
}

pub fn find_env_by_env_variable(
    conn: &Mutex<Connection>,
    env: &str,
    variable: &str,
) -> Result<Option<po::Environment>> {
    let conn = conn.lock().unwrap();
    let sql =
        "SELECT id, env_name, name, value, created_at, updated_at FROM environment where env_name = ?1 and name = ?2";
    let mut stmt = conn.prepare(sql)?;
    let env_list = stmt.query_map([env, variable], |row| {
        let env = po::Environment {
            id: row.get(0)?,
            env_name: row.get(1)?,
            name: row.get(2)?,
            value: row.get(3)?,
            created_at: time::format_date(row.get(4)?),
            updated_at: time::format_date(row.get(5)?),
        };
        Ok(env)
    })?;
    for env in env_list {
        return Ok(Some(env?));
    }
    return return Ok(None);
}

pub fn find_env(conn: &Mutex<Connection>) -> Result<Vec<po::Environment>> {
    let conn = conn.lock().unwrap();
    let sql =
        "SELECT id, env_name, name, value, created_at, updated_at FROM environment ORDER BY id ASC";
    let mut stmt = conn.prepare(sql)?;
    let env_list = stmt.query_map([], |row| {
        let env = po::Environment {
            id: row.get(0)?,
            env_name: row.get(1)?,
            name: row.get(2)?,
            value: row.get(3)?,
            created_at: time::format_date(row.get(4)?),
            updated_at: time::format_date(row.get(5)?),
        };
        Ok(env)
    })?;
    let mut result = vec![];
    for env in env_list {
        result.push(env?)
    }
    return Ok(result);
}

pub fn create_config(conn: &Mutex<Connection>, config: &po::ConfigRef) -> Result {
    let conn = conn.lock().unwrap();
    let find_opt = find_config0(&conn, config.key)?;
    if let Some(_c) = find_opt {
        conn.execute(
            "UPDATE config SET value = ?1, updated_at = ?2 where `key` = ?3",
            [
                config.value,
                &time::to_string_rfc3339(&config.updated_at),
                config.key,
            ],
        )?;
    } else {
        conn.execute("INSERT INTO config (id, `key`, value, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                     [&config.id.to_string(), config.key, config.value, &time::to_string_rfc3339(&config.created_at), &time::to_string_rfc3339(&config.updated_at)])?;
    }
    return Ok(());
}

pub fn delete_configs(conn: &Mutex<Connection>, keys: &Vec<String>) -> Result {
    if keys.is_empty() {
        return Ok(());
    }
    let param = keys.join(",");
    let conn = conn.lock().unwrap();
    let sql = "DELETE FROM config WHERE `key` in (?1)";
    conn.execute(sql, [param])?;
    return Ok(());
}

pub fn find_config<'a>(conn: &'a Mutex<Connection>, key: &'a str) -> Result<Option<po::Config>> {
    let conn = conn.lock().unwrap();
    return find_config0(&conn, key);
}

pub fn find_configs<'a>(
    conn: &'a Mutex<Connection>,
    keys: &'a Vec<String>,
) -> Result<Vec<po::Config>> {
    let conn = conn.lock().unwrap();
    return find_configs0(&conn, keys);
}

fn find_config0(conn: &Connection, key: &str) -> Result<Option<po::Config>> {
    let mut result = find_configs0(conn, &vec![key.to_string()])?;
    if result.is_empty() {
        return Ok(None);
    }
    return Ok(Some(result.remove(0)));
}

fn find_configs0(conn: &Connection, keys: &Vec<String>) -> Result<Vec<po::Config>> {
    if keys.len() == 0 {
        return Ok(vec![]);
    }
    let mut params = vec![];
    for key in keys {
        params.push(format!("'{}'", key))
    }
    let params = params.join(",");
    let sql = format!(
        "SELECT id, `key`, value, created_at, updated_at FROM config where `key` in ({})",
        params
    );
    let mut stmt = conn.prepare(&sql)?;
    let configs = stmt.query_map([], |row| {
        Ok(po::Config {
            id: row.get(0)?,
            key: row.get(1)?,
            value: row.get(2)?,
            created_at: time::format_date(row.get(3)?),
            updated_at: time::format_date(row.get(4)?),
        })
    })?;
    let mut result = vec![];
    for config in configs {
        result.push(config?);
    }
    return Ok(result);
}

pub fn get_unique_id(conn: &Mutex<Connection>, count: i64) -> Result<i64> {
    let id = 1;
    let init_unique_id = 10000;
    let conn = conn.lock().unwrap();
    let sql = "SELECT id, unique_id FROM unique_id where id = ?1";
    let mut stmt = conn.prepare(&sql)?;
    let requests = stmt.query_map([id], |row| {
        Ok(UniqueId {
            id: row.get(0)?,
            unique_id: row.get(1)?,
        })
    })?;
    let mut max_unique_id = 0;
    for unique_id in requests {
        max_unique_id = unique_id?.unique_id;
    }
    if max_unique_id == 0 {
        conn.execute(
            "INSERT INTO unique_id (id, unique_id) VALUES (1, ?1)",
            [init_unique_id],
        )?;
        max_unique_id = init_unique_id;
    }
    conn.execute(
        "UPDATE unique_id SET unique_id = ?1 where id = ?2",
        [max_unique_id + count, id],
    )?;
    return Ok(max_unique_id);
}
