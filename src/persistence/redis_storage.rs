use deadpool_redis::{redis::cmd, Pool};
use rocket::serde::json;

use super::Persistence;

pub struct RedisStorage {
    event_id: Option<String>,
    pool: Pool
}

impl RedisStorage {
    pub fn new() -> Self {
        let cfg = deadpool_redis::Config::from_url(std::env::var("REDIS_URL").unwrap());
        let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).unwrap();

        RedisStorage {
            event_id: None,
            pool
        }
    }

    pub fn event_id(&mut self, event_id: String) {
        self.event_id = Some(event_id)
    }
}

#[rocket::async_trait]
impl Persistence for RedisStorage {
    type PersistenceError = super::PersistenceError;

    type PersistenceResult = super::PersistenceResult;

    async fn persist_json<T: serde::Serialize + std::marker::Sync>(&self, data: &T) -> Result<Self::PersistenceResult, Self::PersistenceError> { 

        let roles = json::to_string(&data);
        if let Err(e) = roles {
            println!("File persising failed: {}", e);
            return Err(super::PersistenceError::JsonParseFailed);
        }

        if let None = self.event_id {
            return Err(super::PersistenceError::NoFileName);
        }

        let roles = roles.unwrap();
        let event_id : String = <Option<String> as Clone>::clone(&self.event_id).unwrap_or_default();
        if let Ok(mut conn) = self.pool.get().await {
            cmd("SET")
                .arg(&[event_id, roles])
                .query_async::<()>(&mut conn)
                .await.unwrap();

            return Ok(super::PersistenceResult::Success);
        }

        return Err(super::PersistenceError::ReadFileFailed); // fix this error to no connection
    }

    async fn retrieve_json<T>(&self) -> Result<T, Self::PersistenceError> where T: serde::de::DeserializeOwned {
        if let None = self.event_id {
            return Err(super::PersistenceError::NoFileName);
        }

        let event_id : String = <Option<String> as Clone>::clone(&self.event_id).unwrap_or_default();
        if let Ok(mut conn) = self.pool.get().await {
            let content: String = cmd("GET")
                .arg(&[event_id])
                .query_async(&mut conn)
                .await.unwrap_or_default();

            match json::from_str::<T>(&content) {
                Ok(json) => return Ok(json),
                Err(e) => {
                    println!("Retrieve json failed: {}", e);
                    return Err(super::PersistenceError::JsonParseFailed);
                }
            }
        }

        return Err(super::PersistenceError::ReadFileFailed); // fix this error to no connection
    }
}
