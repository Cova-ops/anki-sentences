use csv::{ReaderBuilder, StringRecord};
use std::{collections::HashMap, fs::File, path::Path, str::FromStr};

use crate::{
    db::schemas::{
        gram_type::EnumGramType, niveau_liste::EnumNiveauListe, setze::InputSetze, wort::InputWort,
        wort_gender::EnumWortGender,
    },
    helpers::error_handler::{AppError, CsvParseError},
    traits::string::StringConvertion,
};

#[cfg(test)]
mod test;

pub(super) enum EnumCsvType {
    Setze,
    Worte,
}

pub(super) static HEADER_SETZE_CSV: [&str; 4] =
    ["setze_spanisch", "setze_deutsch", "thema", "schwirig_id"];

pub(super) static HEADER_WORTE_CSV: [&str; 11] = [
    "gram_type",
    "gender_id",
    "worte_de",
    "worte_es",
    "plural",
    "niveau_id",
    "example_de",
    "example_es",
    "verb_aux",
    "trennbar",
    "reflexiv",
];

/// Función para validar si un archivo tiene la estructura adecuada
/// params:
/// - path: Ruta local donde se aloja el CSV.
/// - type_file: EnumCsvType Tipo de archivo a subir
///
/// return:
/// Regresa un Result ó Report segun el caso
pub(super) fn is_csv_valid(
    path: &Path,
    type_file: EnumCsvType,
) -> Result<Vec<StringRecord>, AppError> {
    let file = File::open(path).map_err(|e| CsvParseError {
        file: path.to_owned(),
        row: None,
        column: None,
        message: format!("File cannot be open: {}", e),
    })?;

    let header_csv: &'static [&'static str] = match type_file {
        EnumCsvType::Setze => &HEADER_SETZE_CSV,
        EnumCsvType::Worte => &HEADER_WORTE_CSV,
    };

    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let headers = reader.headers().map_err(|e| CsvParseError {
        file: path.to_owned(),
        row: None,
        column: None,
        message: format!("Headers are required: {}", e),
    })?;

    if headers.len() != header_csv.len() {
        return Err(CsvParseError {
            file: path.to_owned(),
            row: None,
            column: None,
            message: format!(
                "Columns expected: {}, founded: {}",
                header_csv.len(),
                headers.len()
            ),
        }
        .into());
    }

    for (i, h) in headers.iter().enumerate() {
        if h != header_csv[i] {
            return Err(CsvParseError {
                file: path.to_owned(),
                row: None,
                column: None,
                message: format!(
                    "Header {} doesn't match with {}, (position {})",
                    h, header_csv[i], i
                ),
            }
            .into());
        }
    }

    let mut vec_out: Vec<StringRecord> = vec![];
    for (i, result) in reader.records().enumerate() {
        let line = result.map_err(|e| CsvParseError {
            file: path.to_owned(),
            row: Some(i + 1),
            column: None,
            message: format!("Error on line: {}", e),
        })?;
        vec_out.push(line);
    }

    Ok(vec_out)
}

