use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadTestError {
    #[error("環境変数の読み込みに失敗しました: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("数値の解析に失敗しました: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("MySQLエラー: {0}")]
    MySqlError(#[from] mysql::Error),

    #[error("タスクの実行に失敗しました")]
    TaskExecutionError,
}

pub type Result<T> = std::result::Result<T, LoadTestError>;