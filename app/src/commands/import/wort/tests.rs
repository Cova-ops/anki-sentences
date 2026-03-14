#[cfg(test)]
mod tests {
    use crate::helpers::error_handler::AppError;

    mod manage_wort_repeated {
        use super::*;

        use crate::{
            commands::import::wort::{ManageWortRepeatedResponse, manage_wort_repeated},
            db::schemas::{
                gram_type::EnumGramType,
                niveau_liste::EnumNiveauListe,
                wort::{InputWort, ModelWort},
                wort_gender::EnumWortGender,
            },
        };

        #[test]
        fn returns_skip_when_old_and_new_are_equal() -> Result<(), AppError> {
            let old = ModelWort {
                id: 10,
                gender: Some(EnumWortGender::Maskuline),
                worte_de: "Haus".to_string(),
                worte_es: "casa".to_string(),
                plural: Some("Häuser".to_string()),
                niveau: EnumNiveauListe::A2,
                example_de: "Das Haus ist groß.".to_string(),
                example_es: "La casa es grande.".to_string(),
                verb_aux: None,
                trennbar: None,
                reflexiv: None,
                gram_type: vec![EnumGramType::NounCommon],
                created_at: chrono::Utc::now(),
                deleted_at: None,
            };

            let new = InputWort {
                gram_type: vec![EnumGramType::NounCommon],
                gender: Some(EnumWortGender::Maskuline),
                worte_de: "Haus".to_string(),
                worte_es: "casa".to_string(),
                plural: Some("Häuser".to_string()),
                niveau: EnumNiveauListe::A2,
                example_de: "Das Haus ist groß.".to_string(),
                example_es: "La casa es grande.".to_string(),
                verb_aux: None,
                trennbar: None,
                reflexiv: None,
            };

            let out = manage_wort_repeated(&old, &new)?;

            match out {
                ManageWortRepeatedResponse::Skip => {}
                other => panic!("Expected Skip, got {other:?}"),
            }

            Ok(())
        }
    }

    mod integration_test_run {
        use crate::{
            console::cli::TypeFile,
            db::{
                get_conn,
                schemas::{gram_type::EnumGramType, wort::ModelWort},
                wort::WortRepo,
            },
            helpers::toml::AppConfig,
        };

        use super::*;

        use std::io::Write;
        use tempfile::{Builder, NamedTempFile, TempDir, tempdir};

        fn make_temp_csv(content: &str) -> NamedTempFile {
            let mut file = Builder::new().suffix(".csv").tempfile().unwrap();

            write!(file, "{content}").unwrap();
            file.flush().unwrap();
            file
        }

        fn setup() -> Result<(TempDir, AppConfig), AppError> {
            let tmp = tempdir().unwrap();
            let config = AppConfig::new_test(&tmp);

            {
                let mut conn = get_conn(config.get_database_path()?)?;
                crate::db::init_db(&mut conn)?;
            }

            Ok((tmp, config))
        }

        #[test]
        fn happy_path() -> Result<(), AppError> {
            let (_tmp, config) = setup()?;

            let csv: NamedTempFile = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
             noun_common,der,Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n\
             verb_main,,lernen,aprender,,B1,Ich lerne Deutsch.,Aprendo alemán.,haben,true,false\n",
            );

            crate::commands::import::wort::run(
                &config,
                csv.path().to_str().unwrap(),
                TypeFile::CSV,
            )?;

            let conn = get_conn(config.get_database_path()?)?;

            let inserted = WortRepo::fetch_by_wort(
                &conn,
                &[
                    ("casa".to_string(), "Haus".to_string()),
                    ("aprender".to_string(), "lernen".to_string()),
                ],
            )?;

            assert_eq!(inserted.len(), 2);

            Ok(())
        }

        #[test]
        fn err_csv_malformed() -> Result<(), AppError> {
            let (_tmp, config) = setup()?;

            let csv = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
             noun_common,der,\"Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n",
            );

            let err = crate::commands::import::wort::run(
                &config,
                csv.path().to_str().unwrap(),
                TypeFile::CSV,
            )
            .unwrap_err();

            let msg = format!("{err:?}");
            assert!(
                msg.contains("Error on line") || msg.contains("CSV"),
                "Unexpected error: {msg}"
            );

