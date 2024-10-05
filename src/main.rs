use std::env;

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use env_logger::Env;

pub mod file_upload_service;

use file_upload_service::appstate::AppState;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    const UPLOAD_DIRECTORY_NAME: &str = "uploads";
    let cwd = env::current_dir()?;
    let upload_directory_path = cwd.join(UPLOAD_DIRECTORY_NAME);
    std::fs::create_dir_all(upload_directory_path)?;

    let app_state = AppState::new(UPLOAD_DIRECTORY_NAME)?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(healthcheck)
            .service(file_upload_service::upload::file_upload)
            .service(file_upload_service::upload::file_query)
    })
    .bind(("::1", 5000))?
    .workers(1)
    .run()
    .await
}
