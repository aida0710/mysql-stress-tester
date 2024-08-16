#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    JsonError(json::Error),
    MissingField(String),
    InvalidType(String, String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::JsonError(e) => write!(f, "JSON parsing error: {}", e),
            AppError::MissingField(field) => write!(f, "Missing field in JSON: {}", field),
            AppError::InvalidType(field, expected_type) =>
                write!(f, "Invalid type for field '{}', expected {}", field, expected_type),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::IoError(err)
    }
}

impl From<json::Error> for AppError {
    fn from(err: json::Error) -> AppError {
        AppError::JsonError(err)
    }
}