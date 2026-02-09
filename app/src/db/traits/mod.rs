// Schemas
pub(in crate::db) trait FromSql {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}
