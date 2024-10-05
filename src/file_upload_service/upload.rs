use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, web};
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use chrono::prelude::*;

use crate::file_upload_service::appstate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileObject {
    original_filename: String,
    date_created: DateTime<Utc>,
}

impl FileObject {
    fn new(original_filename: String) -> Self {
        Self {
            original_filename,
            date_created: Utc::now(),
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

    let uuid = Uuid::new_v4();
    let file_object = FileObject::new(original_filename);
    app_data
        .file_table
        .write()
        .unwrap()
        .insert(uuid, file_object);

    let file_path = app_data.upload_path.join(uuid.to_string());
    if let Err(err) = form.file.file.persist(&file_path) {
        return Ok(HttpResponse::InternalServerError().body(err.to_string()));
    }
    let response = HttpResponse::Created().json(json!({"uuid":uuid}));
    Ok(response)
}

#[get("/upload/{uuid}")]
pub async fn file_query(
    app_data: web::Data<appstate::AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let uuid = path.into_inner();
    let table = app_data.file_table.read().unwrap();
    let found = table.get(&uuid);
    let response = match found {
        Some(file_object) => HttpResponse::Found().json(file_object),
        None => HttpResponse::NotFound().body("File not found"),
    };
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_multipart::test::create_form_data_payload_and_headers;
    use actix_web::test::TestRequest;
    use actix_web::web::Bytes;
    use actix_web::{http, test, App};
    use appstate::AppState;
    use mime;

    #[test]
    async fn test_file_upload() {
        let app_state = AppState::new_temporary();
        std::fs::create_dir_all(&app_state.upload_path).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .service(file_upload),
        )
        .await;
        let (body, headers) = create_form_data_payload_and_headers(
            "file",
            Some("lorem.txt".to_owned()),
            Some(mime::TEXT_PLAIN),
            Bytes::from_static(b"Lorem ipsum."),
        );
        let req = TestRequest::post().uri("/upload");
        let req = headers
            .into_iter()
            .fold(req, |req, hdr| req.insert_header(hdr))
            .set_payload(body)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
