use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::web;
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::file_upload_service::appstate;

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
