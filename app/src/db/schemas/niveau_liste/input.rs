use crate::db::schemas::niveau_liste::EnumNiveauListe;

#[derive(Debug)]
pub(in crate::db) struct SqlNiveauListe {
    pub id: i32,
    pub niveau: String,
}

#[derive(Debug, Clone)]
pub struct InputNiveauListe {
    pub niveau: EnumNiveauListe,
}

impl From<InputNiveauListe> for SqlNiveauListe {
    fn from(value: InputNiveauListe) -> Self {
        Self {
            id: value.niveau.id(),
            niveau: value.niveau.as_str().to_string(),
        }
    }
}

#[cfg(test)]
mod tests_sql_niveau_liste {
    use super::*;

    #[test]
    fn from_input_converts_a1_correctly() {
        let input = InputNiveauListe {
            niveau: EnumNiveauListe::A1,
        };

        let sql: SqlNiveauListe = input.into();

        assert_eq!(sql.id, 0);
        assert_eq!(sql.niveau, "A1".to_string());
    }

    #[test]
    fn from_input_converts_all_levels_correctly() {
        let cases = [
            (EnumNiveauListe::A1, 0, "A1"),
            (EnumNiveauListe::A2, 1, "A2"),
            (EnumNiveauListe::B1, 2, "B1"),
            (EnumNiveauListe::B2, 3, "B2"),
            (EnumNiveauListe::C1, 4, "C1"),
            (EnumNiveauListe::C2, 5, "C2"),
        ];

        for (niveau, id, s) in cases {
            let sql: SqlNiveauListe = InputNiveauListe { niveau }.into();
            assert_eq!(sql.id, id);
            assert_eq!(sql.niveau, s.to_string());
        }
    }
}
