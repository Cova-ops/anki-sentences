use std::{
pub mod queries;
    fmt::{self, Display},
    path::PathBuf,
};

pub mod resul
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

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError {
            kind: AppErrorKind::Db(DbError {
                sql: None,
                message: "db error".into(),
                source: Some(e),
            }),
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

#[derive(Debug)]
pub enum AppErrorKind {
    Io(std::io::Error),
    Csv(CsvParseError),
    Validation(ValidationError),
    Db(DbError),
    Config(String),
    Internal(String),
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub label: &'static str, // "import", "csv", "db", "validation"
    pub message: String,     // "reading file", "parsing row", etc.
}

#[derive(Debug)]
pub struct CsvParseError {
    pub file: PathBuf,
    pub row: Option<usize>, // 1-based si quieres
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
    pub sql: Option<&'static str>,
    pub message: String,
    pub source: Option<rusqlite::Error>,
}

// impl From<rusqlite::Error> for DbError {
//     fn from(err: rusqlite::Error) -> Self {
//         Self {
//             sql: None,
//             message: err.to_string(),
//             source: Some(err),
//         }
//     }
// }

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Database error: {}", self.message)?;

        if let Some(sql) = self.sql {
            writeln!(f, "SQL: {}", sql)?;
        }

        if let Some(source) = &self.source {
            writeln!(f, "Source: {}", source)?;
        }

        Ok(())
    }
}

impl std::error::Error for DbError {}
