use std::fmt::format;
use std::sync::Mutex;

use chrono::{DateTime, Local};
use rusqlite::{Connection, Transaction};

use crate::common::error::Result;
use crate::common::time;
use crate::common::types::ReqType;
use crate::infra::persistence::config::po::UniqueId;
use crate::infra::persistence::proto::po;

pub fn update_project_name(conn: &Mutex<Connection>, project_id: i64, new_name: &str) -> Result {
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    let id = &project_id.to_string();
    tx.execute("UPDATE project SET name = ?1 where id = ?2", [new_name, id])?;
    tx.execute(
        "UPDATE nav_project SET project_name = ?1 where project_id = ?2",
        [new_name, id],
    )?;
    tx.commit()?;
    return Ok(());
}

pub fn delete_project(conn: &Mutex<Connection>, project_id: i64) -> Result {
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    delete_project0(&tx, project_id)?;
    delete_nav_project(&tx, project_id)?;
    delete_request_by_project_id(&tx, project_id)?;
    tx.commit()?;
    return Ok(());
}

fn delete_project0(tx: &Transaction, project_id: i64) -> Result {
    tx.execute("DELETE FROM project WHERE id = ?1", [project_id])?;
    return Ok(());
}

fn delete_request_by_project_id(tx: &Transaction, project_id: i64) -> Result {
    tx.execute("DELETE FROM request WHERE project_id = ?1", [project_id])?;
    return Ok(());
}

fn delete_nav_project(tx: &Transaction, project_id: i64) -> Result {
    tx.execute(
        "DELETE FROM nav_project WHERE project_id = ?1",
        [project_id],
    )?;
    return Ok(());
}

pub fn find_nav_project(conn: &Mutex<Connection>) -> Result<Vec<po::NavProject>> {
    let conn = conn.lock().unwrap();
    let sql = "SELECT * FROM nav_project ORDER BY id DESC";
    let mut stmt = conn.prepare(sql)?;
    let nav_projects = stmt.query_map([], |row| parse_nav_project(row))?;
    let mut result = vec![];
    for p in nav_projects {
        result.push(p?)
    }
    return Ok(result);
}

pub fn find_nav_project_by_project_id(
    conn: &Mutex<Connection>,
    project_id: i64,
) -> Result<Option<po::NavProject>> {
    let conn = conn.lock().unwrap();
    let sql = "SELECT *  FROM nav_project WHERE project_id = ?1 LIMIT 1";
    let mut stmt = conn.prepare(&sql)?;
    let nav_projects = stmt.query_map([project_id], |row| parse_nav_project(row))?;
    for r in nav_projects {
        let p = r?;
        return Ok(Some(p));
    }
    return Ok(None);
}

pub fn find_nav_project_by_id(conn: &Mutex<Connection>, id: i64) -> Result<Option<po::NavProject>> {
    let conn = conn.lock().unwrap();
    let sql = "SELECT * FROM nav_project WHERE id = ?1 LIMIT 1";
    let mut stmt = conn.prepare(&sql)?;
    let nav_projects = stmt.query_map([id], |row| parse_nav_project(row))?;
    for r in nav_projects {
        let p = r?;
        return Ok(Some(p));
    }
    return Ok(None);
}

pub fn create_project_with_tx(
    conn: &Mutex<Connection>,
    project: &po::ProjectRef,
    nav_project: &po::NavProjectRef,
) -> Result {
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    create_project(&tx, project)?;
    create_nav_project(&tx, nav_project)?;
    tx.commit()?;
    return Ok(());
}

pub fn find_project_by_id(
    conn: &Mutex<Connection>,
    project_id: i64,
) -> Result<Option<po::Project>> {
    let conn = conn.lock().unwrap();
    let sql = format!("SELECT id, name,  req_type, proto_file, created_at, updated_at FROM project WHERE id = {} LIMIT 1", project_id);
    let mut stmt = conn.prepare(&sql)?;
    let projects = stmt.query_map([], |row| {
        let req_type: String = row.get(2)?;
        let req_type: ReqType = req_type.into();
        Ok(po::Project {
            id: row.get(0)?,
            name: row.get(1)?,
            req_type,
            proto_file: row.get(3)?,
            created_at: time::format_date(row.get(4)?),
            updated_at: time::format_date(row.get(5)?),
        })
    })?;

    for project in projects {
        let p = project?;
        return Ok(Some(p));
    }
    return Ok(None);
}

