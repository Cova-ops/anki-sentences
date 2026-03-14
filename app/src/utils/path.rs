use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::helpers::error_handler::{AppError, AppErrorKind, CsvParseError};

pub fn validate_save_filename(name: &str, ext: &[impl AsRef<str>]) -> Result<PathBuf, AppError> {
    let file_path: PathBuf = normalize_path(name)?;
    let file_name: &OsStr = match file_path.extension() {
        Some(v) => v,
        _ => {
            return Err(AppError {
                kind: AppErrorKind::Csv(CsvParseError {
                    file: file_path.to_owned(),
                    row: None,
                    column: None,
                    message: format!("Error with extension"),
                }),
                context: vec![],
            });
        }
    };

    if !ext.iter().any(|x| file_name == x.as_ref()) {
        return Err(AppError {
            kind: AppErrorKind::Csv(CsvParseError {
                file: file_path.to_owned(),
                row: None,
                column: None,
                message: format!("Extension not allowed"),
            }),
            context: vec![],
        });
    }

    Ok(file_path)
}

fn normalize_path(input: &str) -> std::io::Result<PathBuf> {
    let expanded: String = shellexpand::tilde(input).into_owned();
    let path: PathBuf = PathBuf::from(expanded);

    path.canonicalize()
}

pub fn path_to_string(path: &Path) -> Result<String, AppErrorKind> {
    path.to_str()
        .map(|x| x.to_owned())
        .ok_or_else(|| AppErrorKind::Internal(String::from("Error getting string from path")))
}

pub fn get_filename_from_path(path: &Path) -> Result<String, AppErrorKind> {
    let filename = path
        .file_name()
        .and_then(|x| x.to_str())
        .ok_or_else(|| {
            AppErrorKind::Internal(format!("Error get_filename_from_path: {}", path.display()))
        })?
        .to_owned();

    Ok(filename)
}

pub fn home_dir() -> PathBuf {
    if cfg!(windows) {
        env::var("USERPROFILE")
            .map(PathBuf::from)
            .expect("No se pudo obtener USERPROFILE")
    } else {
        env::var("HOME")
            .map(PathBuf::from)
            .expect("No se pudo obtener HOME")
    }
}
