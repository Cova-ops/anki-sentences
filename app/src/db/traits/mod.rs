use crate::helpers::error_handler::DbError;

// Models
pub(in crate::db) trait FromDTO<T> {
    type Error;

    fn from_raw<'a>(raw: &'a T) -> Result<Self, Self::Error>;
    fn from_vec_raw<'a, I>(vec: I) -> Result<Vec<Self>, Self::Error>
    where
        I: IntoIterator<Item = &'a T>;
}

// Schemas
pub(in crate::db) trait FromSql {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}
