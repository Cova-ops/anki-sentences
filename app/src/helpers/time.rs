use chrono::{DateTime, NaiveDateTime, Utc};

#[inline]
pub fn string_2_datetime<T: AsRef<str>>(s: Option<T>) -> Option<DateTime<Utc>> {
    match s {
        Some(d) => {
            let created_at =
                NaiveDateTime::parse_from_str(d.as_ref(), "%Y-%m-%d %H:%M:%S").unwrap();
            Some(DateTime::<Utc>::from_naive_utc_and_offset(created_at, Utc))
        }
        None => None,
    }
}
