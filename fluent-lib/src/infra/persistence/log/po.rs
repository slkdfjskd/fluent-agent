// CREATE TABLE IF NOT EXISTS request_log (
// id              BIGINT PRIMARY KEY ,
// request_id      BIGINT NOT NULL,
// base_url        VARCHAR(128) NOT NULL
// path            VARCHAR(256) NOT NULL
// result          TEXT NOT NULL,
// info            TEXT NOT NULL,
// request         TEXT NOT NULL,
// response        TEXT NOT NULL,
// created_at      DATETIME NOT NULL,
// updated_at      DATETIME NOT NULL
// );

use chrono::{DateTime, Local};

pub struct RequestLog {
    pub id: i64,
    pub request_id: i64,
    pub base_url: String,
    pub path: String,
    pub error: String,
    pub info: String,
    pub request: String,
    pub response: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct RequestLogRef<'a> {
    pub id: i64,
    pub request_id: i64,
    pub base_url: &'a str,
    pub path: &'a str,
    pub error: String,
    pub info: String,
    pub request: String,
    pub response: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
