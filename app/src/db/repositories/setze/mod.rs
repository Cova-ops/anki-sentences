use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction, params_from_iter};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::{
    db::schemas::setze::{NewSetzeSchema as New, RawSetzeSchema as Raw, SetzeSchema as Schema},
    with_ctx,
};

#[cfg(test)]
mod setze_test;

pub struct SetzeRepo;

impl SetzeRepo {
    pub fn fetch_all_themas(conn: &Connection) -> Result<Vec<String>> {
        let sql: &'static str = r#"
            SELECT
                DISTINCT(thema)
            FROM setze s
            WHERE s.deleted_at IS NULL
            ORDER BY s.id
        "#;

        let mut stmt = conn.prepare_cached(sql)?;

        let rows = stmt
            .query([])
            .context(with_ctx!(format!("Error query - {}", sql)))?
            .mapped(|row| row.get(0))
            .collect::<Result<Vec<String>, _>>()
            .context("[fetch_random] - recolectar filas")?;

        Ok(rows)
    }

    pub fn fetch_all_only_ids(conn: &Connection) -> Result<Vec<i32>> {
        let sql: &'static str = r#"
            SELECT
                id
            FROM setze s
            WHERE s.deleted_at IS NULL
        "#;

        let mut stmt = conn.prepare_cached(sql)?;

        let rows = stmt
            .query([])
            .context(format!("sql: {}", sql))?
            .mapped(|row| row.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(rows)
    }

    pub fn fetch_id_schwirig_thema(
        conn: &Connection,
        titles: Option<&[String]>,
    ) -> Result<Vec<i32>> {
        let (sql, params) = if titles.is_some() && !titles.unwrap().is_empty() {
            let Some(titles) = titles else { panic!() };
            let placeholders = std::iter::repeat_n("?", titles.len())
                .collect::<Vec<_>>()
                .join(",");

            let sql = format!(
                "SELECT
                    id
                FROM setze
                WHERE thema in ({placeholders}) AND niveau_id >= 3 AND deleted_at IS NULL
                ORDER BY id"
            );

            let params: Vec<&dyn rusqlite::ToSql> =
                titles.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

            (sql, params)
        } else {
            let sql = "SELECT
                    id
                FROM setze
                WHERE niveau_id >= 3 AND deleted_at IS NULL
                ORDER BY id"
                .to_string();

            let params: Vec<&dyn rusqlite::ToSql> = vec![];
            (sql, params)
        };

        let mut stmt = conn.prepare_cached(&sql)?;

        let ids = stmt
            .query(params_from_iter(params))
            .context(with_ctx!(format!("Sql - {}", sql)))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(ids)
    }

    pub fn fetch_id_neue_sentences(conn: &Connection) -> Result<Vec<i32>> {
        let sql = "
            SELECT
                s.id
            FROM setze s
            WHERE NOT EXISTS (
                SELECT 1
                FROM geschichtlich_setze g
                WHERE g.setze_id = s.id
            )
            AND s.deleted_at IS NULL;"
            .to_string();

        let mut stmt = conn.prepare_cached(&sql)?;

        let ids = stmt
            .query([])
            .context(with_ctx!(format!("Sql - {}", sql)))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(ids)
    }

    pub fn bulk_insert(conn: &mut Connection, data: &[New]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let result = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(result)
    }

    pub fn bulk_insert_tx(tx: &Transaction, data: &[New]) -> Result<Vec<Schema>> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO setze (setze_spanisch, setze_deutsch, niveau_id, thema)
                VALUES (?1,?2,?3,?4)
            RETURNING id, setze_spanisch, setze_deutsch, niveau_id, thema, created_at, deleted_at;
        "#;

        let mut out: Vec<Schema> = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql)?;

        for d in data {
            let raw = stmt
                .query_one(d.to_params(), Raw::from_sql)
                .with_context(|| format!("sql: {}, params: {:#?}", sql, d))?;
            out.push(Schema::from_raw(raw)?);
        }

        Ok(out)
    }

    pub fn fetch_id_where_thema(conn: &Connection, titles: &[String]) -> Result<Vec<i32>> {
        let placeholders = std::iter::repeat_n("?", titles.len())
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            "SELECT id
            FROM setze
            WHERE thema in ({placeholders})
            ORDER BY setze_deutsch"
        );

        let mut stmt = conn.prepare_cached(&sql)?;

        let params: Vec<&dyn rusqlite::ToSql> =
            titles.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

        let vec_ids = stmt
            .query(params_from_iter(params))
            .context(with_ctx!(format!("Sql - {}", sql)))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(vec_ids)
    }

    pub fn fetch_by_id(conn: &Connection, ids: &[i32]) -> Result<Vec<Schema>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = std::iter::repeat_n("?", ids.len())
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            "SELECT
                id,
                setze_spanisch,
                setze_deutsch,
                niveau_id,
                thema,
                created_at,
                deleted_at
            FROM setze
            WHERE id in ({placeholders})
            ORDER BY setze_deutsch"
        );

        let mut stmt = conn.prepare_cached(&sql)?;
        let params: Vec<&dyn rusqlite::ToSql> =
            ids.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

        let rows = stmt
            .query(params_from_iter(params))
            .context(format!("sql: {}, params: {:#?}", sql, ids))?
            .mapped(Raw::from_sql)
            .collect::<Result<Vec<Raw>, _>>()?;

        let result = Schema::from_vec_raw(rows)?;
        Ok(result)
    }

    pub fn fetch_setze_without_audio(conn: &Connection) -> Result<Vec<Schema>> {
        let sql = "
            SELECT
                s.id,
                s.setze_spanisch,
                s.setze_deutsch,
                s.niveau_id,
                s.thema,
                s.created_at,
                s.deleted_at
            FROM setze s
            LEFT JOIN setze_audio sa ON s.id = sa.wort_id 
            WHERE s.deleted_at IS NULL AND sa.satz_id is NULL
            ORDER BY s.id ASC;
        "
        .to_string();

        let mut stmt = conn.prepare_cached(&sql)?;

        let raws = stmt
            .query([])
            .context(format!("Sql - {}", sql))?
            .mapped(Raw::from_sql)
            .collect::<Result<Vec<Raw>, _>>()?;

        let vec_out = Schema::from_vec_raw(raws)?;
        Ok(vec_out)
    }
}
