use color_eyre::eyre::{Context, Result};
use rusqlite::{params, params_from_iter};

use crate::{
    ctx,
    db::{NewSetzeSchema, SchwirigkeitListeSchema, SetzeSchema, get_conn},
    helpers, to_strings, with_ctx,
};

struct RawStruct {
    id: i32,
    setze_spanisch: String,
    setze_deutsch: String,
    thema: String,
    schwirig_id_num: i32,
    created_at: String,
    deleted_at: Option<String>,
}

fn from_raw_to_setze(rows: Vec<RawStruct>) -> Result<Vec<SetzeSchema>> {
    rows.into_iter()
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
}

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

    let rows = stmt
        .query([])
        .with_context(|| format!("[fetch_random] - Error al ejecutar el query: {}", sql))?
        .mapped(|row| {
            Ok(RawStruct {
                id: row.get(0)?,
                setze_spanisch: row.get(1)?,
                setze_deutsch: row.get(2)?,
                thema: row.get(3)?,
                schwirig_id_num: row.get(4)?,
                created_at: row.get(5)?,
                deleted_at: row.get(6).ok(),
            })
        })
        .collect::<Result<Vec<RawStruct>, _>>()
        .context("[fetch_random] - recolectar filas")?;

    drop(stmt);
    drop(conn);

    let result = from_raw_to_setze(rows)?;
    Ok(result)
}

pub fn fetch_all_titles() -> Result<Vec<String>> {
    let sql: &'static str = r#"
        SELECT
            DISTINCT(thema)
        FROM setze s
        WHERE s.deleted_at IS NULL
        "#;

    let conn = get_conn();
    let mut stmt = conn.prepare(sql).context(ctx!())?;

    let rows = stmt
        .query([])
        .context(with_ctx!(format!("Error query - {}", sql)))?
        .mapped(|row| row.get(0))
        .collect::<Result<Vec<String>, _>>()
        .context("[fetch_random] - recolectar filas")?;

    drop(stmt);
    drop(conn);

    Ok(rows)
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

pub fn fetch_where_thema(
    titles: &[String],
    offset: impl Into<u32>,
    limit: impl Into<u32>,
) -> Result<Vec<SetzeSchema>> {
    let offset = offset.into();
    let limit = limit.into();

    let placeholders = std::iter::repeat_n("?", titles.len())
        .collect::<Vec<_>>()
        .join(",");

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
        WHERE thema in ({placeholders})
        ORDER BY setze_deutsch
        LIMIT {limit} OFFSET {offset}"
    );

    let conn = get_conn();
    let mut stmt = conn.prepare(&sql)?;

    let params: Vec<&dyn rusqlite::ToSql> =
        titles.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

    let rows = stmt
        .query(params_from_iter(params))
        .context(with_ctx!(format!("Sql - {}", sql)))?
        .mapped(|row| {
            Ok(RawStruct {
                id: row.get(0)?,
                setze_spanisch: row.get(1)?,
                setze_deutsch: row.get(2)?,
                thema: row.get(3)?,
                schwirig_id_num: row.get(4)?,
                created_at: row.get(5)?,
                deleted_at: row.get(6).ok(),
            })
        })
        .collect::<Result<Vec<RawStruct>, _>>()?;

    drop(stmt);
    drop(conn);

    let result = from_raw_to_setze(rows)?;
    Ok(result)
}
