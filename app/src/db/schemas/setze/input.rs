use crate::db::{schemas::niveau_liste::EnumNiveauListe, traits::SqlNew};

#[derive(Debug)]
pub struct SqlSetze {
    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub niveau_id: i32,
    pub thema: String,
}

#[derive(Debug, Clone)]
pub struct InputSetze {
    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub niveau: EnumNiveauListe,
    pub thema: String,
}

impl From<InputSetze> for SqlSetze {
    fn from(value: InputSetze) -> Self {
        Self {
            setze_spanisch: value.setze_spanisch,
            setze_deutsch: value.setze_deutsch,
            niveau_id: value.niveau.id(),
            thema: value.thema,
        }
    }
}

impl SqlNew for SqlSetze {
    type Params<'a>
        = (
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
    )
    where
        Self: 'a;

    /// This orden:
    /// - setze_spanisch
    /// - setze_deutsch
    /// - niveau_id
    /// - thema
    fn to_params<'a>(&'a self) -> Self::Params<'a> {
        (
            &self.setze_spanisch,
            &self.setze_deutsch,
            &self.niveau_id,
            &self.thema,
        )
    }
}

#[cfg(test)]
mod tests_sql_setze_from_input {
    use rusqlite::ToSql;
    use rusqlite::types::{ToSqlOutput, Value, ValueRef};

    mod from_input_setze {
        use super::*;

        #[test]
        fn input_setze_into_sql_setze_maps_fields_correctly() {
            let input = InputSetze {
                setze_spanisch: "Estoy aprendiendo alemán.".to_string(),
                setze_deutsch: "Ich lerne Deutsch.".to_string(),
                niveau: EnumNiveauListe::A2,
                thema: "lernen".to_string(),
            };

            let sql: SqlSetze = input.into();

            assert_eq!(sql.setze_spanisch, "Estoy aprendiendo alemán.");
            assert_eq!(sql.setze_deutsch, "Ich lerne Deutsch.");
            assert_eq!(sql.niveau_id, EnumNiveauListe::A2.id());
            assert_eq!(sql.thema, "lernen");
        }
    }

    mod sql_new {
        use super::*;

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
            let s = SqlSetze {
                setze_spanisch: "Yo corro diario".to_string(),
                setze_deutsch: "Ich laufe jeden Tag".to_string(),
                niveau_id: 2, // ajusta si tu tipo es i64/u32/etc
                thema: "Sport".to_string(),
                // si tu struct tiene más campos, agrégalos aquí
            };

            let (p1, p2, p3, p4) = s.to_params();

            assert_eq!(to_value(p1), Value::Text("Yo corro diario".to_string()));
            assert_eq!(to_value(p2), Value::Text("Ich laufe jeden Tag".to_string()));
            assert_eq!(to_value(p3), Value::Integer(2));
            assert_eq!(to_value(p4), Value::Text("Sport".to_string()));
        }
    }
}
