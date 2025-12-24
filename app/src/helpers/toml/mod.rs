use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{OptionExt, Result, eyre};
use serde::{Deserialize, Serialize};

use crate::utils::{self, path::home_dir};

static DIR_CONFIG: &str = ".config/anki-sentences";
static DIR_PROFILES: &str = "profiles";

static DIR_ASSETS: &str = "assets";
static DIR_AUDIOS: &str = "audios";
static DIR_WORTE: &str = "worte";
static DIR_SETZE: &str = "setze";

static CONFIG_FILE_NAME: &str = "Config.toml";

// ~/.config/anki-sentences/
// ├── Config.toml
// ├── profiles/
// │   ├── default/
// │   │   ├── assets/
// │   │   │   └── audios/
// │   │   │       ├── setze/
// │   │   │       └── worte/
// │   │   └── default.sql
// │   └── user_1/
// │       ├── assets/
// │       │   └── audios/
// │       │       ├── setze/
// │       │       └── worte/
// │       └── user_1.sql

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileConfig {
    name_profile: String,
    audio_enabled: bool,
    database_path: PathBuf,
    audios_worte_path: PathBuf,
    audios_setze_path: PathBuf,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self::new("Default")
    }
}

impl ProfileConfig {
    fn validate_dirs(&self) -> Result<()> {
        if !self.database_path.try_exists()? {
            fs::create_dir_all(&self.database_path)?;
        }

        if !self.audios_setze_path.try_exists()? {
            fs::create_dir_all(&self.audios_setze_path)?;
        }

        if !self.audios_worte_path.try_exists()? {
            fs::create_dir_all(&self.audios_worte_path)?;
        }

        Ok(())
    }

    pub fn new(name_profile: &str) -> Self {
        let lower = name_profile.to_owned().to_lowercase();

        let profile_path = utils::path::home_dir()
            .join(DIR_CONFIG)
            .join(DIR_PROFILES)
            .join(&lower);

        let database_path = profile_path.join(format!("{lower}.sql"));
        let path_audios = profile_path.join(DIR_ASSETS).join(DIR_AUDIOS);

        Self {
            name_profile: name_profile.into(),
            database_path,
            audios_worte_path: path_audios.join(DIR_WORTE),
            audios_setze_path: path_audios.join(DIR_SETZE),
            audio_enabled: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    profiles: HashMap<String, ProfileConfig>,
    actual_profile: String,

    #[serde(skip)]
    config_file_path: Option<PathBuf>,
}

impl AppConfig {
    pub fn new(config_file_path: &Path) -> Self {
        Self {
            profiles: HashMap::from([("Default".into(), ProfileConfig::default())]),
            actual_profile: "Default".into(),
            config_file_path: Some(config_file_path.to_owned()),
        }
    }

    pub fn load_config() -> Result<Self> {
        let folder = home_dir().join(DIR_CONFIG);
        if !folder.try_exists()? {
            fs::create_dir_all(folder.clone())?;
        }

        let folder_profiles = folder.join(DIR_PROFILES);
        if !folder_profiles.try_exists()? {
            fs::create_dir_all(folder_profiles.clone())?;
        }

        let file_path = folder.join(CONFIG_FILE_NAME);
        if !file_path.try_exists()? {
            let new_config = AppConfig::new(&file_path);
            new_config.save_config()?;

            // Create all the dirs
            new_config.get_actual_profile()?.validate_dirs()?;

            return Ok(new_config);
        }

        let content = fs::read_to_string(&file_path)?;
        let mut cfg: AppConfig = toml::from_str(&content)?;
        cfg.config_file_path = Some(file_path);

        // valid if profile is a exists and create dirs
        cfg.get_actual_profile()?.validate_dirs()?;

        Ok(cfg)
    }

    pub fn save_config(&self) -> Result<()> {
        let file_path = self
            .config_file_path
            .as_deref()
            .ok_or_else(|| eyre!("Config file not configurated yet"))?;

        let content = toml::to_string_pretty(self)?;
        fs::write(&file_path, content)?;
        Ok(())
    }

    fn get_profile(&self, profile: &str) -> Result<&ProfileConfig> {
        self.profiles.get(profile).ok_or_eyre(format!(
            r#"Profile {profile} not founded. Try with "anki-sentences db use <name_profile>""#
        ))
    }

    fn get_actual_profile(&self) -> Result<&ProfileConfig> {
        self.get_profile(&self.actual_profile)
    }

    fn get_profile_mut(&mut self, profile: &str) -> Result<&mut ProfileConfig> {
        self.profiles.get_mut(profile).ok_or_eyre(format!(
            r#"Profile {profile} not founded. Try with "anki-sentences db use <name_profile>""#
        ))
    }

    fn get_actual_profile_mut(&mut self) -> Result<&mut ProfileConfig> {
        self.get_profile_mut(&self.actual_profile.clone())
    }

    pub fn change_profile(&mut self, name_profile: &str) -> Result<()> {
        self.get_profile(name_profile)?; // valid if this exists
        self.actual_profile = name_profile.to_string();
        self.get_actual_profile()?.validate_dirs()?;

        self.save_config()?;
        Ok(())
    }

    pub fn set_new_profile(&mut self, name_profile: &str) -> Result<()> {
        let new_profile = ProfileConfig::new(name_profile);
        new_profile.validate_dirs()?;

        self.profiles.insert(name_profile.to_owned(), new_profile);
        self.change_profile(name_profile)?;

        self.save_config()?;
        Ok(())
    }

    // INFORMATION FROM ACTUAL PROFILE
    pub fn get_database_path(&self) -> Result<&Path> {
        Ok(&self.get_actual_profile()?.database_path)
    }

    pub fn is_audio_enable(&self) -> Result<bool> {
        Ok(self.get_actual_profile()?.audio_enabled)
    }

    pub fn set_audio_enable(&mut self, new_value: bool) -> Result<()> {
        self.get_actual_profile_mut()?.audio_enabled = new_value;
        self.save_config()?;

        Ok(())
    }

    pub fn get_path_audios_worte(&self) -> Result<&Path> {
        Ok(&self.get_actual_profile()?.audios_worte_path)
    }

    pub fn get_path_audios_setze(&self) -> Result<&Path> {
        Ok(&self.get_actual_profile()?.audios_setze_path)
    }
}
