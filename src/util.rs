use chrono::{NaiveDate, TimeZone, Utc};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use serde::Serialize;

use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn build_request<T: ToString>(parameters: &BTreeMap<String, T>) -> String {
    if parameters.is_empty() {
        return String::new();
    }

    let mut request = String::with_capacity(
        parameters
            .iter()
            .map(|(k, v)| k.len() + v.to_string().len() + 1)
            .sum(),
    );
    for (key, value) in parameters {
        request.push_str(key);
        request.push('=');
        let mut value = value.to_string();
        if value.starts_with("\"") && value.ends_with("\"") {
            // trim leading and trailing `"`
            value = value[1..value.len() - 1].to_string();
        }
        request.push_str(&value);
        request.push('&');
    }
    request.truncate(request.len() - 1);
    request
}

pub fn build_json_request<T: Serialize>(parameters: &BTreeMap<String, T>) -> String {
    serde_json::to_string(parameters).expect("Failed to serialize parameters to JSON")
}

pub fn to_i64(value: &Value) -> Option<i64> {
    value.as_i64()
}

pub fn to_u64(value: &Value) -> Option<u64> {
    value.as_u64()
}
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

pub fn date_to_milliseconds(date_str: &str) -> u64 {
    let naive_date = NaiveDate::parse_from_str(date_str, "%d%m%y").expect("Failed to parse date");
    let naive_date_time = naive_date
        .and_hms_opt(0, 0, 0)
        .expect("Failed to create NaiveDateTime");
    let datetime_utc = Utc.from_utc_datetime(&naive_date_time);
    datetime_utc.timestamp_millis() as u64
}

pub fn generate_random_uid(length: usize) -> String {
    let mut uid = String::with_capacity(length);
    for _ in 0..length {
        uid.push(rng().sample(Alphanumeric) as char);
    }
    uid
}
