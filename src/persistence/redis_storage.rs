use deadpool_redis::{redis::cmd, Pool};
use rocket::serde::json;

pub struct RedisStorage {
    pool: Pool
}

impl RedisStorage {
    pub fn new() -> Self {
        let cfg = deadpool_redis::Config::from_url(std::env::var("REDIS_URL").unwrap());
        let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).unwrap();

        RedisStorage {
            pool,
        }
    }

    pub async fn persist_json<T: serde::Serialize + std::marker::Sync>(&self, event_id: &str, data: &T) -> Result<super::PersistenceResult, super::PersistenceError> { 
        let roles = json::to_string(&data);
        if let Err(e) = roles {
            println!("File persising failed: {}", e);
            return Err(super::PersistenceError::JsonParseFailed);
        }

        let roles = roles.unwrap();
        if let Ok(mut conn) = self.pool.get().await {
            cmd("SET")
                .arg(&[event_id, roles.as_ref()])
                .query_async::<()>(&mut conn)
                .await.unwrap();

            return Ok(super::PersistenceResult::Success);
        }

        return Err(super::PersistenceError::ReadFileFailed); // fix this error to no connection
    }

    pub async fn retrieve_json<T: serde::de::DeserializeOwned>(&self, event_id: &str) -> Result<T, super::PersistenceError> {
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
