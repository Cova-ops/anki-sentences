use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{OptionExt, Result, eyre};
use serde::{Deserialize, Serialize};

use crate::utils::{self, home_dir};

static DIR_CONFIG: &str = ".config/anki-sentences";
static DIR_DB: &str = "db";

static CONFIG_FILE_NAME: &str = "Config.toml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileConfig {
    name_profile: String,
    database_path: PathBuf,
    audio_enabled: bool,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self::new("Default")
    }
}

impl ProfileConfig {
    pub fn new(name_profile: &str) -> Self {
        let lower = name_profile.to_owned().to_lowercase();
        Self {
            name_profile: name_profile.into(),
            database_path: utils::home_dir()
                .join(DIR_CONFIG)
                .join(DIR_DB)
                .join(format!("{lower}.sql")),
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

        let folder_db = folder.join(DIR_DB);
        if !folder_db.try_exists()? {
            fs::create_dir_all(folder_db.clone())?;
        }

        let file_path = folder.join(CONFIG_FILE_NAME);
        if !file_path.try_exists()? {
            let new_config = AppConfig::new(&file_path);
            new_config.save_config()?;
            return Ok(new_config);
        }

        let content = fs::read_to_string(&file_path)?;
        let mut cfg: AppConfig = toml::from_str(&content)?;
        cfg.config_file_path = Some(file_path);

        cfg.get_actual_profile()?; // valid if profile is a exists

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

    pub fn change_profile(&mut self, name_profile: &str) -> Result<()> {
        self.get_profile(name_profile)?; // valid if this exists
        self.actual_profile = name_profile.to_string();
        self.save_config()?;

        Ok(())
    }

    pub fn set_new_profile(&mut self, name_profile: &str) -> Result<()> {
        let new_profile = ProfileConfig::new(name_profile);
        self.profiles.insert(name_profile.to_string(), new_profile);
        self.change_profile(name_profile)?;
        self.save_config()?;

        Ok(())
    }
}

// TODO: Arreglar el tema del path de db, que cabio de String a Pathbuf
