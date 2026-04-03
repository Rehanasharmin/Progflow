use std::fmt;

#[derive(Debug)]
pub enum AppError {
    User(String),
    Io(String, std::io::Error),
    Json(String, serde_json::Error),
    Config(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::User(msg) => write!(f, "{}", msg),
            AppError::Io(path, err) => {
                write!(f, "IO error: {}: {}", path, err)
            }
            AppError::Json(path, err) => {
                write!(f, "JSON error in {}: {}", path, err)
            }
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io("unknown".to_string(), err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json("unknown".to_string(), err)
    }
}

impl AppError {
    pub fn with_suggestion(msg: &str, suggestion: &str) -> Self {
        AppError::User(format!("{}. {}", msg, suggestion))
    }

    pub fn is_user_error(&self) -> bool {
        matches!(self, AppError::User(_))
    }
}
