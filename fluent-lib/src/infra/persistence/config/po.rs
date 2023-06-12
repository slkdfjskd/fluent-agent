use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Environment {
    pub id: i64,
    pub env_name: String,
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct EnvironmentRef<'a> {
    pub id: i64,
    pub env_name: &'a str,
    pub name: &'a str,
    pub value: &'a str,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct Config {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct ConfigRef<'a> {
    pub id: i64,
    pub key: &'a str,
    pub value: &'a str,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct UniqueId {
    pub id: i64,
    pub unique_id: i64,
}
