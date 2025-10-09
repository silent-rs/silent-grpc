use chrono::{Local, NaiveDateTime};
use scru128::new_string;

pub const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const DEFAULT_STREAM_COUNT: usize = 5;
pub const STREAM_INTERVAL_MS: u64 = 250;

#[derive(Debug, Clone)]
pub struct GreetingRecord {
    pub id: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct SummaryRecord {
    pub id: String,
    pub total: u32,
    pub names: Vec<String>,
    pub created_at: NaiveDateTime,
}

pub fn validate_name(name: &str) -> Result<String, &'static str> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("名称不能为空");
    }
    Ok(trimmed.to_owned())
}

pub fn create_greeting(name: &str) -> GreetingRecord {
    GreetingRecord {
        id: new_string(),
        message: format!("Hello, {}!", name),
        created_at: Local::now().naive_local(),
    }
}

pub fn create_summary(names: Vec<String>) -> SummaryRecord {
    SummaryRecord {
        id: new_string(),
        total: names.len() as u32,
        names,
        created_at: Local::now().naive_local(),
    }
}

pub fn format_timestamp(datetime: NaiveDateTime) -> String {
    datetime.format(TIME_FORMAT).to_string()
}
