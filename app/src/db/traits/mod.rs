// Schemas
pub(in crate::db) trait FromSql {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}

// Input
pub trait SqlNew {
    // GAT: el tipo de parámetros depende del lifetime
    type Params<'a>: rusqlite::Params
    where
        Self: 'a;

    fn to_params<'a>(&'a self) -> Self::Params<'a>;
}
