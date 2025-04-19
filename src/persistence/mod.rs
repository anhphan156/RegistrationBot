use serde::{de::DeserializeOwned, Serialize};

pub mod file_storage;
pub mod redis_storage;

#[rocket::async_trait]
pub trait Persistence {
    type PersistenceError;
    type PersistenceResult;

    async fn persist_json<T: Serialize + std::marker::Sync>(&self, _: &T) -> Result<Self::PersistenceResult, Self::PersistenceError> { 
        unimplemented!()
    }

    fn retrieve_json<T>(&self) -> Result<T, Self::PersistenceError> where T: DeserializeOwned {
        unimplemented!()
    }
}
