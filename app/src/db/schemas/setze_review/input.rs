use chrono::{DateTime, Utc};

use crate::{
    db::traits::{SqlInsert, SqlUpdate},
    helpers::time::datetime_2_string,
};

#[derive(Debug)]
pub struct SqlSetzeReview {
    pub satz_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String, // DateTime<Utc>
    pub next_review: String, // DateTime<Utc>
}

#[derive(Debug, Clone)]
pub struct InputSetzeReview {
    pub satz_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,
}

impl From<InputSetzeReview> for SqlSetzeReview {
    fn from(value: InputSetzeReview) -> Self {
        Self {
            satz_id: value.satz_id,
            interval: value.interval,
            ease_factor: value.ease_factor,
            repetitions: value.repetitions,
            last_review: datetime_2_string(value.last_review),
            next_review: datetime_2_string(value.next_review),
        }
    }
}

impl SqlInsert for SqlSetzeReview {
    /// This orden:
    /// - satz_id
    /// - interval
    /// - ease_factor
    /// - repetitions
    /// - last_review
    /// - next_review
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![
            &self.satz_id,
            &self.interval,
            &self.ease_factor,
            &self.repetitions,
            &self.last_review,
            &self.next_review,
        ]
    }
}

impl SqlUpdate for SqlSetzeReview {}

#[cfg(test)]
mod tests_sql_setze_review {
    use super::*;
    use chrono::TimeZone;

    mod from_input {
        use super::*;

        #[test]
        fn from_input_converts_all_fields_correctly() {
            let last_review = Utc.with_ymd_and_hms(2025, 1, 10, 12, 30, 0).unwrap();
            let next_review = Utc.with_ymd_and_hms(2025, 1, 15, 8, 0, 0).unwrap();

            let input = InputSetzeReview {
                satz_id: 42,
                interval: 5,
                ease_factor: 2.5,
                repetitions: 3,
                last_review,
                next_review,
            };

            let sql: SqlSetzeReview = input.into();

            assert_eq!(sql.satz_id, 42);
            assert_eq!(sql.interval, 5);
            assert_eq!(sql.ease_factor, 2.5);
            assert_eq!(sql.repetitions, 3);

            // Fechas convertidas correctamente a String
            assert_eq!(sql.last_review, datetime_2_string(last_review));
            assert_eq!(sql.next_review, datetime_2_string(next_review));
        }
    }

    mod sql_params {
        use super::*;

        use rusqlite::ToSql;
        use rusqlite::types::{ToSqlOutput, Value, ValueRef};

        fn to_value(p: &dyn ToSql) -> Value {
            match p.to_sql().expect("to_sql should work") {
                ToSqlOutput::Owned(v) => v,
                ToSqlOutput::Borrowed(vr) => match vr {
                    ValueRef::Null => Value::Null,
                    ValueRef::Integer(i) => Value::Integer(i),
                    ValueRef::Real(f) => Value::Real(f),
                    ValueRef::Text(t) => Value::Text(String::from_utf8_lossy(t).into_owned()),
                    ValueRef::Blob(b) => Value::Blob(b.to_vec()),
                },
                _ => panic!(""),
            }
        }

        #[test]
        fn insert_params() {
            let s = SqlSetzeReview {
                satz_id: 42,
                interval: 5,
                ease_factor: 2.5,
                repetitions: 3,
                last_review: String::from("2026-01-02 06:50:00"),
                next_review: String::from("2026-01-03 06:50:00"),
            };

            let params = s.insert_params();

            assert_eq!(to_value(params[0]), Value::Integer(42));
            assert_eq!(to_value(params[1]), Value::Integer(5));
            assert_eq!(to_value(params[2]), Value::Real(2.5));
            assert_eq!(to_value(params[3]), Value::Integer(3));
            assert_eq!(
                to_value(params[4]),
                Value::Text(String::from("2026-01-02 06:50:00"))
            );
            assert_eq!(
                to_value(params[5]),
                Value::Text(String::from("2026-01-03 06:50:00"))
            );
        }

        #[test]
        fn update_params() {
            let s = SqlSetzeReview {
                satz_id: 42,
                interval: 5,
                ease_factor: 2.5,
                repetitions: 3,
                last_review: String::from("2026-01-02 06:50:00"),
                next_review: String::from("2026-01-03 06:50:00"),
            };

            let params = s.update_params(&99);

            assert_eq!(to_value(params[0]), Value::Integer(42));
            assert_eq!(to_value(params[1]), Value::Integer(5));
            assert_eq!(to_value(params[2]), Value::Real(2.5));
            assert_eq!(to_value(params[3]), Value::Integer(3));
            assert_eq!(
                to_value(params[4]),
                Value::Text(String::from("2026-01-02 06:50:00"))
            );
            assert_eq!(
                to_value(params[5]),
                Value::Text(String::from("2026-01-03 06:50:00"))
            );
            assert_eq!(to_value(params[6]), Value::Integer(99));
        }
    }
}
