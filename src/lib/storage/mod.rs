use std::fmt;
use serde_json;

#[derive(Debug)]
pub enum StorageError {
    SerdeError(serde_json::Error),
    StorageError(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            StorageError::StorageError(e) => write!(f, "Storage error: {}", e),
        }
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerdeError(err)
    }
}

pub trait KeyValueStorage {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError>;
    fn set(&self, key: &str, value: &str) -> Result<(), StorageError>;
    fn remove(&self, key: &str) -> Result<(), StorageError>;
}

pub struct Storage {
    backend: Box<dyn KeyValueStorage>,
}

impl Storage {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        let backend: Box<dyn KeyValueStorage> = Box::new(WebStorageBackend::new());

        #[cfg(not(target_arch = "wasm32"))]
        let backend: Box<dyn KeyValueStorage> = Box::new(file_storage::FileStorageBackend::new());

        Self { backend }
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        self.backend.get(key)
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        self.backend.set(key, value)
    }

    pub fn remove(&self, key: &str) -> Result<(), StorageError> {
        self.backend.remove(key)
    }
}

#[cfg(target_arch = "wasm32")]
mod web_storage {
    use super::*;
    use gloo_storage::{LocalStorage, Storage as GlooStorage};

    pub struct WebStorageBackend;

    impl WebStorageBackend {
        pub fn new() -> Self {
            Self
        }
    }

    impl KeyValueStorage for WebStorageBackend {
        fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
            match LocalStorage::get(key) {
                Ok(value) => Ok(Some(value)),
                Err(_) => Ok(None),
            }
        }

        fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
            LocalStorage::set(key, value)
                .map_err(|e| StorageError::StorageError(e.to_string()))
        }

        fn remove(&self, key: &str) -> Result<(), StorageError> {
            LocalStorage::delete(key);
            Ok(())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod file_storage {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    pub struct FileStorageBackend {
        storage_dir: PathBuf,
    }

    impl FileStorageBackend {
        pub fn new() -> Self {
            let exe_path = std::env::current_exe()
                .unwrap_or_else(|_| PathBuf::from("."));
            let storage_dir = exe_path.parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            fs::create_dir_all(&storage_dir).ok();
            Self { storage_dir }
        }

        fn get_file_path(&self, key: &str) -> PathBuf {
            self.storage_dir.join(format!("{}.json", key))
        }
    }

    impl KeyValueStorage for FileStorageBackend {
        fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
            match fs::read_to_string(self.get_file_path(key)) {
                Ok(content) => Ok(Some(content)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(StorageError::StorageError(e.to_string())),
            }
        }

        fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
            let json: serde_json::Value = serde_json::from_str(value)?;
            let pretty = serde_json::to_string_pretty(&json)?;
            fs::write(self.get_file_path(key), pretty)
                .map_err(|e| StorageError::StorageError(e.to_string()))
        }

        fn remove(&self, key: &str) -> Result<(), StorageError> {
            let _ = fs::remove_file(self.get_file_path(key));
            Ok(())
        }
    }
}