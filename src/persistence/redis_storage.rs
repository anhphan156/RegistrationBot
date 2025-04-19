use deadpool_redis::{redis::cmd, Pool};

use super::Persistence;

pub struct RedisStorage {
    pool: Pool
}

impl RedisStorage {
    pub fn new() -> Self {
        let cfg = deadpool_redis::Config::from_url(std::env::var("REDIS_URL").unwrap());
        let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).unwrap();

        RedisStorage {
            pool
        }
    }
}

pub enum PersistenceError {
    JsonParseFailed,
    ReadFileFailed,
    NoFileName,
}

pub enum PersistenceResult {
    Success,
}

#[rocket::async_trait]
impl Persistence for RedisStorage {
    type PersistenceError = PersistenceError;

    type PersistenceResult = PersistenceResult;

    async fn persist_json<T: serde::Serialize + std::marker::Sync>(&self, _: &T) -> Result<Self::PersistenceResult, Self::PersistenceError> { 
        if let Ok(mut conn) = self.pool.get().await {
            cmd("SET")
                .arg(&["reg_bot/event1", "42"])
                .query_async::<()>(&mut conn)
                .await.unwrap();
        }

        Ok(PersistenceResult::Success)
    }

    fn retrieve_json<T>(&self) -> Result<T, Self::PersistenceError> where T: serde::de::DeserializeOwned {
        std::unimplemented!()
    }
}
