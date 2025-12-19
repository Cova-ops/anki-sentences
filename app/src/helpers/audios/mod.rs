use std::{
    fs::{self, File},
    path::Path,
};

use color_eyre::eyre::Result;

pub mod audio_player;

const PATH_FOLDER: &str = "assets/audios";
const PATH_AUDIOS_WORTE: &str = "assets/audios/worte";
const PATH_AUDIOS_SETZE: &str = "assets/audios/setze";

enum TypeFile {
    AudioWort,
    AudioSatz,
}

pub struct ManageAudios {}

impl ManageAudios {
    pub fn init_dir() -> Result<()> {
        let folder_path = Path::new(PATH_FOLDER);
        if !folder_path.exists() {
            fs::create_dir_all(folder_path)?;
        }

        let worte_path = Path::new(PATH_AUDIOS_WORTE);
        if !worte_path.exists() {
            fs::create_dir_all(worte_path)?;
        }

        let setze_path = Path::new(PATH_AUDIOS_SETZE);
        if !setze_path.exists() {
            fs::create_dir_all(setze_path)?;
        }

        Ok(())
    }

    pub fn save_audio_setze(bytes: Vec<u8>, id: i32) -> Result<String> {
        Self::save_file(bytes, id, TypeFile::AudioSatz)
    }

    pub fn save_audio_worte(bytes: Vec<u8>, id: i32) -> Result<String> {
        Self::save_file(bytes, id, TypeFile::AudioWort)
    }

    fn save_file(bytes: Vec<u8>, id: i32, type_file: TypeFile) -> Result<String> {
        let path_final = match type_file {
            TypeFile::AudioWort => format!("{}/wort_{:06}.mp3", PATH_AUDIOS_WORTE, id),
            TypeFile::AudioSatz => format!("{}/satz_{:06}.mp3", PATH_AUDIOS_SETZE, id),
        };

        fs::write(&path_final, bytes)?;

        Ok(path_final)
    }

    pub fn get_audio_setze(id: i32) -> Result<Option<File>> {
        Self::get_file(id, TypeFile::AudioSatz)
    }
    pub fn get_audio_worte(id: i32) -> Result<Option<File>> {
        Self::get_file(id, TypeFile::AudioWort)
    }

    fn get_file(id: i32, type_file: TypeFile) -> Result<Option<File>> {
        let path = match type_file {
            TypeFile::AudioWort => format!("{}/wort_{:06}.mp3", PATH_AUDIOS_WORTE, id),
            TypeFile::AudioSatz => format!("{}/wort_{:06}.mp3", PATH_AUDIOS_SETZE, id),
        };

        let file = File::open(path);
        match file {
            Ok(s) => Ok(Some(s)),
            _ => Ok(None),
        }
    }
}
