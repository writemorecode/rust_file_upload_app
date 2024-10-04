use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use env_logger::Env;

use crate::upload::file_upload;

pub mod upload;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    const UPLOAD_DIRECTORY_NAME: &str = "uploads";
    let app_state = upload::AppState::new(UPLOAD_DIRECTORY_NAME)?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(file_upload)
    })
    .bind(("::1", 5000))?
    .run()
    .await
}
