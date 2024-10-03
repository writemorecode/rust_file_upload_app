use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use env_logger::Env;
use std::env;

mod upload;

use upload::file_upload;

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
