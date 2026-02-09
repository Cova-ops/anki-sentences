use crate::db::{schemas::gram_type::EnumGramType, traits::SqlNew};

#[derive(Debug, Clone)]
pub struct InputGramType {
    pub gram: EnumGramType,
}

#[derive(Debug, Clone)]
pub struct SqlGramType {
    pub id: i32,
    pub code: String,
    pub name: String,
}

impl From<InputGramType> for SqlGramType {
    fn from(value: InputGramType) -> Self {
        Self {
            id: value.gram.id(),
            code: value.gram.to_code().to_string(),
            name: value.gram.to_name().to_string(),
        }
    }
}

impl SqlNew for SqlGramType {
    type Params<'a>
        = (
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
    )
    where
        Self: 'a;

    fn to_params<'a>(&'a self) -> Self::Params<'a> {
        (&self.id, &self.code, &self.name)
    }
}

#[cfg(test)]
mod tests_sql_gram_type {
    use rusqlite::{Connection, params_from_iter};

    use super::*;

    #[test]
    fn from_input_gram_type_noun_common() {
        let input = InputGramType {
            gram: EnumGramType::NounCommon,
        };

        let sql: SqlGramType = input.into();

        assert_eq!(sql.id, 0);
        assert_eq!(sql.code, "noun_common");
        assert_eq!(sql.name, "Sustantivo común");
    }

    #[test]
    fn from_input_gram_type_verb_main() {
        let input = InputGramType {
            gram: EnumGramType::VerbMain,
        };

        let sql: SqlGramType = input.into();

        assert_eq!(sql.id, 2);
        assert_eq!(sql.code, "verb_main");
        assert_eq!(sql.name, "Verbo léxico");
    }

    #[test]
    fn to_params_binds_correctly_in_sqlite() -> rusqlite::Result<()> {
        let conn = Connection::open_in_memory()?;

        conn.execute_batch(
            r#"
            CREATE TABLE gram_type (
                id   INTEGER NOT NULL,
                code TEXT    NOT NULL,
                name TEXT    NOT NULL
            );
            "#,
        )?;

        let model = SqlGramType {
            id: 7,
            code: "verb_main".to_string(),
            name: "Verb (main)".to_string(),
        };

        // 1) Obtenemos params desde el trait
        let params = model.to_params();

        // 2) Insert usando esos params (como slice/iter)
        conn.execute(
            "INSERT INTO gram_type (id, code, name) VALUES (?1, ?2, ?3)",
            params_from_iter([params.0, params.1, params.2]),
        )?;

        // 3) Leemos de vuelta y comprobamos valores
        let (id, code, name): (i64, String, String) =
            conn.query_row("SELECT id, code, name FROM gram_type LIMIT 1", [], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?;

        assert_eq!(id, model.id as i64);
        assert_eq!(code, model.code);
        assert_eq!(name, model.name);

        Ok(())
    }

    #[test]
    fn to_params_has_expected_shape() {
        // Este test “fuerza” que el tipo sea un tuple de 3.
        // Si cambias el impl (por ejemplo a Vec), esto deja de compilar.

        fn assert_params_shape<'a>(_: <SqlGramType as SqlNew>::Params<'a>) {}

        let model = SqlGramType {
            id: 1,
            code: "a".into(),
            name: "b".into(),
        };

        let p = model.to_params();
        assert_params_shape(p);
    }
}
