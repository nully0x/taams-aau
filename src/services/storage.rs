use firebase_storage::FirebaseStorage;
use std::path::Path;

pub struct StorageService {
    storage: FirebaseStorage,
}

impl StorageService {
    pub fn new(bucket_url: &str, credentials_path: &str) -> Self {
        let storage = FirebaseStorage::new(bucket_url, credentials_path)
            .expect("Failed to initialize Firebase Storage");
        Self { storage }
    }

    pub async fn upload_pdf(&self, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

        let destination = format!("submissions/{}", file_name);
        self.storage
            .upload(file_path, &destination)
            .await
            .map_err(|e| e.into())
    }
}
