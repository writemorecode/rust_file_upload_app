#[derive(Clone, Debug)]
pub struct AppState {
    upload_path: PathBuf,
}
impl AppState {
    pub fn new(upload_directory_name: &str) -> std::io::Result<AppState> {
        let cwd = env::current_dir()?;
        let upload_path = cwd.join(upload_directory_name);
        match upload_path.try_exists() {
            Ok(true) => Ok(AppState { upload_path }),
            Ok(false) => Err(std::io::ErrorKind::NotFound.into()),
            Err(err) => Err(err),
        }
    }
}
