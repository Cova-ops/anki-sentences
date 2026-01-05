use color_eyre::eyre::{Result, bail};

#[inline]
pub fn clean_sentences(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

pub fn validate_profile_name(name: &str) -> Result<()> {
    let ok_len = (1..=32).contains(&name.len());
    let ok_chars = name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');

    if !ok_len || !ok_chars {
        bail!("Profile name not valid. Only allow [a-zA-Z0-9_] and max length 32");
    }

    Ok(())
}
