use crate::common::error::Result;
use crate::domain::log::entity;
use crate::infra::persistence::log::po;
use chrono::Local;

pub fn to_request_log_po(log: &entity::RequestLog) -> Result<po::RequestLogRef> {
    let error = log
        .error
        .as_ref()
        .map_or(Ok("".to_string()), |e| serde_json::to_string(e))?;
    let response = log
        .response
        .as_ref()
        .map_or(Ok("".to_string()), |r| serde_json::to_string(r))?;
    let log = po::RequestLogRef {
        id: log.id,
        request_id: log.request_id,
        base_url: &log.base_url,
        path: &log.path,
        error,
        info: log.info.to_string(),
        request: serde_json::to_string(&log.request)?,
        response,
        created_at: Local::now(),
        updated_at: Local::now(),
    };

    Ok(log)
}

pub fn batch_to_request_log_entity(logs: Vec<po::RequestLog>) -> Result<Vec<entity::RequestLog>> {
    let mut result = vec![];
    for log in logs {
        let opt_log = to_request_log_entity(Some(log))?;
        if let Some(log) = opt_log {
            result.push(log);
        }
    }

    return Ok(result);
}

pub fn to_request_log_entity(log: Option<po::RequestLog>) -> Result<Option<entity::RequestLog>> {
    if log.is_none() {
        return Ok(None);
    }
    let log = log.unwrap();

    let error = if !log.error.is_empty() {
        Some(serde_json::from_str(&log.error)?)
    } else {
        None
    };
    let response = if !log.response.is_empty() {
        Some(serde_json::from_str(&log.response)?)
    } else {
        None
    };

    let log = entity::RequestLog {
        id: log.id,
        request_id: log.request_id,
        base_url: log.base_url,
        path: log.path,
        error,
        info: log.info,
        request: serde_json::from_str(&log.request)?,
        response,
        created_at: log.created_at,
    };

    return Ok(Some(log));
}
