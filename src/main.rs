use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer};

use env_logger::Env;

use crate::upload::file_upload;

pub mod upload;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    const UPLOAD_DIRECTORY_NAME: &str = "uploads";
    let app_state = upload::AppState::new(UPLOAD_DIRECTORY_NAME)?;
    dbg!(&app_state);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(file_upload)
    })
    .bind(("::1", 5000))?
    .workers(1)
    .run()
    .await
}
