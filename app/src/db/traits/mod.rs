// Schemas
pub trait FromSql {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}

// Input
pub trait SqlInsert {
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql>;
}

pub trait SqlUpdate: SqlInsert {
    /// Add id to the final of the vec
    fn update_params<'a>(&'a self, id: &'a i32) -> Vec<&'a dyn rusqlite::ToSql> {
        let mut params = self.insert_params();
        params.push(id as &'a dyn rusqlite::ToSql);
        params
    }
}
