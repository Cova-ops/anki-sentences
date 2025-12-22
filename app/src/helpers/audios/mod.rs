use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;

pub mod audio_player;

enum TypeFile {
    AudioWort,
    AudioSatz,
}

#[derive(Debug)]
pub struct ManageAudios {
    path_audios_worte: PathBuf,
    path_audios_setze: PathBuf,
}

impl ManageAudios {
    pub fn new<S>(path_audios_worte: S, path_audios_setze: S) -> Self
    where
        S: Into<PathBuf>,
    {
        Self {
            path_audios_setze: path_audios_setze.into(),
            path_audios_worte: path_audios_worte.into(),
        }
    }

    pub fn save_audio_setze(&self, bytes: Vec<u8>, id: i32) -> Result<PathBuf> {
        self.save_file(bytes, id, TypeFile::AudioSatz)
    }

    pub fn save_audio_worte(&self, bytes: Vec<u8>, id: i32) -> Result<PathBuf> {
        self.save_file(bytes, id, TypeFile::AudioWort)
    }

    fn save_file(&self, bytes: Vec<u8>, id: i32, type_file: TypeFile) -> Result<PathBuf> {
        let path_final = match type_file {
            TypeFile::AudioWort => self.path_audios_worte.join(format!("wort_{:06}.mp3", id)),
            TypeFile::AudioSatz => self.path_audios_setze.join(format!("satz_{:06}.mp3", id)),
        };

        fs::write(&path_final, bytes)?;

        Ok(path_final)
    }

    pub fn get_audio_setze(&self, id: i32) -> Result<Option<File>> {
        self.get_file(id, TypeFile::AudioSatz)
    }
    pub fn get_audio_worte(&self, id: i32) -> Result<Option<File>> {
        self.get_file(id, TypeFile::AudioWort)
    }

    fn get_file(&self, id: i32, type_file: TypeFile) -> Result<Option<File>> {
        let path = match type_file {
            TypeFile::AudioWort => self.path_audios_worte.join(format!("wort_{:06}.mp3", id)),
            TypeFile::AudioSatz => self.path_audios_setze.join(format!("satz_{:06}.mp3", id)),
        };

        let file = File::open(path);
        match file {
            Ok(s) => Ok(Some(s)),
            _ => Ok(None),
        }
    }
}
