use std::{
    env,
    fs::canonicalize,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};

pub fn path_format_valid(path: &str) -> bool {
    Path::new(path).components().count() > 1
}

pub fn path_from_string(path: &str) -> &Path {
    Path::new(path)
}

/// Función para validar si una ruta es archivo o carpeta
///
/// return:
/// - Si te regresa un None, en la ruta no se encuentra ningun archivo
/// - En caso de regresa una tupla (bool, bool), el caso devolvera un true en caso de que coincida:
///   - El primer objeto es para archivos.
///   - El segundo es para carpetas.
pub fn path_file_oder_dir(path: &str) -> Result<(bool, bool)> {
    if !path_format_valid(path) {
        return Err(eyre!(
            "[path_file_oder_dir] - El archivo proporcionado no existe"
        ));
    }

    let p = path_from_string(path);
    Ok((p.is_file(), p.is_dir()))
}

/// Función para convertir ruta de relativa a absoluta.
///
/// return:
/// - None - En caso de la ruta no sea pueda resolver
/// - Some(Path) - Con la ruta convertida a absoluta
pub fn _path_relativ_2_absolute(path: &str) -> Result<String> {
    if !path_format_valid(path) {
        return Err(eyre!(
            "[path_relativ_2_absolute] - El archivo proporcionado no existe"
        ));
    }

    match canonicalize(path) {
        Ok(abs) => Ok(abs.display().to_string()),
        Err(e) => Err(eyre!(
            "[path_relativ_2_absolute] - ❌ No se pudo resolver: {}",
            e
        )),
    }
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
