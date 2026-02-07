use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};

use crate::helpers::error_handler::InvalidValueError;

#[inline]
pub fn string_2_datetime<T: AsRef<str>>(s: T) -> Result<DateTime<Utc>, InvalidValueError> {
    // 1) Intentar formato SQLite: "YYYY-MM-DD HH:MM:SS"
    if let Ok(dt) = NaiveDateTime::parse_from_str(s.as_ref(), "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
    }

    // 2) Formato con fracciones y zona: "2025-12-04 17:44:37.548062+00:00"
    if let Ok(dt) = DateTime::parse_from_str(s.as_ref(), "%Y-%m-%d %H:%M:%S%.f%:z") {
        return Ok(dt.with_timezone(&Utc));
    }

    // 3) Formato RFC3339 si algún día lo usas: "2025-12-04T17:44:37.548062Z"
    if let Ok(dt) = DateTime::parse_from_rfc3339(s.as_ref()) {
        return Ok(dt.with_timezone(&Utc));
    }

    Err(InvalidValueError {
        field: "datetime",
        message: format!("Invalid datetime format: '{}'", s.as_ref()),
        valid_options: Some(vec![
            "YYYY-MM-DD HH:MM:SS",
            "YYYY-MM-DD HH:MM:SS.ssssss+00:00",
            "RFC3339 (YYYY-MM-DDTHH:MM:SSZ)",
        ]),
    })
}

#[inline]
pub fn fixed_date(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> DateTime<Utc> {
    let naive = NaiveDate::from_ymd_opt(y, m, d)
        .unwrap()
        .and_hms_opt(h, min, s)
        .unwrap();

    DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)
}

#[inline]
pub fn datetime_2_string(dt: DateTime<Utc>) -> String {
    dt.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[inline]
pub fn today_local_string(offset: i64) -> String {
    let today_local = Local::now().date_naive();
    let today_local_naive: NaiveDateTime = (today_local + Duration::days(offset))
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let target_local = Local
        .from_local_datetime(&today_local_naive)
        .single()
        .expect("local date not defined / impossible");

    let target_utc = target_local.with_timezone(&Utc);
    target_utc.format("%Y-%m-%d %H:%M:%S").to_string()
}
