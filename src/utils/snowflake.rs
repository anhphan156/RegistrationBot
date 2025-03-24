use std::{sync::atomic::{AtomicU64, Ordering}, time::{SystemTime, UNIX_EPOCH}};

const EPOCH: u64 = 1_288_834_974_657; // Twitter Snowflake epoch (Nov 4, 2010)
const MACHINE_ID_BITS: u64 = 10;
const SEQUENCE_BITS: u64 = 12;
const MAX_MACHINE_ID: u64 = (1 << MACHINE_ID_BITS) - 1;
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;
const TIMESTAMP_SHIFT: u64 = MACHINE_ID_BITS + SEQUENCE_BITS;
const MACHINE_ID_SHIFT: u64 = SEQUENCE_BITS;

pub type Snowflake = String;

pub struct SnowflakeGenerator {
    pub machine_id: u64,
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
    pub fn generate(&self) -> Snowflake {
        let mut timestamp = self.current_timestamp();
        let last_timestamp = self.last_timestamp.load(Ordering::SeqCst);
        let mut sequence = self.sequence.load(Ordering::SeqCst);

        if timestamp == last_timestamp {
            sequence = (sequence + 1) & MAX_SEQUENCE;
            if sequence == 0 {
                timestamp = self.get_next_milisec(last_timestamp);
            }
        } else {
            sequence = 0;
        }

        self.last_timestamp.store(timestamp, Ordering::SeqCst);
        self.sequence.store(sequence, Ordering::SeqCst);

        let snowflake = (timestamp << TIMESTAMP_SHIFT)
            | (self.machine_id << MACHINE_ID_SHIFT)
            | sequence;

        format!("{}", snowflake)
    }

    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward")
            .as_millis() as u64 - EPOCH
    }
    fn get_next_milisec(&self, last_timestamp: u64) -> u64 {
        let mut timestamp = self.current_timestamp();
        while timestamp <= last_timestamp {
            timestamp = self.current_timestamp();
        }

        timestamp
    }
}
