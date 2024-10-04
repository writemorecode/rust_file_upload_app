use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::web;
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::env;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct FileObject {
    uuid: Uuid,
}

impl FileObject {
    fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[derive(Clone, Debug)]
pub struct AppState {
    upload_path: PathBuf,
}
impl AppState {
    pub fn new(upload_directory_name: &str) -> std::io::Result<AppState> {
        let cwd = env::current_dir()?;
        let upload_path = cwd.join(upload_directory_name);
        match upload_path.try_exists() {
            Ok(true) => Ok(AppState { upload_path }),
            Ok(false) => Err(std::io::ErrorKind::NotFound.into()),
            Err(err) => Err(err),
        }
    }
}

#[post("/upload")]
pub async fn file_upload(
    app_data: web::Data<AppState>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let response = FileObject::new();

    form.file.file.persist(&app_data.upload_path).unwrap();

    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempdir::TempDir;

    #[test]
    fn appstate_from_existing_directory() {
        let upload_dir = TempDir::new("uploads").unwrap();
        let upload_dir_name = upload_dir.path().to_str().unwrap();
        let state = AppState::new(upload_dir_name);
        assert!(state.is_ok());
    }

    #[test]
    fn appstate_from_nonexistent_directory() {
        let upload_dir_name = "does_not_exist";
        let state = AppState::new(upload_dir_name);
        assert!(state.is_err());
    }
}
