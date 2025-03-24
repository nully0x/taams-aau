use serde::Serialize;

#[derive(Serialize)]
pub struct SubmissionResponse {
    pub success: bool,
    pub submission_id: i32,
    pub message: String,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub journal_id: i32,
    pub message: String,
}

#[derive(Serialize)]
pub struct ValidationResponse {
    pub field: String,
    pub message: String,
}
