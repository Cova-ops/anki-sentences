use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use color_eyre::eyre::{Context, Result};
use once_cell::sync::Lazy;
use rusqlite::params;

use crate::{
    ctx,
    db::{
        get_conn,
        schemas::schwirigkeit_liste::{NewSchwirigkeitSchema, SchwirigkeitListeSchema},
    },
    with_ctx,
};

pub static SCHWIRIGKEIT_CACHE: Lazy<HashMap<i32, SchwirigkeitListeSchema>> = Lazy::new(|| {
    let sql = "
        SELECT id, schwirigkeit, created_at, deleted_at
        FROM schwirigkeit_liste
    ";

    let conn = get_conn();
    let mut stmt = conn
        .prepare(sql)
        .expect("Fallo al preparar query de schwirigkeit_liste");

    let rows = stmt
        .query([]) // query() en vez de query_map + collect manual
        .expect("Fallo al hacer query de schwirigkeit_liste");

    // Vamos a recolectar manualmente porque tenemos que parsear fechas
    let mut map_out: HashMap<i32, SchwirigkeitListeSchema> = HashMap::new();

    let mut rows = rows;
    while let Some(row) = rows.next().expect("Error leyendo filas") {
        let created_at_str: String = row
            .get(2)
            .expect("Fallo al leer created_at de schwirigkeit_liste");

        let deleted_at_str: Option<String> = row
            .get::<_, Option<String>>(3)
            .expect("Fallo al leer deleted_at de schwirigkeit_liste");

        let naive_created_at = NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
            .expect("Formato de fecha inválido en created_at");

        let created_at = DateTime::<Utc>::from_naive_utc_and_offset(naive_created_at, Utc);

        let deleted_at = if let Some(s) = deleted_at_str {
            let naive_deleted_at = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .expect("Formato de fecha inválido en deleted_at");

            Some(DateTime::<Utc>::from_naive_utc_and_offset(
                naive_deleted_at,
                Utc,
            ))
        } else {
            None
        };

        let id = row.get(0).expect("id inválido");
        let schwirigkeit: String = row.get(1).expect("schwirigkeit inválido");

        map_out.insert(
            id,
            SchwirigkeitListeSchema {
                id,
                schwirigkeit,
                created_at,
                deleted_at,
            },
        );
    }

    map_out
});

// Función para obtener referencia inmutable al cache
pub fn fetch_all() -> &'static HashMap<i32, SchwirigkeitListeSchema> {
    &SCHWIRIGKEIT_CACHE
}

pub fn bulk_insert(data: &Vec<NewSchwirigkeitSchema>) -> Result<()> {
    let sql = "INSERT INTO schwirigkeit_liste (id, schwirigkeit)
        VALUES (?1,?2) ON CONFLICT(id) DO UPDATE SET schwirigkeit = ?3;";

    let mut conn = get_conn();
    let tx = conn.transaction().context(ctx!())?;

    {
        let mut stmt = tx
            .prepare_cached(sql)
            .context(with_ctx!(format!("error sql: {}", sql)))?;

        for d in data {
            stmt.execute(params![d.id, d.schwirigkeit, d.schwirigkeit])
                .with_context(|| {
                    format!(
                        "[bulk_insert] - Error en sql: {}. Con params: {:#?}",
                        sql,
                        [
                            d.id.to_string(),
                            d.schwirigkeit.clone(),
                            d.schwirigkeit.clone()
                        ]
                    )
                })?;
        }
    }

    tx.commit().context("[bulk_insert] - Error: {}")?;

    Ok(())
}
