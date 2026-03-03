use chrono::{DateTime, Utc};

use crate::{
    db::{
        schemas::wort_review::EnumReviewDirection,
        traits::{SqlInsert, SqlUpdate},
    },
    helpers::time::datetime_2_string,
};

#[derive(Debug)]
pub struct SqlWortReview {
    pub wort_id: i32,
    pub direction: String,
    pub interval: u32,
    pub ease_factor: f64,
    pub repetitions: u32,
    pub last_review: String,
    pub next_review: String,
}

#[derive(Debug, Clone)]
pub struct InputWortReview {
    pub wort_id: i32,
    pub direction: EnumReviewDirection,
    pub interval: u32,
    pub ease_factor: f64,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,
}

impl From<InputWortReview> for SqlWortReview {
    fn from(value: InputWortReview) -> Self {
        Self {
            wort_id: value.wort_id,
            direction: value.direction.as_str().to_string(),
            interval: value.interval,
            ease_factor: value.ease_factor,
            repetitions: value.repetitions,
            last_review: datetime_2_string(value.last_review),
            next_review: datetime_2_string(value.next_review),
        }
    }
}

impl SqlInsert for SqlWortReview {
    /// This orden:
    /// - wort_id
    /// - direction
    /// - interval
    /// - ease_factor
    /// - repetitions
    /// - last_review
    /// - next_review
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![
            &self.wort_id,
            &self.direction,
            &self.interval,
            &self.ease_factor,
            &self.repetitions,
            &self.last_review,
            &self.next_review,
        ]
    }
}

impl SqlUpdate for SqlWortReview {}

#[cfg(test)]
mod tests_sql_wort_review {
    use super::*;
    use chrono::TimeZone;

    mod from_input {
        use super::*;

        fn dt1() -> DateTime<Utc> {
            Utc.with_ymd_and_hms(2025, 12, 4, 17, 44, 37).unwrap()
        }
        fn dt2() -> DateTime<Utc> {
            Utc.with_ymd_and_hms(2026, 1, 10, 8, 0, 1).unwrap()
        }

        #[test]
        fn convertion_1() {
            let input = InputWortReview {
                wort_id: 123,
                direction: EnumReviewDirection::ES2DE,
                interval: 7,
                ease_factor: 2.45,
                repetitions: 3,
                last_review: dt1(),
                next_review: dt2(),
            };

            let sql: SqlWortReview = input.into();

            assert_eq!(sql.wort_id, 123);
            assert_eq!(
                sql.direction,
                EnumReviewDirection::ES2DE.as_str().to_string()
            );
            assert_eq!(sql.interval, 7);
            assert!((sql.ease_factor - 2.45).abs() < f64::EPSILON);
            assert_eq!(sql.repetitions, 3);

            assert_eq!(sql.last_review, datetime_2_string(dt1()));
            assert_eq!(sql.next_review, datetime_2_string(dt2()));
        }

        #[test]
        fn convertion_2() {
            let input = InputWortReview {
                wort_id: 1,
                direction: EnumReviewDirection::DE2ES,
                interval: 1,
                ease_factor: 1.3,
                repetitions: 0,
                last_review: dt1(),
                next_review: dt2(),
            };

            let sql: SqlWortReview = input.into();

            assert_eq!(sql.direction, EnumReviewDirection::DE2ES.as_str());
        }
    }

    mod sql_params {
        use super::*;
        use rusqlite::{
            ToSql,
            types::{ToSqlOutput, Value, ValueRef},
        };

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
            let s = SqlWortReview {
                wort_id: 1,
                direction: String::from("es_2_de"),
                interval: 2,
                ease_factor: 1.2,
                repetitions: 10,
                last_review: String::from("2018-04-01 20:00:00"),
                next_review: String::from("2018-05-01 20:00:00"),
            };

            let params = s.insert_params();

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Text("es_2_de".to_string()));
            assert_eq!(to_value(params[2]), Value::Integer(2));
            assert_eq!(to_value(params[3]), Value::Real(1.2));
            assert_eq!(to_value(params[4]), Value::Integer(10));
            assert_eq!(
                to_value(params[5]),
                Value::Text(String::from("2018-04-01 20:00:00"))
            );
            assert_eq!(
                to_value(params[6]),
                Value::Text(String::from("2018-05-01 20:00:00"))
            );
        }

        #[test]
        fn update_params() {
            let s = SqlWortReview {
                wort_id: 1,
                direction: String::from("es_2_de"),
                interval: 2,
                ease_factor: 1.2,
                repetitions: 10,
                last_review: String::from("2018-04-01 20:00:00"),
                next_review: String::from("2018-05-01 20:00:00"),
            };

            let params = s.update_params(&99);

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Text("es_2_de".to_string()));
            assert_eq!(to_value(params[2]), Value::Integer(2));
            assert_eq!(to_value(params[3]), Value::Real(1.2));
            assert_eq!(to_value(params[4]), Value::Integer(10));
            assert_eq!(
                to_value(params[5]),
                Value::Text(String::from("2018-04-01 20:00:00"))
            );
            assert_eq!(
                to_value(params[6]),
                Value::Text(String::from("2018-05-01 20:00:00"))
            );
            assert_eq!(to_value(params[7]), Value::Integer(99));
        }
    }
}