pub fn find_requests_by_project_id(
    conn: &Mutex<Connection>,
    project_id: i64,
) -> Result<Vec<po::Request>> {
    let conn = conn.lock().unwrap();
    let sql = format!("SELECT id, name, project_id, url, req_type, service, method, headers, params, req_json, resp_json, created_at, updated_at FROM request WHERE project_id = {} ", project_id);
    let mut stmt = conn.prepare(&sql)?;
    let requests = stmt.query_map([], |row| parse_request(row))?;
    let mut result = vec![];
    for r in requests {
        result.push(r?)
    }
    Ok(result)
}

pub fn find_request_by_id(
    conn: &Mutex<Connection>,
    request_id: i64,
) -> Result<Option<po::Request>> {
    let conn = conn.lock().unwrap();
    let sql = format!("SELECT id, name, project_id, url, req_type, service, method, headers, params, req_json, resp_json, created_at, updated_at FROM request WHERE id = {} LIMIT 1", request_id);
    let mut stmt = conn.prepare(&sql)?;
    let requests = stmt.query_map([], |row| parse_request(row))?;
    for req in requests {
        let r = req?;
        return Ok(Some(r));
    }
    return Ok(None);
}

fn parse_request(row: &rusqlite::Row) -> rusqlite::Result<po::Request> {
    let req_type: String = row.get(4)?;
    let req_type: ReqType = req_type.into();
    Ok(po::Request {
        id: row.get(0)?,
        name: row.get(1)?,
        project_id: row.get(2)?,
        url: row.get(3)?,
        req_type,
        service: row.get(5)?,
        method: row.get(6)?,
        headers: row.get(7)?,
        params: row.get(8)?,
        req_json: row.get(9)?,
        resp_json: row.get(10)?,
        created_at: time::format_date(row.get(11)?),
        updated_at: time::format_date(row.get(12)?),
    })
}

fn parse_nav_project(row: &rusqlite::Row) -> rusqlite::Result<po::NavProject> {
    let req_type: String = row.get(3)?;
    let req_type: ReqType = req_type.into();
    Ok(po::NavProject {
        id: row.get(0)?,
        project_name: row.get(1)?,
        project_id: row.get(2)?,
        req_type,
        services: row.get(4)?,
        order_no: row.get(5)?,
        created_at: time::format_date(row.get(6)?),
        updated_at: time::format_date(row.get(7)?),
    })
}

pub fn batch_create_request_with_tx(
    conn: &Mutex<Connection>,
    nav_project: &po::NavProjectRef,
    project: &po::Project,
    insert_requests: &Vec<po::RequestRef>,
    update_requests: &Vec<po::RequestRef>,
    delete_req_ids: &Vec<i64>,
) -> Result {
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    update_nav_project(&tx, nav_project)?;
    update_project(&tx, project)?;
    batch_create_request0(&tx, insert_requests)?;
    batch_update_request0(&tx, update_requests)?;
    batch_delete_request(&tx, delete_req_ids)?;
    tx.commit()?;
    return Ok(());
}

fn batch_update_request0(tx: &Transaction, requests: &Vec<po::RequestRef>) -> Result {
    for req in requests {
        update_request0(&tx, req)?;
    }
    return Ok(());
}

fn batch_delete_request(tx: &Transaction, ids: &Vec<i64>) -> Result {
    if ids.len() <= 0 {
        return Ok(());
    }
    let ids = ids.iter().map(|v| v.to_string()).collect::<Vec<String>>();
    let sql = format!("DELETE FROM request WHERE id in ({})", ids.join(","));
    tx.execute(&sql, [])?;
    return Ok(());
}

