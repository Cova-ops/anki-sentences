#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    use crate::AppError;

    fn make_temp_csv(content: &str) -> Result<NamedTempFile, Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        write!(file, "{content}")?;
        file.flush()?;
        Ok(file)
    }

    mod is_csv_valid {
        use crate::helpers::{
            csv::{EnumCsvType, is_csv_valid},
            error_handler::AppErrorKind,
        };

        use super::*;

        #[test]
        fn ok_setze_csv() -> Result<(), AppError> {
            let file = make_temp_csv(
                "setze_spanisch,setze_deutsch,thema,schwirig_id\n\
                 Hola mundo,Hallo Welt,greetings,1\n\
                 Estoy aprendiendo alemán.,Ich lerne Deutsch.,learning,2\n",
            )
            .unwrap();

            let out = is_csv_valid(file.path(), EnumCsvType::Setze)?;

            assert_eq!(out.len(), 2);
            assert_eq!(out[0].get(0), Some("Hola mundo"));
            assert_eq!(out[0].get(1), Some("Hallo Welt"));
            assert_eq!(out[0].get(2), Some("greetings"));
            assert_eq!(out[0].get(3), Some("1"));

            assert_eq!(out[1].get(0), Some("Estoy aprendiendo alemán."));
            assert_eq!(out[1].get(1), Some("Ich lerne Deutsch."));
            assert_eq!(out[1].get(2), Some("learning"));
            assert_eq!(out[1].get(3), Some("2"));

            Ok(())
        }

        #[test]
        fn ok_worte_csv() -> Result<(), AppError> {
            let file = make_temp_csv(
        "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 noun_common,1,Haus,casa,Häuser,2,Das Haus ist groß.,La casa es grande.,,,\n\
                 verb_main,,lernen,aprender,,2,Ich lerne Deutsch.,Aprendo alemán.,haben,,\n\
                 verb_main,,gehen,ir,,1,Ich gehe nach Hause.,Voy a casa.,sein,,\n\
                 verb_separable,,aufstehen,levantarse,,2,Ich stehe um 7 Uhr auf.,Me levanto a las 7.,sein,true,\n\
                 verb_reflexive,,sich erinnern,recordar,,3,Ich erinnere mich daran.,Lo recuerdo.,haben,,true\n\
                 fixed_phrase,,Guten Morgen,buenos días,,1,Guten Morgen!,¡Buenos días!,,,\n",
                 )
                .unwrap();

            let out = is_csv_valid(file.path(), EnumCsvType::Worte)?;

            assert_eq!(out.len(), 6);

            // 1. noun_common
            assert_eq!(out[0].get(0), Some("noun_common"));
            assert_eq!(out[0].get(1), Some("1"));
            assert_eq!(out[0].get(2), Some("Haus"));
            assert_eq!(out[0].get(3), Some("casa"));
            assert_eq!(out[0].get(4), Some("Häuser"));
            assert_eq!(out[0].get(5), Some("2"));
            assert_eq!(out[0].get(6), Some("Das Haus ist groß."));
            assert_eq!(out[0].get(7), Some("La casa es grande."));
            assert_eq!(out[0].get(8), Some(""));
            assert_eq!(out[0].get(9), Some(""));
            assert_eq!(out[0].get(10), Some(""));

            // 2. verb_main with aux = haben
            assert_eq!(out[1].get(0), Some("verb_main"));
            assert_eq!(out[1].get(1), Some(""));
            assert_eq!(out[1].get(2), Some("lernen"));
            assert_eq!(out[1].get(3), Some("aprender"));
            assert_eq!(out[1].get(4), Some(""));
            assert_eq!(out[1].get(5), Some("2"));
            assert_eq!(out[1].get(6), Some("Ich lerne Deutsch."));
            assert_eq!(out[1].get(7), Some("Aprendo alemán."));
            assert_eq!(out[1].get(8), Some("haben"));
            assert_eq!(out[1].get(9), Some(""));
            assert_eq!(out[1].get(10), Some(""));

            // 3. verb_main with aux = sein
            assert_eq!(out[2].get(0), Some("verb_main"));
            assert_eq!(out[2].get(1), Some(""));
            assert_eq!(out[2].get(2), Some("gehen"));
            assert_eq!(out[2].get(3), Some("ir"));
            assert_eq!(out[2].get(4), Some(""));
            assert_eq!(out[2].get(5), Some("1"));
            assert_eq!(out[2].get(6), Some("Ich gehe nach Hause."));
            assert_eq!(out[2].get(7), Some("Voy a casa."));
            assert_eq!(out[2].get(8), Some("sein"));
            assert_eq!(out[2].get(9), Some(""));
            assert_eq!(out[2].get(10), Some(""));

            // 4. verb_separable
            assert_eq!(out[3].get(0), Some("verb_separable"));
            assert_eq!(out[3].get(1), Some(""));
            assert_eq!(out[3].get(2), Some("aufstehen"));
            assert_eq!(out[3].get(3), Some("levantarse"));
            assert_eq!(out[3].get(4), Some(""));
            assert_eq!(out[3].get(5), Some("2"));
            assert_eq!(out[3].get(6), Some("Ich stehe um 7 Uhr auf."));
            assert_eq!(out[3].get(7), Some("Me levanto a las 7."));
            assert_eq!(out[3].get(8), Some("sein"));
            assert_eq!(out[3].get(9), Some("true"));
            assert_eq!(out[3].get(10), Some(""));

            // 5. verb_reflexive
            assert_eq!(out[4].get(0), Some("verb_reflexive"));
            assert_eq!(out[4].get(1), Some(""));
            assert_eq!(out[4].get(2), Some("sich erinnern"));
            assert_eq!(out[4].get(3), Some("recordar"));
            assert_eq!(out[4].get(4), Some(""));
            assert_eq!(out[4].get(5), Some("3"));
            assert_eq!(out[4].get(6), Some("Ich erinnere mich daran."));
            assert_eq!(out[4].get(7), Some("Lo recuerdo."));
            assert_eq!(out[4].get(8), Some("haben"));
            assert_eq!(out[4].get(9), Some(""));
            assert_eq!(out[4].get(10), Some("true"));

            // 6. fixed_phrase
            assert_eq!(out[5].get(0), Some("fixed_phrase"));
            assert_eq!(out[5].get(1), Some(""));
            assert_eq!(out[5].get(2), Some("Guten Morgen"));
            assert_eq!(out[5].get(3), Some("buenos días"));
            assert_eq!(out[5].get(4), Some(""));
            assert_eq!(out[5].get(5), Some("1"));
            assert_eq!(out[5].get(6), Some("Guten Morgen!"));
            assert_eq!(out[5].get(7), Some("¡Buenos días!"));
            assert_eq!(out[5].get(8), Some(""));
            assert_eq!(out[5].get(9), Some(""));
            assert_eq!(out[5].get(10), Some(""));

            Ok(())
        }

        #[test]
        fn err_when_columns_count_is_invalid() -> Result<(), AppError> {
            let file = make_temp_csv(
                "setze_spanisch,setze_deutsch,thema\n\
                 Hola mundo,Hallo Welt,greetings\n",
            )
            .unwrap();

            let err: AppError = is_csv_valid(file.path(), EnumCsvType::Setze).unwrap_err();
            match err.kind {
                AppErrorKind::Csv(e) => {
                    assert_eq!(e.file, file.path());
                    assert_eq!(e.row, None);
                    assert_eq!(e.column, None);
                    assert!(e.message.contains("Columns expected"));
                }
                _ => panic!("Should be a CsvParseError"),
            };

            Ok(())
        }

        #[test]
        fn err_when_header_name_does_not_match() -> Result<(), AppError> {
            let file = make_temp_csv(
                "setze_spanisch,setze_deutsch,thema,bad_header\n\
                 Hola mundo,Hallo Welt,greetings,1\n",
            )
            .unwrap();

            let err: AppError = is_csv_valid(file.path(), EnumCsvType::Setze).unwrap_err();
            match err.kind {
                AppErrorKind::Csv(e) => {
                    assert_eq!(e.file, file.path());
                    assert_eq!(e.row, None);
                    assert_eq!(e.column, None);
                    assert!(e.message.contains("Header bad_header doesn't match with"));
                }
                _ => panic!("Should be a CsvParseError"),
            };

            Ok(())
        }

        #[test]
        fn err_when_csv_row_is_malformed() -> Result<(), AppError> {
            let file = make_temp_csv(
                "setze_spanisch,setze_deutsch,thema,schwirig_id\n\
                 \"Hola mundo,Hallo Welt,greetings,1\n",
            )
            .unwrap();

            let err: AppError = is_csv_valid(file.path(), EnumCsvType::Setze).unwrap_err();
            match err.kind {
                AppErrorKind::Csv(e) => {
                    assert_eq!(e.file, file.path());
                    assert_eq!(e.row, Some(1));
                    assert_eq!(e.column, None);
                    assert!(e.message.contains("Error on line"));
                }
                _ => panic!("Should be a CsvParseError"),
            };

            Ok(())
        }

        #[test]
        fn err_when_file_does_not_exist() {
            let missing_path = std::env::temp_dir().join("definitely_missing_csv_for_test_123.csv");

            if missing_path.exists() {
                let _ = fs::remove_file(&missing_path);
            }

            let err: AppError =
                is_csv_valid(missing_path.as_path(), EnumCsvType::Setze).unwrap_err();

            match err.kind {
                AppErrorKind::Csv(e) => {
                    assert_eq!(e.file, missing_path.as_path());
                    assert_eq!(e.row, None);
                    assert_eq!(e.column, None);
                    assert!(e.message.contains("File cannot be open"));
                }
                _ => panic!("Should be a CsvParseError"),
            };
        }
    }

    mod extract_worte_csv {
        use crate::{
            db::schemas::{gram_type::EnumGramType, wort::InputWort, wort_gender::EnumWortGender},
            helpers::csv::extract_worte_csv,
        };

        use super::*;

        #[test]
        fn ok_multiple_rows_and_types() -> Result<(), AppError> {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 noun_common,der,Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n\
                 verb_main,,lernen,aprender,,B1,Ich lerne Deutsch.,Aprendo alemán.,haben,true,false\n\
                 verb_reflexive,,sich erinnern,recordar,,B2,Ich erinnere mich.,Lo recuerdo.,haben,,true\n\
                 fixed_phrase,,Guten Morgen,buenos días,,A1,Guten Morgen!,¡Buenos días!,,,\n",
            )
            .unwrap();

            let out:Vec<InputWort> = extract_worte_csv(file.path())?;

            println!("{:#?}", out[0]);

            assert_eq!(out.len(), 4);

            // noun
            assert_eq!(out[0].gram_type, vec![EnumGramType::NounCommon]);
            assert_eq!(out[0].gender, Some(EnumWortGender::Maskuline));
            assert_eq!(out[0].plural.as_deref(), Some("Häuser"));
            assert_eq!(out[0].verb_aux, None);
            assert_eq!(out[0].trennbar, None);
            assert_eq!(out[0].reflexiv, None);

            // verb_main
            assert_eq!(out[1].gram_type, vec![EnumGramType::VerbMain]);
            assert_eq!(out[1].gender, None);
            assert_eq!(out[1].verb_aux.as_deref(), Some("haben"));
            assert_eq!(out[1].trennbar, Some(true));
            assert_eq!(out[1].reflexiv, Some(false));

            // reflexive verb
            assert_eq!(out[2].gram_type, vec![EnumGramType::VerbReflexive]);
            assert_eq!(out[2].verb_aux.as_deref(), Some("haben"));
            assert_eq!(out[2].trennbar, None);
            assert_eq!(out[2].reflexiv, Some(true));

            // phrase
            assert_eq!(out[3].gram_type, vec![EnumGramType::FixedPhrase]);
            assert_eq!(out[3].verb_aux, None);
            assert_eq!(out[3].trennbar, None);
            assert_eq!(out[3].reflexiv, None);

            Ok(())
        }

        #[test]
        fn ok_multiple_gram_types() -> Result<(), AppError> {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 \"noun_common,adjective\",der,Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n",
            )
            .unwrap();

            let out = extract_worte_csv(file.path())?;

            assert_eq!(
                out[0].gram_type,
                vec![EnumGramType::NounCommon, EnumGramType::Adjective]
            );

            Ok(())
        }

        #[test]
        fn ok_optional_fields_return_none() -> Result<(), AppError> {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 noun_common,,Haus,casa,,A2,Das Haus ist groß.,La casa es grande.,,,\n",
            )
            .unwrap();

            let out = extract_worte_csv(file.path())?;

            let row = &out[0];

            assert_eq!(row.gender, None);
            assert_eq!(row.plural, None);
            assert_eq!(row.verb_aux, None);
            assert_eq!(row.trennbar, None);
            assert_eq!(row.reflexiv, None);

            Ok(())
        }

        #[test]
        fn err_when_gram_type_empty() {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 ,der,Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n",
            )
            .unwrap();

            let err = extract_worte_csv(file.path()).unwrap_err();
            let msg = format!("{err:?}");

            assert!(msg.contains("gram_type"));
        }

        #[test]
        fn err_when_gram_type_invalid() {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 bad_type,der,Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n",
            )
            .unwrap();

            let err = extract_worte_csv(file.path()).unwrap_err();
            let msg = format!("{err:?}");

            assert!(msg.contains("gram_type"));
        }

        #[test]
        fn err_when_worte_de_empty() {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 noun_common,der,,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n",
            )
            .unwrap();

            let err = extract_worte_csv(file.path()).unwrap_err();
            let msg = format!("{err:?}");

            assert!(msg.contains("worte_de"));
        }

        #[test]
        fn err_when_worte_es_empty() {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 noun_common,der,Haus,,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n",
            )
            .unwrap();

            let err = extract_worte_csv(file.path()).unwrap_err();
            let msg = format!("{err:?}");

            assert!(msg.contains("worte_es"));
        }

        #[test]
        fn err_when_niveau_invalid() {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 noun_common,der,Haus,casa,Häuser,Z9,Das Haus ist groß.,La casa es grande.,,,\n",
            )
            .unwrap();

            let err = extract_worte_csv(file.path()).unwrap_err();
            let msg = format!("{err:?}");

            assert!(msg.contains("niveau"));
        }

        #[test]
        fn err_when_example_de_empty() {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 noun_common,der,Haus,casa,Häuser,A2,,La casa es grande.,,,\n",
            )
            .unwrap();

            let err = extract_worte_csv(file.path()).unwrap_err();
            let msg = format!("{err:?}");

            assert!(msg.contains("Cannot be empty"));
        }

        #[test]
        fn err_when_example_es_empty() {
            let file = make_temp_csv(
                "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                 noun_common,der,Haus,casa,Häuser,A2,Das Haus ist groß.,,,,\n",
            )
            .unwrap();

            let err = extract_worte_csv(file.path()).unwrap_err();
            let msg = format!("{err:?}");

            assert!(msg.contains("Cannot be empty"));
        }

        #[test]
        fn err_when_csv_format_is_invalid() {
            let file = make_temp_csv(
                    "gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv\n\
                    noun_common,der,\"Haus,casa,Häuser,A2,Das Haus ist groß.,La casa es grande.,,,\n",
                )
                .unwrap();

            let err = extract_worte_csv(file.path()).unwrap_err();
            let msg = format!("{err:?}");

            assert!(msg.contains("Error on line"));
        }
    }
}
