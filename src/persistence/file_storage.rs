use super::Persistence;
use std::fs;
use rocket::serde::json;
use serde::{de::DeserializeOwned, Serialize};

pub struct FileStorage<'a> {
    file_name: Option<&'a str>,
}

impl<'a> FileStorage<'a> {
    pub fn new(file_name: &'a str) -> Self {
        FileStorage {
            file_name: Some(file_name),
        }
    }
}

#[rocket::async_trait]
impl Persistence for FileStorage<'_> {
    type PersistenceError = super::PersistenceError;
    type PersistenceResult = super::PersistenceResult;

    async fn persist_json<T: Serialize + std::marker::Sync>(&self, data: &T) -> Result<Self::PersistenceResult, Self::PersistenceError> {
        let roles = json::to_string(&data);
        if let Err(e) = roles {
            println!("File persising failed: {}", e);
            return Err(super::PersistenceError::JsonParseFailed);
        }

        if let None = self.file_name {
            return Err(super::PersistenceError::NoFileName);
        }

        let path = self.file_name.unwrap_or_default();
        match fs::write(path, roles.unwrap_or_default()) {
            _ => {}
        };

        Ok(super::PersistenceResult::Success)
    }

    fn retrieve_json<T>(&self) -> Result<T, super::PersistenceError> where T: DeserializeOwned {

        if let None = self.file_name {
            return Err(super::PersistenceError::NoFileName);
        }

        let path = self.file_name.unwrap_or_default();
        match fs::read_to_string(path) {
            Ok(content) => {
                match json::from_str::<T>(&content) {
                    Ok(json) => Ok(json),
                    Err(e) => {
                        println!("Retrieve json failed: {}", e);
                        return Err(super::PersistenceError::JsonParseFailed);
                    }
                }
            }
            Err(e) => {
                println!("Retrieve json failed: {}", e);
                return Err(super::PersistenceError::ReadFileFailed);
            }
        }
    }
}
