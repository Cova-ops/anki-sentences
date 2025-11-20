pub trait StringConvertion {
    fn to_bool(&self) -> bool;
}

impl<T> StringConvertion for T
where
    T: AsRef<str>,
{
    fn to_bool(&self) -> bool {
        let s = self.as_ref().trim();
        let lower = s.to_ascii_lowercase();
        matches!(lower.as_str(), "si" | "sí" | "yes" | "1" | "true")
    }
}