            Ok(())
        }

        #[test]
        fn csv_columns_are_saved_in_expected_db_fields() -> Result<(), AppError> {
            let (_tmp, config) = setup()?;

            let csv = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
             verb_main,der,aufstehen,levantarse,,B1,Ich stehe um 7 Uhr auf.,Me levanto a las 7.,sein,true,false\n",
            );

            crate::commands::import::wort::run(
                &config,
                csv.path().to_str().unwrap(),
                TypeFile::CSV,
            )?;

            let conn = get_conn(config.get_database_path()?)?;

            let inserted: Vec<_> = WortRepo::fetch_by_wort(
                &conn,
                &[("levantarse".to_string(), "aufstehen".to_string())],
            )?;
            let mut inserted: Vec<ModelWort> = ModelWort::try_from_iter(inserted)?;

            assert_eq!(inserted.len(), 1);

            let wort = inserted.remove(0);

            assert_eq!(wort.gram_type, vec![EnumGramType::VerbMain]);
            assert_eq!(wort.worte_de, "aufstehen");
            assert_eq!(wort.worte_es, "levantarse");
            assert_eq!(wort.plural, None);
            assert_eq!(wort.example_de, "Ich stehe um 7 Uhr auf.");
            assert_eq!(wort.example_es, "Me levanto a las 7.");
            assert_eq!(wort.verb_aux.as_deref(), Some("sein"));
            assert_eq!(wort.trennbar, Some(true));
            assert_eq!(wort.reflexiv, Some(false));

            Ok(())
        }

        #[test]
        #[ignore = "It need to make it no_prompt"]
        fn updates_existing_word_and_replaces_gram_types() -> Result<(), AppError> {
            let (_tmp, config) = setup()?;

            let csv_initial = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
             noun_common,der,Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n",
            );

            crate::commands::import::wort::run(
                &config,
                csv_initial.path().to_str().unwrap(),
                TypeFile::CSV,
            )?;

            // second import changes gram_type and other fields -> this currently triggers interactive prompt
            let csv_updated = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
             fixed_phrase,,Haus,casa,,B1,Guten Morgen Haus.,Buenos días casa.,,,\n",
            );

            crate::commands::import::wort::run(
                &config,
                csv_updated.path().to_str().unwrap(),
                TypeFile::CSV,
            )?;

            let conn: rusqlite::Connection = get_conn(config.get_database_path()?)?;
            let inserted: Vec<_> =
                WortRepo::fetch_by_wort(&conn, &[("casa".to_string(), "Haus".to_string())])?;
            let inserted: Vec<ModelWort> = ModelWort::try_from_iter(inserted)?;

            assert_eq!(inserted.len(), 1);

            let wort = &inserted[0];
            assert_eq!(wort.gram_type, vec![EnumGramType::FixedPhrase]);
            assert_eq!(wort.niveau.as_str(), "B1");
            assert_eq!(wort.example_de, "Guten Morgen Haus.");
            assert_eq!(wort.example_es, "Buenos días casa.");

            Ok(())
        }

        #[test]
        fn duplicated_words_in_same_csv_should_only_take_first_one() -> Result<(), AppError> {
            let (_tmp, config) = setup()?;

            let csv = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
             noun_common,der,Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n\
             verb_main,,Haus,casa,,B1,Ich lerne Deutsch.,Aprendo alemán.,haben,true,false\n",
            );

            crate::commands::import::wort::run(
                &config,
                csv.path().to_str().unwrap(),
                TypeFile::CSV,
            )?;

            let conn = get_conn(config.get_database_path()?)?;
            let inserted: Vec<_> =
                WortRepo::fetch_by_wort(&conn, &[("casa".to_string(), "Haus".to_string())])?;
            let inserted: Vec<ModelWort> = ModelWort::try_from_iter(inserted)?;

            // Expected behavior requested: only first row should survive
            assert_eq!(inserted.len(), 1);

            let wort = &inserted[0];
            assert_eq!(wort.plural.as_deref(), Some("Häuser"));
            assert_eq!(wort.niveau.as_str(), "A2");
            assert_eq!(wort.example_de, "Das Haus ist groß.");
            assert_eq!(wort.example_es, "La casa es grande.");

            Ok(())
        }
    }
}
