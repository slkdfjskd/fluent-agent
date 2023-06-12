use crate::domain::proto::entity::{Entry, EntryRef};

pub struct EnvironmentRef<'a> {
    pub id: i64,
    pub env_name: &'a str,
    pub name: &'a str,
    pub value: &'a str,
}

pub struct Environment {
    pub id: i64,
    pub env_name: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ConfigRef<'a> {
    pub id: i64,
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub id: i64,
    pub key: String,
    pub value: String,
}
