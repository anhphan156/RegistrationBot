use serde::{de::DeserializeOwned, Serialize};

pub mod file_storage;

pub trait Persistence {
    type PersistenceError;
    type PersistenceResult;

    fn persist_json<T: Serialize>(&self, _: &T) -> Result<Self::PersistenceResult, Self::PersistenceError> { 
        unimplemented!()
    }

    fn retrieve_json<T>(&self) -> Result<T, Self::PersistenceError> where T: DeserializeOwned {
        unimplemented!()
    }
}
