use crate::common::error::Result;
use crate::infra::persistence;
use crate::infra::persistence::CONN;
use std::sync::Mutex;

lazy_static! {
    pub static ref UNIQUE_ID: Mutex<UniqueId> = Mutex::new(UniqueId::new());
}

const UNIQUE_ID_COUNT: i64 = 10;

pub struct UniqueId {
    start: i64,
    count: i64,
    offset: i64,
}

impl UniqueId {
    pub fn new() -> Self {
        UniqueId {
            start: 0,
            count: 0,
            offset: 0,
        }
    }

    pub fn generate(&mut self) -> Result<i64> {
        if self.offset >= self.start + self.count {
            let max_unique_id = persistence::config::db::get_unique_id(&CONN, UNIQUE_ID_COUNT)?;
            self.start = max_unique_id;
            self.count = UNIQUE_ID_COUNT;
            self.offset = max_unique_id;
        }
        let r = self.offset;
        self.offset += 1;
        return Ok(r);
    }
}

pub fn generate() -> Result<i64> {
    let mut unique_id = UNIQUE_ID.lock().unwrap();
    return unique_id.generate();
}
