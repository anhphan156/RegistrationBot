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

pub enum PersistenceError {
    JsonParseFailed,
    ReadFileFailed,
    NoFileName,
}

pub enum PersistenceResult {
    Success,
}

impl Persistence for FileStorage<'_> {
    type PersistenceError = PersistenceError;
    type PersistenceResult = PersistenceResult;

    fn persist_json<T: Serialize>(&self, data: &T) -> Result<PersistenceResult, PersistenceError> {
        let roles = json::to_string(&data);
        if let Err(e) = roles {
            println!("File persising failed: {}", e);
            return Err(PersistenceError::JsonParseFailed);
        }

        if let None = self.file_name {
            return Err(PersistenceError::NoFileName);
        }

        let path = self.file_name.unwrap_or_default();
        match fs::write(path, roles.unwrap_or_default()) {
            _ => {}
        };

        Ok(PersistenceResult::Success)
    }

    fn retrieve_json<T>(&self) -> Result<T, PersistenceError> where T: DeserializeOwned {

        if let None = self.file_name {
            return Err(PersistenceError::NoFileName);
        }

        let path = self.file_name.unwrap_or_default();
        match fs::read_to_string(path) {
            Ok(content) => {
                match json::from_str::<T>(&content) {
                    Ok(json) => Ok(json),
                    Err(e) => {
                        println!("Retrieve json failed: {}", e);
                        return Err(PersistenceError::JsonParseFailed);
                    }
                }
            }
            Err(e) => {
                println!("Retrieve json failed: {}", e);
                return Err(PersistenceError::ReadFileFailed);
            }
        }
    }
}
