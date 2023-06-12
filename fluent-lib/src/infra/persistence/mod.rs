use chrono::{DateTime, Local};
use std::sync::Mutex;

use rusqlite::Connection;

use crate::common::error::Result;
use crate::infra::persistence::proto::file;

pub mod config;
pub mod log;
pub mod proto;

lazy_static! {
    pub static ref CONN: Mutex<Connection> = {
        let db_name = "data".to_string();
        let result = init_conn(&db_name);
        return match result {
            Ok(c) => c,
            Err(e) => panic!("init sqlite connection err:{}", e),
        };
    };
}

fn init_conn(name: &String) -> Result<Mutex<Connection>> {
    let db_name = name.to_owned() + ".db";
    let data_dir = file::data_dir()?;
    let db_file = data_dir.join(db_name);
    debug!("init_conn db_file:{:?}", db_file);

    let conn = Connection::open(db_file.as_path())?;

    return Ok(Mutex::new(conn));
}

pub fn init_db() -> Result {
    // select * from sqlite_master where name = "";
    // .tables
    // .indices nav_project
    let conn = &CONN.lock().unwrap();
    let row = conn.execute(
        "
        CREATE TABLE IF NOT EXISTS config (
            id          BIGINT PRIMARY KEY ,
            key         VARCHAR(128) NOT NULL,
            value       TEXT NOT NULL,
            created_at  DATETIME NOT NULL,
            updated_at  DATETIME NOT NULL
        );",
        [], // empty list of parameters.
    )?;

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS uk_key on config (`key`);",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS nav_project  (
            id              BIGINT PRIMARY KEY ,
            project_name    VARCHAR(128) NOT NULL,
            project_id      BIGINT NOT NULL,
            req_type        VARCHAR(32) NOT NULL,
            services        TEXT NOT NULL,
            order_no        INTEGER NOT NULL,
            created_at      DATETIME NOT NULL,
            updated_at      DATETIME NOT NULL
        );
    ",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_project_id on nav_project (project_id);",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS project (
            id              BIGINT PRIMARY KEY ,
            name            VARCHAR(128) NOT NULL,
            req_type        VARCHAR(32) NOT NULL,
            proto_file      TEXT NOT NULL,
            created_at      DATETIME NOT NULL,
            updated_at      DATETIME NOT NULL
        );
    ",
        [],
    )?;

    conn.execute(
        "
         CREATE TABLE IF NOT EXISTS request (
            id              BIGINT PRIMARY KEY ,
            name            VARCHAR(128) NOT NULL,
            project_id      BIGINT NOT NULL,
            url             VARCHAR(256) NOT NULL,
            req_type        VARCHAR(32) NOT NULL,
            service         VARCHAR(128) NOT NULL,
            method          VARCHAR(128) NOT NULL,
            headers         TEXT NOT NULL,
            params          TEXT NOT NULL,
            req_json        TEXT NOT NULL,
            resp_json       TEXT NOT NULL,
            created_at      DATETIME NOT NULL,
            updated_at      DATETIME NOT NULL
        );
    ",
        [],
    )?;

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS uk_project_id_service_method on request (`project_id`, `service`, `method`);",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS unique_id (
            id             BIGINT PRIMARY KEY ,
            unique_id      BIGINT NOT NULL
        );
    ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS environment (
            id              BIGINT PRIMARY KEY , 
            env_name        VARCHAR(128) NOT NULL,
            name            VARCHAR(128) NOT NULL,
            value           TEXT NOT NULL,
            created_at      DATETIME NOT NULL,
            updated_at      DATETIME NOT NULL
        );
    ",
        [],
    )?;

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS uk_env_name on environment (`env_name`, `name`);",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS request_log (
            id              BIGINT PRIMARY KEY , 
            request_id      BIGINT NOT NULL,
            base_url        VARCHAR(128) NOT NULL,
            path            VARCHAR(256) NOT NULL, 
            error           TEXT NOT NULL,
            info            TEXT NOT NULL,
            request         TEXT NOT NULL,
            response        TEXT NOT NULL,
            created_at      DATETIME NOT NULL,
            updated_at      DATETIME NOT NULL
        );
    ",
        [],
    )?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    // use crate::infra::persistence::init;

    #[test]
    fn test_init() {
        // init(db_file.as_path())
    }
}