pub fn extract_sentences_csv(path: &Path) -> Result<Vec<InputSetze>, AppError> {
    let records = is_csv_valid(path, EnumCsvType::Setze)?;

    let mut vec_out: Vec<InputSetze> = Vec::new();
    for (i, value) in records.into_iter().enumerate() {
        let setze_spanisch = match value.get(0) {
            Some(v) if v.trim().len() > 0 => v.to_owned(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("setze_spanisch"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let setze_deutsch = match value.get(1) {
            Some(v) if v.trim().len() > 0 => v.to_owned(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("setze_deutsch"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let thema = match value.get(2) {
            Some(v) if v.trim().len() > 0 => v.to_owned(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("thema"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let niveau = match value.get(3) {
            Some(v) if v.trim().len() > 0 => v.to_owned(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("thema"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let niveau = match niveau.parse::<i32>() {
            Ok(v) => EnumNiveauListe::try_from(v),
            Err(_) => EnumNiveauListe::from_str(&niveau),
        }
        .map_err(|e| CsvParseError {
            file: path.to_owned(),
            row: Some(i + 1),
            column: Some("niveau"),
            message: e.message,
        })?;

        let satz = InputSetze {
            setze_spanisch,
            setze_deutsch,
            thema,
            niveau,
        };

        vec_out.push(satz);
    }

    Ok(vec_out)
}

/// This function extract the data from the CSV and convert into Vec<InputWort>
/// Considerations:
/// - For duplicated words, it only the first one
pub fn extract_worte_csv(path: &Path) -> Result<Vec<InputWort>, AppError> {
    let records = is_csv_valid(path, EnumCsvType::Worte)?;

    // (worte_es, worte_de) is the key for duplicated
    let mut hash_input = HashMap::<(String, String), InputWort>::new();
    let mut vec_out: Vec<InputWort> = vec![];
    for (i, value) in records.into_iter().enumerate() {
        if value.is_empty() {
            continue;
        }

        let worte_de: String = match value.get(2) {
            Some(v) if v.trim().len() > 0 => v.to_string(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("worte_de"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let worte_es: String = match value.get(3) {
            Some(v) if v.trim().len() > 0 => v.to_string(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("worte_es"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        // Check is it is duplicated
        if hash_input.contains_key(&(worte_es.clone(), worte_de.clone())) {
            println!("Line repetead on line {} skipping it.", i + 1);
            continue;
        }

        let gram_type_list = match value.get(0) {
            Some(v) if !v.trim().is_empty() => v.to_owned(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("gram_type"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let split_gram_type: Vec<&str> = gram_type_list.split(',').collect();
        let mut vec_gram_type: Vec<EnumGramType> = Vec::with_capacity(split_gram_type.len());
        for gt in split_gram_type {
            if gt.is_empty() {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("gram_type"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }

            let gram_type: EnumGramType =
                EnumGramType::from_str(gt).map_err(|e| CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("gram_type"),
                    message: e.message,
                })?;
            vec_gram_type.push(gram_type);
        }

        let gender: Option<EnumWortGender> = match value.get(1) {
            Some(v) if v.trim().is_empty() => None,
            Some(v) => Some(EnumWortGender::from_str(v).map_err(|e| CsvParseError {
                file: path.to_owned(),
                row: Some(i + 1),
                column: Some("gender_id"),
                message: e.message,
            })?),
            None => None,
        };

        let plural: Option<String> = match value.get(4) {
            Some(v) if v.trim().len() > 0 => Some(v.to_owned()),
            _ => None,
        };

        let niveau: EnumNiveauListe = match value.get(5) {
            Some(v) if v.trim().len() > 0 => {
                EnumNiveauListe::from_str(v).map_err(|e| CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("niveau"),
                    message: e.message,
                })?
            }
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("niveau"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let example_de: String = match value.get(6) {
            Some(v) if v.trim().len() > 0 => v.to_string(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("example_de"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let example_es: String = match value.get(7) {
            Some(v) if v.trim().len() > 0 => v.to_string(),
            _ => {
                return Err(CsvParseError {
                    file: path.to_owned(),
                    row: Some(i + 1),
                    column: Some("example_es"),
                    message: String::from("Cannot be empty"),
                }
                .into());
            }
        };

        let verb_aux: Option<String> = match value.get(8) {
            Some(v) if v.trim().len() > 0 => Some(v.to_string()),
            _ => None,
        };

        let trennbar: Option<bool> = match value.get(9) {
            Some(v) if v.trim().len() > 0 => Some(v.to_bool()),
            _ => None,
        };

        let reflexiv: Option<bool> = match value.get(10) {
            Some(v) if v.trim().len() > 0 => Some(v.to_bool()),
            _ => None,
        };

        let input = InputWort {
            gram_type: vec_gram_type,
            gender,
            worte_de: worte_de.clone(),
            worte_es: worte_es.clone(),
            plural,
            niveau,
            example_de,
            example_es,
            verb_aux,
            trennbar,
            reflexiv,
        };

        hash_input.insert((worte_es, worte_de), input.clone());
        vec_out.push(input);
    }

    Ok(vec_out)
}
