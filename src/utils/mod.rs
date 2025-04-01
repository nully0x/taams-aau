use log::info;
use std::fs;
use std::path::Path;

pub fn ensure_upload_dir() -> std::io::Result<()> {
    let upload_dir = Path::new("./data/uploads");
    if !upload_dir.exists() {
        info!("Creating uploads directory...");
        fs::create_dir_all(upload_dir)?;
    }
    Ok(())
}
