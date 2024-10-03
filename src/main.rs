use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use env_logger::Env;
use std::env;

mod upload;

use upload::file_upload;

use std::io::ErrorKind;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const UPLOAD_DIRECTORY_NAME: &str = "uploads";
    let cwd_path = env::current_dir()?;
    let upload_directory_path = cwd_path.join(UPLOAD_DIRECTORY_NAME);
    println!("Current directory: {}", cwd_path.display());
    println!("Upload directory: {}", upload_directory_path.display());

    let mkdir_result = std::fs::create_dir(upload_directory_path);
    match mkdir_result {
        Err(ref err) if err.kind() == ErrorKind::AlreadyExists => {}
        Err(err) => {
            return Err(err);
        }
        _ => {}
    }

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(Logger::default()).service(file_upload))
        .bind(("::1", 5000))?
        .run()
        .await
}
