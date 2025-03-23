use std::sync::atomic::AtomicU64;

pub struct SnowflakeGenerator {
    pub machine_id: u32,
    pub last_timestamp: AtomicU64,
    pub sequence: AtomicU64,
}

impl SnowflakeGenerator {
    pub fn new() -> Self {
        SnowflakeGenerator {
            machine_id: 0,
            last_timestamp: AtomicU64::new(0),
            sequence: AtomicU64::new(0),
        }
    }
}
