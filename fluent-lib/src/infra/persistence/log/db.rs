// CREATE TABLE IF NOT EXISTS request_log (
// id              BIGINT PRIMARY KEY ,
// request_id      BIGINT NOT NULL,
// base_url        VARCHAR(128) NOT NULL
// path            VARCHAR(256) NOT NULL
// error           TEXT NOT NULL,
// info            TEXT NOT NULL,
// request         TEXT NOT NULL,
// response        TEXT NOT NULL,
// created_at      DATETIME NOT NULL,
// updated_at      DATETIME NOT NULL
// );

use crate::common;
use crate::common::error::Result;
use crate::common::time;
use crate::infra::persistence::log::po;
use crate::infra::persistence::log::po::RequestLog;
use rusqlite::{Connection, Error, Row};
use std::sync::Mutex;

pub fn create_request_log(conn: &Mutex<Connection>, log: &po::RequestLogRef) -> Result {
    let conn = conn.lock().unwrap();
    let params: [&str; 10] = [
        &log.id.to_string(),
        &log.request_id.to_string(),
        log.base_url,
        log.path,
        &log.error,
        &log.info,
        &log.request,
        &log.response,
        &time::to_string_rfc3339(&log.created_at),
        &time::to_string_rfc3339(&log.updated_at),
    ];
    conn.execute(
        "
        INSERT INTO request_log (id, request_id, base_url, path, error, info, request, response, created_at, updated_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        ",
        params,
    )?;

    Ok(())
}

pub fn find_request_log_by_request_id(
    conn: &Mutex<Connection>,
    request_id: i64,
    page_size: u16,
) -> Result<Vec<RequestLog>> {
    let conn = conn.lock().unwrap();
    let mut sql = "SELECT id, request_id, base_url, path, error, info, request, response, created_at, updated_at FROM request_log where request_id = ?1 order by id desc limit ?2";
    let mut stmt = conn.prepare(sql)?;
    let mut rows = stmt.query_map([&request_id.to_string(), &page_size.to_string()], |row| {
        parse_request_log(row)
    })?;
    let mut result = vec![];
    for r in rows {
        result.push(r?)
    }

    Ok(result)
}

pub fn find_request_logs(
    conn: &Mutex<Connection>,
    id: i64,
    keyword: &str,
    mut page_size: u16,
    is_next: bool,
) -> Result<Vec<RequestLog>> {
    if page_size == 0 || page_size > 1000 {
        page_size = 50;
    }
    let conn = conn.lock().unwrap();
    let mut sql = "SELECT id, request_id, base_url, path, error, info, request, response, created_at, updated_at FROM request_log ".to_string();

    let mut wheres = vec![];
    if is_next {
        if id > 0 || id == -1 {
            wheres.push(format!(" id < {} ", id))
        }
    } else {
        wheres.push(format!(" id > {} ", id))
    }

    if !keyword.is_empty() {
        wheres.push(format!(" (base_url request_log '%{}%' or path like '%{}%' or request like '%{}%' or response like '%{}%' or error like '%{}%') ", keyword, keyword, keyword, keyword, keyword));
    }

    let w = wheres.join(" AND ");
    if !wheres.is_empty() {
        sql.push_str(" WHERE ")
    }
    sql.push_str(&w);

    sql.push_str(&format!(" ORDER BY id DESC LIMIT {}", page_size));

    let mut stmt = conn.prepare(&sql)?;
    let logs = stmt.query_map([], |row| parse_request_log(row))?;

    let mut result = vec![];
    for r in logs {
        result.push(r?)
    }

    Ok(result)
}

fn parse_request_log(row: &Row) -> rusqlite::Result<RequestLog> {
    Ok(po::RequestLog {
        id: row.get(0)?,
        request_id: row.get(1)?,
        base_url: row.get(2)?,
        path: row.get(3)?,
        error: row.get(4)?,
        info: row.get(5)?,
        request: row.get(6)?,
        response: row.get(7)?,
        created_at: time::format_date(row.get(8)?),
        updated_at: time::format_date(row.get(9)?),
    })
}

pub fn deallocate_request_log(conn: &Mutex<Connection>, max_retain_count: i32) -> Result {
    let conn = conn.lock().unwrap();
    let sql = "SELECT count(*) FROM request_log";
    let count = conn.execute(sql, [])?;
    let redundant = count - (max_retain_count as usize);
    if redundant <= 0 {
        return Ok(());
    }
    let sql = "DELETE FROM request_log order by id desc limit ?1";
    conn.execute(sql, [&redundant])?;

    Ok(())
}
