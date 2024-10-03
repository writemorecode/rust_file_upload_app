use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::middleware::Logger;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};

use env_logger::Env;
use std::env;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[post("/upload")]
async fn file_upload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let path = format!("uploads/{}", form.file.file_name.unwrap());
    form.file.file.persist(&path).unwrap();
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(Logger::default()).service(file_upload))
        .bind(("::1", 5000))?
        .run()
        .await
}
