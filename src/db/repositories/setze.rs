use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    db::{NewSetzeSchema, SchwirigkeitListeSchema, SetzeSchema, get_conn},
    helpers, to_strings,
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
        WHERE rowid >= (abs(random()) % (SELECT IFNULL(MAX(rowid),1) FROM setze))
        LIMIT {limit}"
    );

    let conn = get_conn();
    let mut stmt = conn
        .prepare(&sql)
        .context("[fetch_random] - Error al iniciar la conexión de la DB. Error: {}")?;

    struct Raw {
        id: i32,
        setze_spanisch: String,
        setze_deutsch: String,
        thema: String,
        schwirig_id_num: i32,
        created_at: String,
        deleted_at: Option<String>,
    }
    let rows = stmt
        .query([])
        .with_context(|| format!("[fetch_random] - Error al ejecutar el query: {}", sql))?
        .mapped(|row| {
            Ok(Raw {
                id: row.get(0)?,
                setze_spanisch: row.get(1)?,
                setze_deutsch: row.get(2)?,
                thema: row.get(3)?,
                schwirig_id_num: row.get(4)?,
                created_at: row.get(5)?,
                deleted_at: row.get(6).ok(),
            })
        })
        .collect::<Result<Vec<Raw>, _>>()
        .context("[fetch_random] - recolectar filas")?;

    drop(stmt);
    drop(conn);

    let result: Vec<SetzeSchema> = rows
        .into_iter()
        .map(|r| -> Result<SetzeSchema> {
            let created_at = helpers::time::string_2_datetime(Some(r.created_at)).unwrap();
            let deleted_at = helpers::time::string_2_datetime(r.deleted_at);

            let schwirig_id = SchwirigkeitListeSchema::from_id(r.schwirig_id_num)
                .expect("[fetch_random] - Error al obtener la relación de dificultad");

            Ok(SetzeSchema {
                id: r.id,
                setze_spanisch: r.setze_spanisch,
                setze_deutsch: r.setze_deutsch,
                thema: r.thema,
                schwirig_id,
                created_at,
                deleted_at,
            })
        })
        .collect::<Result<Vec<SetzeSchema>, _>>()
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
