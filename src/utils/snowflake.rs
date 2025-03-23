use std::sync::atomic::AtomicU64;

pub struct Snowflake {
    pub machine_id: u32,
    pub last_timestamp: AtomicU64,
    pub sequence: AtomicU64,
}

impl Snowflake {
    pub fn new() -> Self {
        Snowflake {
            machine_id: 0,
            last_timestamp: AtomicU64::new(0),
            sequence: AtomicU64::new(0),
        }
    }
}
