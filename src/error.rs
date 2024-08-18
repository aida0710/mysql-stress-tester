use std::fmt;
use mysql::Error as MySqlError;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    JsonError(json::Error),
    MissingField(String),
    MySqlError(MySqlError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::JsonError(e) => write!(f, "JSON parsing error: {}", e),
            AppError::MissingField(field) => write!(f, "Missing field in JSON: {}", field),
            AppError::MySqlError(field) => write!(f, "sqlの接続時にエラーが発生しました: {}", field)
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

impl From<MySqlError> for AppError {
    fn from(err: MySqlError) -> AppError {
        AppError::MySqlError(err)
    }
}