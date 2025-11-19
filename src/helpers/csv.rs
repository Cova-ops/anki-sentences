use color_eyre::eyre::{Context, Result, bail, eyre};
use csv::ReaderBuilder;
use std::fs::File;

use crate::{
    db::schemas::{
        gender_worte::GenderWorteSchema, gram_type::GramTypeSchema,
        niveau_worte::NiveauWorteSchema, schwirigkeit_liste::SchwirigkeitListeSchema,
        setze::NewSetzeSchema, worte::NewWorteSchema,
    },
    utils::string::StringBoolConvertion,
};

const HEADER_CSV: [&str; 4] = ["setze_spanisch", "setze_deutsch", "thema", "schwirig_id"];

pub fn is_csv_valid(path: &str) -> Result<()> {
    let file = File::open(path)
        .with_context(|| format!("[is_csv_valid] - No se puede abrir el archivo: {}", path))?;

    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let headers = reader
        .headers()
        .context("[is_csv_valid] - Error en encabezados")?;

    if headers.len() != HEADER_CSV.len() {
        return Err(eyre!(
            "[is_csv_valid] - Número de columnas inválido esperado {}, recibido {}",
            HEADER_CSV.len(),
            headers.len()
        ));
    }

    for (i, h) in headers.iter().enumerate() {
        if h != HEADER_CSV[i] {
            return Err(eyre!(
                "[is_csv_valid] - La cabera {} no corresponde con {} (pos {})",
                h,
                HEADER_CSV[i],
                i
            ));
        }
    }

    for (i, result) in reader.records().enumerate() {
        result.with_context(|| format!("[is_csv_valid] - Error en la línea {}", i + 1))?;
    }

    Ok(())
}

pub fn extract_sentences_csv(path: &str) -> Result<Vec<NewSetzeSchema>> {
    let file = File::open(path).with_context(|| {
        format!(
            "[extract_sentences_from] - No se puede abrir el archivo: {}",
            path
        )
    })?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut r: Vec<NewSetzeSchema> = Vec::new();
    for (i, result) in reader.records().enumerate() {
        let value = result
            .with_context(|| format!("[extract_sentences_from] - Error en la línea {}", i + 1))?;

        let col3_value = value.get(3).unwrap_or("1").to_string();
        let schwirig_id = if let Ok(col3) = col3_value.parse::<i32>() {
            SchwirigkeitListeSchema::from_id(col3)
        } else {
            SchwirigkeitListeSchema::from_name(&col3_value)
        };

        let span = value.get(0).unwrap_or("").to_string();
        let deut = value.get(1).unwrap_or("").to_string();
        let them = value.get(2).unwrap_or("").to_string();

        r.push(NewSetzeSchema::new(span, deut, them, schwirig_id?.id));
    }

    Ok(r)
}

pub fn extract_worte_csv(path: &str) -> Result<Vec<NewWorteSchema>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut vec_result: Vec<NewWorteSchema> = Vec::new();
    for (i, result) in reader.records().enumerate() {
        let value = result.with_context(|| format!("Error en la línea del CSV: {}", i + 1))?;

        if value.is_empty() {
            continue;
        }

        let gram_type_list = match value.get(0) {
            Some(v) => v.to_string(),
            None => bail!(
                "El valor gram_type no puede estar vacio. Linea del CSV: {}",
                i
            ),
        };

        let split_gram_type: Vec<&str> = gram_type_list.split(',').collect();
        let mut vec_gram_type: Vec<i32> = Vec::with_capacity(split_gram_type.len());
        for gt in split_gram_type {
            if gt.is_empty() {
                bail!(
                    "El valor de gram_type no puede estar vacio. Linea del CSV: {}",
                    i
                )
            }

            let gram_type = GramTypeSchema::from_code(gt)?;
            vec_gram_type.push(gram_type.id);
        }

        let gender_id = match value.get(1) {
            Some(v) if v.trim().is_empty() => None,
            Some(v) => Some(GenderWorteSchema::from_gender(v)?.id),
            None => None,
        };
        let worte_de = match value.get(2) {
            Some(v) => v.to_string(),
            None => bail!("worte_de no puede ser vacio. Línea del CSV: {}", i),
        };
        let worte_es = match value.get(3) {
            Some(v) => v.to_string(),
            None => bail!("worte_es no puede ser vacio. Línea del CSV: {}", i),
        };
        let plural = value.get(4).map(|s| s.to_string());
        let niveau_id = match value.get(5) {
            Some(v) => NiveauWorteSchema::from_niveau(v)?.id,
            None => bail!("niveau no puede ser vacio. Línea del CSV: {}", i),
        };
        let example_de = match value.get(6) {
            Some(v) => v.to_string(),
            None => bail!("example_de no puede ser vacio. Línea del CSV: {}", i),
        };
        let example_es = match value.get(7) {
            Some(v) => v.to_string(),
            None => bail!("example_es no puede ser vacio. Línea del CSV: {}", i),
        };

        let verb_aux = value.get(8).map(|s| s.to_string());
        let trennbar = value.get(9).map(|s| s.to_bool());
        let reflexiv = value.get(10).map(|s| s.to_bool());

        vec_result.push(NewWorteSchema {
            gram_type: vec_gram_type,
            gender_id,
            worte_de,
            worte_es,
            plural,
            niveau_id,
            example_de,
            example_es,
            verb_aux,
            trennbar,
            reflexiv,
        });
    }

    Ok(vec_result)
}
