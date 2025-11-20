use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{Params, Row};

pub type ModelResult<T> = color_eyre::Result<T>;

/// Para structs "NewXxx", usados en INSERT/UPDATE
pub trait SqlNew {
    // GAT: el tipo de parámetros depende del lifetime
    type Params<'a>: Params
    where
        Self: 'a;

    fn to_params<'a>(&'a self) -> Self::Params<'a>;
}

/// Para structs "RawXxx", que vienen directo de rusqlite::Row
pub trait SqlRaw: Sized {
    fn from_sql(r: &Row<'_>) -> rusqlite::Result<Self>;
}

/// Para structs "Schema", que representan el modelo de dominio
pub trait FromRaw<R>: Sized {
    fn from_raw(r: R) -> ModelResult<Self>;

    fn from_vec_raw(data: Vec<R>) -> ModelResult<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}

/// Conversión estándar de TEXT (SQLite) -> DateTime<Utc>
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

/// Re-export del derive
pub use sql_model_derive::SqlModel;
