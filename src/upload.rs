use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
//use std::fmt;
use uuid::Uuid;

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

#[post("/upload")]
pub async fn file_upload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let response = FileObject::new();

    // TODO: Use proper path joining here
    // TODO: Store name/path of upload dir in shared app state
    let path = format!("uploads/{}", response.uuid);
    form.file.file.persist(&path).unwrap();

    HttpResponse::Ok().json(response)
}
