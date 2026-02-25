use rusqlite::{Connection, OptionalExtension, Transaction, params, params_from_iter};

use crate::{
    db::{
        schemas::setze::{InputSetze, SchemaSetze, SqlSetze},
        traits::{FromSql, SqlInsert},
    },
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod setze_test;

pub struct SetzeRepo;

impl SetzeRepo {
    pub fn bulk_insert(
        conn: &mut Connection,
        data: &[InputSetze],
    ) -> Result<Vec<SchemaSetze>, DbError> {
        let tx = conn.transaction()?;
        let result = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(result)
    }

    pub fn bulk_insert_tx(
        tx: &Transaction,
        data: &[InputSetze],
    ) -> Result<Vec<SchemaSetze>, DbError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO setze (setze_spanisch, setze_deutsch, niveau_id, thema)
                VALUES (?1,?2,?3,?4)
            RETURNING id, setze_spanisch, setze_deutsch, niveau_id, thema, created_at, deleted_at;
        "#;

        let mut out: Vec<SchemaSetze> = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare(sql)?;

        for d in data {
            let params: SqlSetze = d.to_owned().into();
            let raw = stmt
                .query_one(
                    params_from_iter(params.insert_params()),
                    SchemaSetze::from_sql,
                )
                .map_err(DbError::with_sql(sql))?;

            out.push(raw);
        }

        Ok(out)
    }

    pub fn fetch_id_neue_sentences(conn: &Connection) -> Result<Vec<i32>, DbError> {
        let sql = "
            SELECT
                s.id
            FROM setze s
            WHERE NOT EXISTS (
                SELECT 1
                FROM setze_review sr
                WHERE sr.satz_id = s.id
            )
            AND s.deleted_at IS NULL
            ORDER BY s.id ASC;
            ";

        let mut stmt = conn.prepare(sql)?;

        let ids = stmt
            .query([])
            .map_err(DbError::with_sql(sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()
            .map_err(DbError::with_sql(sql))?;

        Ok(ids)
    }

    pub fn fetch_by_id(conn: &Connection, ids: &[i32]) -> Result<Vec<SchemaSetze>, DbError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = vec!["?"; ids.len()].join(",");

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

        let mut stmt = conn.prepare(&sql)?;

        let rows = stmt
            .query(params_from_iter(ids))
            .map_err(DbError::with_sql(&sql))?
            .mapped(SchemaSetze::from_sql)
            .collect::<Result<Vec<SchemaSetze>, _>>()
            .map_err(DbError::with_sql(&sql))?;

        Ok(rows)
    }

    pub fn fetch_one(conn: &Connection, id: i32) -> Result<Option<SchemaSetze>, DbError> {
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
            WHERE id = ?1 AND deleted_at is NULL
            ORDER BY setze_deutsch"
        );

        let mut stmt = conn.prepare(&sql)?;

        let rows = stmt
            .query_one(params![id], SchemaSetze::from_sql)
            .optional() // Catch if it doesn't exists
            .map_err(DbError::with_sql(&sql))?;

        Ok(rows)
    }

    pub fn fetch_id_without_audio(conn: &Connection) -> Result<Vec<i32>, DbError> {
        let sql = "
            SELECT
                s.id,
            FROM setze s
            LEFT JOIN setze_audio sa ON s.id = sa.wort_id 
            WHERE s.deleted_at IS NULL AND sa.satz_id is NULL
            ORDER BY s.id ASC;
        ";

        let mut stmt = conn.prepare_cached(sql)?;

        let raws = stmt
            .query([])
            .map_err(DbError::with_sql(sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()
            .map_err(DbError::with_sql(sql))?;

        Ok(raws)
    }
}