fn create_project(tx: &Transaction, project: &po::ProjectRef) -> Result {
    let req_type: String = project.req_type.into();
    let params: [&str; 6] = [
        &project.id.to_string(),
        &project.name,
        &req_type,
        &project.proto_file,
        &project.created_at.to_rfc3339(),
        &project.updated_at.to_rfc3339(),
    ];
    tx.execute(
        "
        INSERT INTO project (id, name, req_type, proto_file, created_at, updated_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ",
        params,
    )?;
    return Ok(());
}

fn create_nav_project(tx: &Transaction, nav_project: &po::NavProjectRef) -> Result {
    let req_type: String = nav_project.req_type.into();
    // let req_type: String = req_type.into();
    let params: [&str; 8] = [
        &nav_project.id.to_string(),
        nav_project.project_name,
        &nav_project.project_id.to_string(),
        &req_type,
        &nav_project.services,
        &nav_project.order_no.to_string(),
        &time::to_string_rfc3339(&nav_project.created_at),
        &time::to_string_rfc3339(&nav_project.updated_at),
    ];
    tx.execute("
    INSERT INTO nav_project (id, project_name, project_id, req_type, services, order_no, created_at, updated_at) VALUES
    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
   ", params)?;
    return Ok(());
}

fn update_project(tx: &Transaction, project: &po::Project) -> Result {
    tx.execute(
        "UPDATE project SET name = ?1, proto_file = ?2, updated_at = ?3 where id = ?4",
        [
            &project.name,
            &project.proto_file,
            &time::to_string_rfc3339(&project.updated_at),
            &project.id.to_string(),
        ],
    )?;
    return Ok(());
}

fn update_nav_project(tx: &Transaction, nav_project: &po::NavProjectRef) -> Result {
    let params = [
        nav_project.project_name,
        &nav_project.services,
        &nav_project.order_no.to_string(),
        &time::to_string_rfc3339(&nav_project.updated_at),
        &nav_project.id.to_string(),
    ];
    tx.execute("UPDATE nav_project SET project_name = ?1, services = ?2, order_no = ?3 , updated_at = ?4 where id = ?5",
               params)?;
    return Ok(());
}

fn batch_create_request0(tx: &Transaction, requests: &Vec<po::RequestRef>) -> Result {
    if requests.len() <= 0 {
        return Ok(());
    }
    let sql = "INSERT INTO request (id, name, project_id, url, req_type, service, method, headers, params, req_json, resp_json, created_at, updated_at)
                VALUES ";
    let mut values_list = Vec::new();
    for req in requests {
        let mut values = Vec::new();
        let req_type: String = req.req_type.into();
        values.push(format!("'{}'", req.id));
        values.push(format!("'{}'", req.name));
        values.push(format!("'{}'", req.project_id));
        values.push(format!("'{}'", req.url));
        values.push(format!("'{}'", req_type));
        values.push(format!("'{}'", req.service));
        values.push(format!("'{}'", req.method));
        values.push(format!("'{}'", req.headers));
        values.push(format!("'{}'", req.params));
        values.push(format!("'{}'", req.req_json));
        values.push(format!("'{}'", req.resp_json));
        values.push(format!("'{:?}'", &time::to_string_rfc3339(&req.created_at)));
        values.push(format!("'{:?}'", &time::to_string_rfc3339(&req.created_at)));
        let s = values.join(",");
        values_list.push("(".to_owned() + &s + ")");
    }
    let s = values_list.join(",");
    let sql = sql.to_owned() + &s;
    tx.execute_batch(&sql)?;
    return Ok(());
}

fn update_request0<'a>(tx: &Transaction, request: &po::RequestRef<'a>) -> Result {
    tx.execute("UPDATE request SET name = ?1, url = ?2, method = ?3, headers = ?4, params = ?5, req_json = ?6, resp_json = ?7, updated_at = ?8 where id = ?9",
                 [request.name, request.url, request.method, &request.headers, &request.params, &request.req_json, &request.resp_json,
                     &time::to_string_rfc3339(&request.updated_at), &request.id.to_string()])?;
    return Ok(());
}

pub fn update_request<'a>(conn: &Mutex<Connection>, request: &po::RequestRef<'a>) -> Result {
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    update_request0(&tx, request)?;
    tx.commit()?;
    return Ok(());
}
