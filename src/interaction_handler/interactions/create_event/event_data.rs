use derive_builder::Builder;
use crate::utils::snowflake::Snowflake;

#[derive(Builder)]
pub struct EventData {
    event_id: Snowflake, // &'r str 
    event_time: i64,
}

impl EventData {
    pub fn get_event_id(&self) -> &str {
        self.event_id.as_ref()
    }
}
