use chrono::{DateTime, Local};

pub fn to_string_rfc3339(date: &DateTime<Local>) -> String {
    return date.to_rfc3339();
}

pub fn format_date(date: String) -> DateTime<Local> {
    let r = DateTime::parse_from_rfc3339(&date);
    match r {
        Ok(d) => DateTime::from(d),
        Err(e) => {
            warn!(
                "DateTime::parse_from_rfc3339 error:{}  date_str:{}",
                e, date
            );
            Local::now()
        }
    }
}

pub fn to_string(date: &DateTime<Local>) -> String {
    return date.format("%Y-%m-%d %H:%M:%S").to_string();
}
