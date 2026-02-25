use std::{
    collections::HashSet,
    fs::{self, File},
    path::PathBuf,
};

use crate::{
    db::schemas::wort_gender::EnumWortGender,
    helpers::error_handler::{AppError, AppErrorKind},
    services::{self, tts::eleven_labs::LanguageVoice},
};

pub mod audio_player;

pub enum AudioKind {
    Wort,
    Satz,
}

#[derive(Debug)]
pub struct ManageAudios {
    path_audios_worte: PathBuf,
    path_audios_setze: PathBuf,
    path_audios_artikel: PathBuf,
}

impl ManageAudios {
    pub fn new<S>(path_audios_worte: S, path_audios_setze: S, path_audios_artikel: S) -> Self
    where
        S: Into<PathBuf>,
    {
        Self {
            path_audios_setze: path_audios_setze.into(),
            path_audios_worte: path_audios_worte.into(),
            path_audios_artikel: path_audios_artikel.into(),
        }
    }

    pub fn check_audios_artikel(&self) -> Result<(), AppError> {
        let text = "der";
        let path = self.path_audios_artikel.join(format!("{text}.mp3"));
        if !path.exists() {
            let bytes = services::tts::eleven_labs::generate_tts(text, LanguageVoice::Deutsch)?;
            fs::write(&path, bytes)?;
        }

        let text = "die";
        let path = self.path_audios_artikel.join(format!("{text}.mp3"));
        if !path.exists() {
            let bytes = services::tts::eleven_labs::generate_tts(text, LanguageVoice::Deutsch)?;
            fs::write(&path, bytes)?;
        }

        let text = "das";
        let path = self.path_audios_artikel.join(format!("{text}.mp3"));
        if !path.exists() {
            let bytes = services::tts::eleven_labs::generate_tts(text, LanguageVoice::Deutsch)?;
            fs::write(&path, bytes)?;
        }

        Ok(())
    }

    pub fn get_audio_artikel(&self, gender: &EnumWortGender) -> Result<File, AppError> {
        let path = self
            .path_audios_artikel
            .join(format!("{}.mp3", gender.artikel()));

        let file = File::open(path)?;
        Ok(file)
    }

    pub fn save_audio_setze(
        &self,
        bytes: Vec<u8>,
        id: i32,
        lang: LanguageVoice,
    ) -> Result<PathBuf, AppError> {
        self.save_file(bytes, id, AudioKind::Satz, lang)
    }

    pub fn save_audio_worte(
        &self,
        bytes: Vec<u8>,
        id: i32,
        lang: LanguageVoice,
    ) -> Result<PathBuf, AppError> {
        self.save_file(bytes, id, AudioKind::Wort, lang)
    }

    fn save_file(
        &self,
        bytes: Vec<u8>,
        id: i32,
        type_file: AudioKind,
        lang: LanguageVoice,
    ) -> Result<PathBuf, AppError> {
        let path_final = match type_file {
            AudioKind::Wort => {
                self.path_audios_worte
                    .join(format!("wort_{:06}_{}.mp3", id, lang.get_posfix()))
            }
            AudioKind::Satz => {
                self.path_audios_setze
                    .join(format!("satz_{:06}_{}.mp3", id, lang.get_posfix()))
            }
        };

        fs::write(&path_final, bytes)?;

        Ok(path_final)
    }

    pub fn get_audio_setze(&self, id: i32, lang: LanguageVoice) -> Result<Option<File>, AppError> {
        self.get_file(id, AudioKind::Satz, lang)
    }

    pub fn get_audio_worte(&self, id: i32, lang: LanguageVoice) -> Result<Option<File>, AppError> {
        self.get_file(id, AudioKind::Wort, lang)
    }

    fn get_file(
        &self,
        id: i32,
        type_file: AudioKind,
        lang: LanguageVoice,
    ) -> Result<Option<File>, AppError> {
        let path = match type_file {
            AudioKind::Wort => {
                self.path_audios_worte
                    .join(format!("wort_{:06}_{}.mp3", id, lang.get_posfix()))
            }
            AudioKind::Satz => {
                self.path_audios_setze
                    .join(format!("satz_{:06}_{}.mp3", id, lang.get_posfix()))
            }
        };

        let file = File::open(path);
        match file {
            Ok(s) => Ok(Some(s)),
            _ => Ok(None),
        }
    }

