use crate::domain::proto::entity::EntryRef;

#[derive(Debug, Clone)]
pub struct RequestRef<'a> {
    pub id: i64,
    pub name: &'a str,
    pub url: &'a str,
    pub method: &'a str,
    pub headers: Vec<EntryRef<'a>>,
    pub params: Vec<EntryRef<'a>>,
    pub req_json: Option<serde_json::Value>,
    pub resp_json: Option<serde_json::Value>,
}
