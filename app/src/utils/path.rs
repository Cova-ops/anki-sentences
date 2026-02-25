use std::{
    env,
    path::{Path, PathBuf},
};

use crate::helpers::error_handler::{AppError, AppErrorKind, CsvParseError};

pub fn validate_save_filename<P, S>(name: P, ext: &[S]) -> Result<(), AppError>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let name = name.as_ref();

    let file_name = get_filename_from_path(name)?;

    if !file_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        return Err(AppError {
            kind: AppErrorKind::Csv(CsvParseError {
                file: name.to_owned(),
                row: None,
                column: None,
                message: String::from(
                    "File name can only contain Alphanumerics and '_', '-', '.'.",
                ),
            }),
            context: vec![],
        });
    }

    if !ext.iter().any(|x| file_name.ends_with(x.as_ref())) {
        return Err(AppError {
            kind: AppErrorKind::Csv(CsvParseError {
                file: name.to_owned(),
                row: None,
                column: None,
                message: format!("Extension not allowed"),
            }),
            context: vec![],
        });
    }

    Ok(())
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
