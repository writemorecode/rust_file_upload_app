use std::env;
use std::path::PathBuf;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tempdir::TempDir;

use uuid::Uuid;

use crate::file_upload_service::upload::FileObject;

#[derive(Clone, Debug)]
pub struct AppState {
    pub upload_path: PathBuf,
    pub file_table: Arc<RwLock<HashMap<Uuid, FileObject>>>,
}
impl AppState {
    pub fn new(upload_directory_name: &str) -> std::io::Result<AppState> {
        let cwd = env::current_dir()?;
        let upload_path = cwd.join(upload_directory_name);
        match upload_path.try_exists() {
            Ok(true) => Ok(AppState {
                upload_path,
                file_table: Arc::new(RwLock::new(HashMap::new())),
            }),
            Ok(false) => Err(std::io::ErrorKind::NotFound.into()),
            Err(err) => Err(err),
        }
    }

    pub fn new_temporary() -> AppState {
        let upload_path = TempDir::new("").unwrap();
        AppState {
            upload_path: upload_path.path().to_path_buf(),
            file_table: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempdir::TempDir;

    #[test]
    fn can_create_appstate_from_existing_directory() {
        let upload_dir = TempDir::new("uploads").unwrap();
        let upload_dir_name = upload_dir.path().to_str().unwrap();
        let state = AppState::new(upload_dir_name);
        assert!(state.is_ok());
    }

    #[test]
    fn cannot_create_appstate_from_nonexistent_directory() {
        let upload_dir_name = "does_not_exist";
        let state = AppState::new(upload_dir_name);
        assert!(state.is_err());
    }
}
