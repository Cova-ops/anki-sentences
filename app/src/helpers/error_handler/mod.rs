use std::{
    fmt::{self},
    path::PathBuf,
};

use reqwest::header::{self, HeaderMap};

pub mod result;

#[derive(Debug)]
pub struct AppError {
    pub kind: AppErrorKind,
    pub context: Vec<ErrorContext>,
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError {
            kind: AppErrorKind::Io(e),
            context: vec![],
        }
    }
}

impl From<Vec<InvalidValueError>> for AppError {
    fn from(value: Vec<InvalidValueError>) -> Self {
        Self {
            kind: AppErrorKind::Validation(ValidationError {
                issues: value
                    .into_iter()
                    .map(|v| FieldIssue {
                        row: None,
                        invalid: v,
                    })
                    .collect(),
            }),
            context: vec![],
        }
    }
}

impl From<toml::de::Error> for AppError {
    fn from(value: toml::de::Error) -> Self {
        Self {
            kind: AppErrorKind::Toml(TomlErrors::DE(value)),
            context: vec![],
        }
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(value: toml::ser::Error) -> Self {
        Self {
            kind: AppErrorKind::Toml(TomlErrors::SER(value)),
            context: vec![],
        }
    }
}

impl From<DbError> for AppError {
    fn from(value: DbError) -> Self {
        Self {
            kind: AppErrorKind::Db(value),
            context: vec![],
        }
    }
}

impl From<ConfigError> for AppError {
    fn from(value: ConfigError) -> Self {
        Self {
            kind: AppErrorKind::Config(value),
            context: vec![],
        }
    }
}

impl From<CsvParseError> for AppError {
    fn from(value: CsvParseError) -> Self {
        Self {
            kind: AppErrorKind::Csv(value),
            context: vec![],
        }
    }
}

impl From<AppErrorKind> for AppError {
    fn from(value: AppErrorKind) -> Self {
        Self {
            kind: value,
            context: vec![],
        }
    }
}

impl From<ApiError> for AppError {
    fn from(value: ApiError) -> Self {
        Self {
            kind: AppErrorKind::Api(value),
            context: vec![],
        }
    }
}

impl From<dotenvy::Error> for AppError {
    fn from(value: dotenvy::Error) -> Self {
        Self {
            kind: AppErrorKind::Internal(format!("Error loading envs: {value}")),
            context: vec![],
        }
    }
}

impl From<std::env::VarError> for AppError {
    fn from(value: std::env::VarError) -> Self {
        Self {
            kind: AppErrorKind::Internal(format!("Error getting env value: {value}")),
            context: vec![],
        }
    }
}

impl From<reqwest::header::InvalidHeaderValue> for AppError {
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        Self {
            kind: AppErrorKind::Internal(format!("Error creationg headers: {value}")),
            context: vec![],
        }
    }
}

impl AppError {
    pub fn with_ctx(mut self, label: &'static str, message: impl Into<String>) -> Self {
        self.context.push(ErrorContext {
            label,
            message: message.into(),
        });
        self
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            kind: AppErrorKind::JSON(format!("Error serialize JSON: {value}")),
            context: vec![],
        }
    }
}

#[derive(Debug)]
pub enum AppErrorKind {
    Io(std::io::Error),
    Toml(TomlErrors),
    Csv(CsvParseError),
    JSON(String),
    Validation(ValidationError),
    Db(DbError),
    Config(ConfigError),
    Internal(String),
    Audio(AudioError),
    Api(ApiError),
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub url: Option<String>,
    pub headers: HeaderMap<reqwest::header::HeaderValue>,
    pub method: String,
    pub payload: Option<String>,
    pub response: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AudioError {
    Decoder(rodio::decoder::DecoderError),
}

#[derive(Debug, Clone)]
pub struct ConfigError {
    pub message: String,
    pub action: String,
}

#[derive(Debug, Clone)]
pub enum TomlErrors {
    DE(toml::de::Error),
    SER(toml::ser::Error),
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub label: &'static str, // "import", "csv", "db", "validation"
    pub message: String,     // "reading file", "parsing row", etc.
}

#[derive(Debug)]
pub struct CsvParseError {
    pub file: PathBuf,
    pub row: Option<usize>, // 1-based for users
    pub column: Option<&'static str>,
    pub message: String,
}

#[derive(Debug)]
pub struct InvalidValueError {
    pub field: &'static str,
    pub message: String,
    pub valid_options: Option<Vec<&'static str>>,
}

#[derive(Debug)]
pub struct ValidationError {
    pub issues: Vec<FieldIssue>,
}

#[derive(Debug)]
pub struct FieldIssue {
    pub row: Option<usize>,
    pub invalid: InvalidValueError,
}

#[derive(Debug)]
pub struct DbError {
    pub sql: Option<String>,
    pub message: String,
    pub source: Option<rusqlite::Error>,
}

impl From<rusqlite::Error> for DbError {
    fn from(err: rusqlite::Error) -> Self {
        Self {
            sql: None,
            message: err.to_string(),
            source: Some(err),
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Database error: {}", self.message)?;

        if let Some(sql) = self.sql.as_deref() {
            writeln!(f, "SQL: {}", sql)?;
        }

        if let Some(source) = &self.source {
            writeln!(f, "Source: {}", source)?;
        }

        Ok(())
    }
}

impl std::error::Error for DbError {}

impl DbError {
    pub fn with_sql<'a>(sql: impl Into<String>) -> impl FnOnce(rusqlite::Error) -> DbError {
        move |e| DbError {
            sql: Some(sql.into()),
            message: e.to_string(),
            source: Some(e),
        }
    }
}
