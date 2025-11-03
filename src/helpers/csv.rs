use color_eyre::eyre::{Context, Result, eyre};
use csv::ReaderBuilder;
use std::{fs::File, path::Path};

use crate::db::{NewSetzeSchema, SchwirigkeitListeSchema};

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
