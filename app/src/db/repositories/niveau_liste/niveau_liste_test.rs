#[cfg(test)]
mod test_niveau_liste_repo {
    use color_eyre::eyre::Result;

    use crate::test_utils::prelude::*;

    mod bulk_upsert {

        use super::*;

        #[test]
        fn test_bulk_upsert() -> Result<()> {
            let mut conn = setup_test_db()?;

            let res = NiveauListeRepo::bulk_insert(&mut conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("[NiveauListe::bulk_upsert] - empty", res.snapshot());

            let sc = scenario_niveau_liste();

            let res = NiveauListeRepo::bulk_insert(&mut conn, &sc.initial)?;
            res.assert_eq_fields(&sc.initial);
            insta::assert_debug_snapshot!(
                "[NiveauListe::bulk_upsert] - after insert",
                res.snapshot()
            );

            let res = NiveauListeRepo::bulk_insert(&mut conn, &sc.update)?;
            res.assert_eq_fields(&sc.update);
            insta::assert_debug_snapshot!(
                "[NiveauListe::bulk_upsert] - after update",
                res.snapshot()
            );

            Ok(())
        }
    }
}
