use chrono::{DateTime, Utc};

use crate::{db::traits::SqlNew, helpers::time::datetime_2_string};

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

impl SqlNew for SqlSetzeReview {
    type Params<'a>
        = (
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
    )
    where
        Self: 'a;

    /// This orden:
    /// - satz_id
    /// - interval
    /// - ease_factor
    /// - repetitions
    /// - last_review
    /// - next_review
    fn to_params<'a>(&'a self) -> Self::Params<'a> {
        (
            &self.satz_id,
            &self.interval,
            &self.ease_factor,
            &self.repetitions,
            &self.last_review,
            &self.next_review,
        )
    }
}

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

    mod sql_new {
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
        fn to_params_returns_values_in_expected_order() {
            let s = SqlSetzeReview {
                satz_id: 42,
                interval: 5,
                ease_factor: 2.5,
                repetitions: 3,
                last_review: String::from("2026-01-02 06:50:00"),
                next_review: String::from("2026-01-03 06:50:00"),
            };

            let (p1, p2, p3, p4, p5, p6) = s.to_params();

            assert_eq!(to_value(p1), Value::Integer(42));
            assert_eq!(to_value(p2), Value::Integer(5));
            assert_eq!(to_value(p3), Value::Real(2.5));
            assert_eq!(to_value(p4), Value::Integer(3));
            assert_eq!(
                to_value(p5),
                Value::Text(String::from("2026-01-02 06:50:00"))
            );
            assert_eq!(
                to_value(p6),
                Value::Text(String::from("2026-01-03 06:50:00"))
            );
        }
    }
}
