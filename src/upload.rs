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
    original_filename: String,
}

impl FileObject {
    fn new(original_filename: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            original_filename,
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
) -> actix_web::Result<impl Responder> {
    form.file.file.persist(&app_data.upload_path).unwrap();

    let original_filename = form.file.file_name;
    let response = match original_filename {
        Some(name) => HttpResponse::Ok().json(FileObject::new(name)),
        None => HttpResponse::BadRequest().body("The uploaded file must have a filename."),
    };
    Ok(response)
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
