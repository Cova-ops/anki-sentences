#[macro_export]
macro_rules! to_strings {
    ( $( $x:expr ),* $(,)? ) => {
        vec![ $( ::std::format!("{}", $x) ),* ]
    };
}

pub fn clean_sentences(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

pub trait StringBoolConvertion {
    fn to_bool(&self) -> bool;
}

impl<T> StringBoolConvertion for T
where
    T: AsRef<str>,
{
    fn to_bool(&self) -> bool {
        let s = self.as_ref().trim();
        let lower = s.to_ascii_lowercase();
        matches!(lower.as_str(), "si" | "sí" | "yes" | "1" | "true")
    }
}

pub fn string_to_bool(s: &str) -> bool {
    match s {
        "Si" | "si" | "yes" | "Yes" | "1" => true,
        _ => false,
    }
}
