use crate::common::error::Result;
use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn init() -> Result {
    Ok(SimpleLogger::new().with_level(LevelFilter::Debug).init()?)
}
