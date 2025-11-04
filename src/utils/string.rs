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
