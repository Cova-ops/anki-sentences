use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};

#[inline]
pub fn string_2_datetime<T: AsRef<str>>(s: Option<T>) -> Option<DateTime<Utc>> {
    match s {
        Some(d) => {
            // 1) Intentar formato SQLite: "YYYY-MM-DD HH:MM:SS"
            if let Ok(dt) = NaiveDateTime::parse_from_str(d.as_ref(), "%Y-%m-%d %H:%M:%S") {
                return Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
            }

            // 2) Formato con fracciones y zona: "2025-12-04 17:44:37.548062+00:00"
            if let Ok(dt) = DateTime::parse_from_str(d.as_ref(), "%Y-%m-%d %H:%M:%S%.f%:z") {
                return Some(dt.with_timezone(&Utc));
            }

            // 3) Formato RFC3339 si algún día lo usas: "2025-12-04T17:44:37.548062Z"
            if let Ok(dt) = DateTime::parse_from_rfc3339(d.as_ref()) {
                return Some(dt.with_timezone(&Utc));
            }
            panic!("Formato de fecha no soportado: {}", d.as_ref());
        }
        None => None,
    }
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
        .expect("fecha local ambigua / imposible");

    let target_utc = target_local.with_timezone(&Utc);
    target_utc.format("%Y-%m-%d %H:%M:%S").to_string()
}
