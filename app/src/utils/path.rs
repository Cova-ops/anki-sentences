use std::{
    env,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{self, OptionExt, Result};

pub fn validate_save_filename<P, S>(name: P, ext: &[S]) -> Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let name = name.as_ref();

    let file_name = name
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| eyre::eyre!("File's name invalid"))?;

    if name != Path::new(file_name) {
        eyre::bail!("Only accept the name of the file, not paths");
    }

    if !file_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        eyre::bail!("File's name invalid")
    }

    if !ext.iter().any(|x| file_name.ends_with(x.as_ref())) {
        eyre::bail!("File's format no valid")
    }

    Ok(())
}

pub fn path_to_string(path: &Path) -> Result<String> {
    path.to_str()
        .map(|x| x.to_owned())
        .ok_or_eyre("Error getting string from path")
}

pub fn get_filename_from_path(path: &Path) -> Result<String> {
    let filename = path
        .file_name()
        .and_then(|x| x.to_str())
        .ok_or_eyre("Error converting OsStr to &str")?
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