    pub fn get_all_ids_files(&self) -> Result<(HashSet<i32>, HashSet<i32>), AppError> {
        let mut hash_worte: HashSet<i32> = HashSet::new();
        let mut hash_setze: HashSet<i32> = HashSet::new();

        let audios_dir = self.path_audios_worte.read_dir()?;
        for audio in audios_dir {
            let audio_name = audio?.file_name().to_str().map(|d| d.to_owned());
            let audio_name = audio_name.ok_or_else(|| AppError {
                kind: AppErrorKind::Internal(String::from("File with name not valid")),
                context: vec![],
            })?;

            if !audio_name.ends_with(".mp3") {
                println!("This file should be here: {}", audio_name);
                continue;
            }

            let audio_id_str = audio_name
                .strip_prefix("wort_")
                .and_then(|r| r.strip_suffix("_es.mp3"))
                .and_then(|r| r.strip_suffix("_de.mp3"));

            let audio_id_str = match audio_id_str {
                Some(v) if v.trim().len() > 0 => v,
                _ => {
                    println!("This file should not be here: wort/{audio_name}");
                    continue;
                }
            };

            let audio_id: i32 = match audio_id_str.parse() {
                Ok(v) => v,
                _ => {
                    println!("This file should not be here: wort/{audio_name}");
                    continue;
                }
            };

            hash_worte.insert(audio_id);
        }

        let audios_dir = self.path_audios_setze.read_dir()?;
        for audio in audios_dir {
            let audio_name = audio?.file_name().to_str().map(|d| d.to_owned());
            let audio_name = audio_name.ok_or_else(|| AppError {
                kind: AppErrorKind::Internal(String::from("File with name not valid")),
                context: vec![],
            })?;

            if !audio_name.ends_with(".mp3") {
                println!("This file should not be here: {}", audio_name);
                continue;
            }

            let audio_id_str = audio_name
                .strip_prefix("wort_")
                .and_then(|r| r.strip_suffix("_es.mp3"))
                .and_then(|r| r.strip_suffix("_de.mp3"));

            let audio_id_str = match audio_id_str {
                Some(v) if v.trim().len() > 0 => v,
                _ => {
                    println!("This file should not be here: wort/{audio_name}");
                    continue;
                }
            };

            let audio_id: i32 = match audio_id_str.parse() {
                Ok(v) => v,
                _ => {
                    println!("This file should not be here: wort/{audio_name}");
                    continue;
                }
            };

            hash_setze.insert(audio_id);
        }

        Ok((hash_worte, hash_setze))
    }

    pub fn remove_audios<I>(&self, ids_files: I, type_file: AudioKind) -> Result<(), AppError>
    where
        I: IntoIterator,
        I::Item: Copy + Into<i32>,
    {
        for id in ids_files {
            let id = id.into();

            let file_es = match type_file {
                AudioKind::Wort => format!("wort_{:06}_es.mp3", id),
                AudioKind::Satz => format!("satz_{:06}_es.mp3", id),
            };

            let file_de = match type_file {
                AudioKind::Wort => format!("wort_{:06}_de.mp3", id),
                AudioKind::Satz => format!("satz_{:06}_de.mp3", id),
            };

            let path_es = match type_file {
                AudioKind::Wort => self.path_audios_worte.join(&file_es),
                AudioKind::Satz => self.path_audios_setze.join(&file_es),
            };

            let path_de = match type_file {
                AudioKind::Wort => self.path_audios_worte.join(&file_de),
                AudioKind::Satz => self.path_audios_setze.join(&file_de),
            };

            match fs::remove_file(path_es) {
                Ok(_) => println!("File removed: {}", file_es),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
            };

            match fs::remove_file(path_de) {
                Ok(_) => println!("File removed: {}", file_de),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
            };
        }

        Ok(())
    }
}
