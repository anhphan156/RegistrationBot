use std::error::Error;

use chrono::{DateTime, NaiveDateTime, Utc};

pub struct RegistrationTime;

impl RegistrationTime {
    pub fn utc_to_unix(time: String) -> Result<i64, Box<dyn Error>> {
        let time = format!("{} +0000", time);
        let time = DateTime::parse_from_str(&time, "%m/%d/%Y %H:%M %P %z")?;
        let timestamp = time.timestamp();
        Ok(timestamp)
    }

    pub fn unix_to_utc(time: i64) -> String {
        let naive_datetime = NaiveDateTime::from_timestamp_opt(time, 0).unwrap();
        let datetime: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);

        datetime.format("%A, %B %d %Y at %H:%M %p").to_string()
    }
}
