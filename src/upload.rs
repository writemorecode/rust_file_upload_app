use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct FileUploadResponse {
    uuid: Uuid,
}

impl FileUploadResponse {
    fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

impl fmt::Display for FileUploadResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[post("/upload")]
pub async fn file_upload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let response = FileUploadResponse::new();

    // TODO: Use proper path joining here
    // TODO: Store name/path of upload dir in shared app state
    let path = format!("uploads/{}", response);
    println!("'{}' => '{}'", form.file.file_name.unwrap(), response.uuid);
    form.file.file.persist(&path).unwrap();

    HttpResponse::Ok().json(response)
}
