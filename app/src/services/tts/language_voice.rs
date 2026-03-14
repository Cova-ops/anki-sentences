#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVoice {
    Deutsch,
    Spanisch,
}

impl LanguageVoice {
    pub fn get_posfix(&self) -> String {
        match self {
            LanguageVoice::Spanisch => "es".to_owned(),
            LanguageVoice::Deutsch => "de".to_owned(),
        }
    }
}
