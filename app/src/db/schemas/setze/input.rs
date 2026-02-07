use crate::db::schemas::niveau_liste::EnumNiveauListe;

#[derive(Debug)]
pub(in crate::db) struct SqlSetze {
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

#[cfg(test)]
mod tests_sql_setze_from_input {
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
