use std::ops::DerefMut;

use color_eyre::eyre::{Context, Result};
use rand::seq::SliceRandom;
use rusqlite::{params, params_from_iter};

use crate::{
    ctx,
    db::{
        get_conn,
        schemas::setze::{NewSetzeSchema, SetzeSchema},
    },
    helpers, to_strings, with_ctx,
};

#[derive(Debug)]
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

            let schwirig_id =
                crate::db::schemas::schwirigkeit_liste::SchwirigkeitListeSchema::from_id(
                    r.schwirig_id_num,
                )
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

pub fn fetch_random(limit: impl Into<Option<u32>>, ids: &mut Vec<i32>) -> Result<Vec<SetzeSchema>> {
    let limit = limit.into().unwrap_or(50);

    if ids.is_empty() {
        return Ok(vec![]);
    }

    let mut seed_rand = rand::rng();
    ids.shuffle(&mut seed_rand);

    let select_ids: Vec<i32> = ids.drain(..limit as usize).collect();
    let placeholders = std::iter::repeat_n("?", select_ids.len())
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
        WHERE id IN ({}) AND deleted_at IS NULL
        LIMIT {limit}",
        placeholders
    );

    println!("sql: {}", sql);
    println!("select_ids: {:#?}", select_ids);

    let conn = get_conn();
    let mut stmt = conn.prepare(&sql).context(ctx!())?;

    let rows = stmt
        .query(params_from_iter(select_ids))
        .context(with_ctx!(format!("Error query - {}", sql)))?
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
        .context(ctx!())?;

    drop(stmt);
    drop(conn);

    println!("rows: {:#?}", rows);
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

pub fn fetch_all_only_ids() -> Result<Vec<i32>> {
    let sql: &'static str = r#"
        SELECT
            id
        FROM setze s
        WHERE s.deleted_at IS NULL
        "#;

    let conn = get_conn();
    let mut stmt = conn.prepare(sql).context(ctx!())?;

    let rows = stmt
        .query([])
        .context(with_ctx!(format!("Error query - {}", sql)))?
        .mapped(|row| row.get(0))
        .collect::<Result<Vec<i32>, _>>()
        .context(ctx!())?;

    drop(stmt);
    drop(conn);

    Ok(rows)
}

pub fn fetch_id_schwirig_thema(titles: Option<&[String]>) -> Result<Vec<i32>> {
    let (sql, params) = if titles.is_some() && !titles.unwrap().is_empty() {
        let Some(titles) = titles else { panic!() };
        let placeholders = std::iter::repeat_n("?", titles.len())
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            "SELECT
                id
            FROM setze
            WHERE thema in ({placeholders}) AND schwirigkeit_id = 2 AND deleted_at IS NULL
            ORDER BY id"
        );

        let params: Vec<&dyn rusqlite::ToSql> =
            titles.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

        (sql, params)
    } else {
        let sql = "SELECT
                id
            FROM setze
            WHERE schwirigkeit_id = 2 AND deleted_at IS NULL
            ORDER BY id"
            .to_string();

        let params: Vec<&dyn rusqlite::ToSql> = vec![];
        (sql, params)
    };

    let conn = get_conn();
    let mut stmt = conn.prepare(&sql)?;

    let ids = stmt
        .query(params_from_iter(params))
        .context(with_ctx!(format!("Sql - {}", sql)))?
        .mapped(|r| r.get(0))
        .collect::<Result<Vec<i32>, _>>()?;

    Ok(ids)
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

pub fn fetch_by_id(ids: &[i32]) -> Result<Vec<SetzeSchema>> {
    let placeholders = std::iter::repeat_n("?", ids.len())
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
        WHERE id in ({placeholders})
        ORDER BY setze_deutsch"
    );

    let conn = get_conn();
    let mut stmt = conn.prepare(&sql)?;

    let params: Vec<&dyn rusqlite::ToSql> = ids.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

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
