use crate::errors::SubmissionError;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST}; // Assuming SubmissionError is in scope

// Hashes a password using bcrypt
pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

// Verifies a password against a bcrypt hash
// Changed return type to Result<bool, BcryptError> to propagate errors
pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}

// Optional: Map BcryptError to your SubmissionError if needed elsewhere
impl From<BcryptError> for SubmissionError {
    fn from(err: BcryptError) -> Self {
        SubmissionError::HashingError(format!("bcrypt error: {}", err))
    }
}
