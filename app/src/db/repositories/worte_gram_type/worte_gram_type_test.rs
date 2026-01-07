#[cfg(test)]
mod test_worte_gram_type_repo {

    use color_eyre::eyre::Result;
    use rusqlite::Connection;

    use crate::test_utils::prelude::*;

    fn data_minimun(conn: &mut Connection) -> Result<()> {
        let sc = scenario_worte_gender();
        let data = WorteGenderRepo::bulk_insert(conn, &sc.initial)?;
        WorteGenderSchema::init_data(&data);

        let sc = scenario_gram_type();
        let data = GramTypeRepo::bulk_insert(conn, &sc.initial)?;
        GramTypeSchema::init_data(&data);

        let sc = scenario_niveau_liste();
        let data = NiveauListeRepo::bulk_insert(conn, &sc.initial)?;
        NiveauListeSchema::init_data(&data);

        let sc = scenario_worte();
        WorteRepo::bulk_insert(conn, &sc.initial)?;

        Ok(())
    }

    mod bulk_insert {
        use super::*;

        #[test]
        fn test_bulk_insert_and_update() -> Result<()> {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn)?;

            let sc = scenario_worte_gram_type();

            let res = WorteGramTypeRepo::bulk_insert(&mut conn, &sc.initial)?;
            res.assert_eq_fields(&sc.initial);

            insta::assert_debug_snapshot!(res.snapshot());

            Ok(())
        }
    }
}
