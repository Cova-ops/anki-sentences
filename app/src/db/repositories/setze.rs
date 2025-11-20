use color_eyre::eyre::{Context, Result};
use rand::seq::SliceRandom;
use rusqlite::{Connection, Transaction, params, params_from_iter};
use sql_model::{FromRaw, SqlRaw};

use crate::{
    ctx,
    db::schemas::setze::{NewSetzeSchema, RawSetzeSchema, SetzeSchema},
    with_ctx,
};

pub struct SetzeRepo;

impl SetzeRepo {
    pub fn fetch_random(
        conn: &Connection,
        limit: impl Into<Option<u32>>,
        ids: &mut Vec<i32>,
    ) -> Result<Vec<SetzeSchema>> {
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

        let mut stmt = conn.prepare_cached(&sql)?;
        let rows = stmt
            .query(params_from_iter(select_ids))
            .context(with_ctx!(format!("Error query - {}", sql)))?
            .mapped(RawSetzeSchema::from_sql)
            .collect::<Result<Vec<RawSetzeSchema>, _>>()
            .context(ctx!())?;

        let result = SetzeSchema::from_vec_raw(rows)?;
        Ok(result)
    }

    pub fn fetch_all_themas(conn: &Connection) -> Result<Vec<String>> {
        let sql: &'static str = r#"
        SELECT
            DISTINCT(thema)
        FROM setze s
        WHERE s.deleted_at IS NULL
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
            .context(with_ctx!(format!("Error query - {}", sql)))?
            .mapped(|row| row.get(0))
            .collect::<Result<Vec<i32>, _>>()
            .context(ctx!())?;

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

        let mut stmt = conn.prepare_cached(&sql)?;

        let ids = stmt
            .query(params_from_iter(params))
            .context(with_ctx!(format!("Sql - {}", sql)))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(ids)
    }

    pub fn bulk_insert(conn: &mut Connection, data: &[NewSetzeSchema]) -> Result<Vec<SetzeSchema>> {
        let tx = conn.transaction()?;
        let result = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(result)
    }

    pub fn bulk_insert_tx(tx: &Transaction, data: &[NewSetzeSchema]) -> Result<Vec<SetzeSchema>> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO setze (setze_spanisch, setze_deutsch, thema, schwirigkeit_id)
                VALUES (?1,?2,?3,?4)
            RETURNING id, setze_spanisch, setze_deutsch, schwirig_id, thema, created_at, deleted_at;
        "#;

        let mut out: Vec<SetzeSchema> = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql)?;

        for d in data {
            let raw = stmt.query_one(
                params![d.setze_spanisch, d.setze_deutsch, d.thema, d.schwirig_id],
                RawSetzeSchema::from_sql,
            )?;
            out.push(SetzeSchema::from_raw(raw)?);
        }

        Ok(out)
    }

    pub fn fetch_where_thema(
        conn: &Connection,
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

        let mut stmt = conn.prepare_cached(&sql)?;

        let params: Vec<&dyn rusqlite::ToSql> =
            titles.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

        let rows = stmt
            .query(params_from_iter(params))
            .context(with_ctx!(format!("Sql - {}", sql)))?
            .mapped(RawSetzeSchema::from_sql)
            .collect::<Result<Vec<RawSetzeSchema>, _>>()?;

        let result = SetzeSchema::from_vec_raw(rows)?;
        Ok(result)
    }

    pub fn fetch_by_id(conn: &Connection, ids: &[i32]) -> Result<Vec<SetzeSchema>> {
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

        let mut stmt = conn.prepare_cached(&sql)?;
        let params: Vec<&dyn rusqlite::ToSql> =
            ids.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

        let rows = stmt
            .query(params_from_iter(params))
            .context(with_ctx!(format!("Sql - {}", sql)))?
            .mapped(RawSetzeSchema::from_sql)
            .collect::<Result<Vec<RawSetzeSchema>, _>>()?;

        let result = SetzeSchema::from_vec_raw(rows)?;
        Ok(result)
    }
}
