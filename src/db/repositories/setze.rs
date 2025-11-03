use chrono::{DateTime, NaiveDateTime, Utc};
use color_eyre::eyre::{Context, Result};
use rusqlite::{fallible_iterator::FallibleIterator, params};

use crate::{
    db::{NewSetzeSchema, SchwirigkeitListeSchema, SetzeSchema, get_conn},
    to_strings,
};

pub fn fetch_random(limit: impl Into<Option<u32>>) -> Result<Vec<SetzeSchema>> {
    let limit = limit.into().unwrap_or(50);

    let sql = format!(
        "SELECT
            id,
            setze_spanisch,
            setze_deutsch,
            thema,
            schwirigkeit_id,
            created_at,
            deleted_at
        FROM setze
        ORDER BY RANDOM()
        LIMIT {limit}"
    );

    let conn = get_conn();
    let mut stmt = conn
        .prepare(&sql)
        .context("[fetch_random] - Error al iniciar la conexión de la DB. Error: {}")?;

    let rows = stmt
        .query([])
        .with_context(|| format!("[fetch_random] - Error al ejecutar el query: {}", sql))?
        .map(|row| {
            let created_at_str: String = row.get(5)?;
            let deleted_at_str: Option<String> = row.get::<_, Option<String>>(6)?;

            let naive_created_at =
                NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S").unwrap();
            let created_at = DateTime::<Utc>::from_naive_utc_and_offset(naive_created_at, Utc);

            let mut deleted_at: Option<DateTime<Utc>> = None;
            if let Some(aux_date) = deleted_at_str {
                let naive_deleted_at =
                    NaiveDateTime::parse_from_str(&aux_date, "%Y-%m-%d %H:%M:%S").unwrap();
                deleted_at = Some(DateTime::<Utc>::from_naive_utc_and_offset(
                    naive_deleted_at,
                    Utc,
                ));
            }

            let id: i32 = row.get(0)?;
            let setze_spanisch: String = row.get(1)?;
            let setze_deutsch: String = row.get(2)?;
            let thema: String = row.get(3)?;
            let schwirig_id = SchwirigkeitListeSchema::from_id(row.get::<_, i32>(4)?)
                .expect("[fetch_random] - Error al obtener la relación de dificultad");

            Ok(SetzeSchema {
                id,
                setze_spanisch,
                setze_deutsch,
                thema,
                schwirig_id,
                created_at,
                deleted_at,
            })
        });

    let result: Vec<SetzeSchema> = rows
        .collect()
        .context("[fetch_random] - Error al recolectar la información")?;
    Ok(result)
}

pub fn bulk_insert(data: Vec<NewSetzeSchema>) -> Result<()> {
    let sql = "INSERT INTO setze (setze_spanisch, setze_deutsch, thema, schwirigkeit_id)
        VALUES (?1,?2,?3,?4);";

    let mut conn = get_conn();
    let tx = conn
        .transaction()
        .context("[bulk_insert] - Error al crear la transacción.")?;

    {
        let mut stmt = tx
            .prepare_cached(sql)
            .with_context(|| format!("[bulk_insert] - Error con el sql: {}", sql))?;

        for d in data {
            stmt.execute(params![
                d.setze_spanisch,
                d.setze_deutsch,
                d.thema,
                d.schwirig_id
            ])
            .with_context(|| {
                format!(
                    "[bulk_insert] - Error con sql: {}. Con parametros: {:?}",
                    sql,
                    to_strings!(d.setze_spanisch, d.setze_deutsch, d.thema, d.schwirig_id)
                )
            })?;
        }
    }

    tx.commit()
        .context("[bulk_insert] - Error al hacer el commit")?;

    Ok(())
}
