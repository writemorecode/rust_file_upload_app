use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::web;
use actix_web::{get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub mod appstate {
    use std::env;
    use std::path::PathBuf;

    #[derive(Clone, Debug)]
    pub struct AppState {
        pub upload_path: PathBuf,
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
}

#[post("/upload")]
pub async fn file_upload(
    app_data: web::Data<appstate::AppState>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> actix_web::Result<impl Responder> {
    let Some(original_filename) = form.file.file_name else {
        return Ok(HttpResponse::BadRequest().body("The uploaded file must have a filename."));
    };
    let file_object = FileObject::new(original_filename);
    let file_path = app_data.upload_path.join(file_object.uuid.to_string());
    if let Err(err) = form.file.file.persist(&file_path) {
        return Ok(HttpResponse::InternalServerError().body(err.to_string()));
    }
    let response = HttpResponse::Ok().json(file_object);
    Ok(response)
}

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[test]
    async fn passes_healthcheck() {
        let app = test::init_service(App::new().service(healthcheck)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }
}
