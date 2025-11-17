use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    ctx,
    db::{get_conn, schemas::geschichtlich_setze::GeschichtlichSetzeSchema},
    helpers, to_strings, with_ctx,
};

#[derive(Debug)]
struct Raw {
    id: i32,
    setze_id: i32,
    result: i32,
    created_at: String,
    deleted_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewGeschichtlichSetze {
    pub setze_id: i32,
}

// TODO: Pasar esto al schema de geschichtlich_setze
impl NewGeschichtlichSetze {
    pub fn new(s_id: impl Into<i32>) -> Self {
        Self {
            setze_id: s_id.into(),
        }
    }

    pub fn insert_db(&self, result: bool) -> Result<GeschichtlichSetzeSchema> {
        let mut conn = get_conn();
        let tx = conn.transaction().context(ctx!())?;

        let sql = r#"
        INSERT INTO geschichtlich_setze (setze_id, result)
        VALUES (?1,?2)
        RETURNING id,setze_id, result, created_at,deleted_at;"#;

        let raw: Raw = {
            let mut stmt = tx
                .prepare_cached(sql)
                .context(with_ctx!(format!("sql: {}", sql)))?;

            stmt.query_row(params![self.setze_id, result], |r| {
                Ok(Raw {
                    id: r.get(0)?,
                    setze_id: r.get(1)?,
                    result: r.get(2)?,
                    created_at: r.get(3)?,
                    deleted_at: r.get::<_, Option<String>>(4)?,
                })
            })
            .context(with_ctx!(format!(
                "sql: {} & params: {:#?}",
                sql,
                to_strings!(self.setze_id, result)
            )))?
        };

        tx.commit().context(ctx!())?;

        let result = {
            let created_at = helpers::time::string_2_datetime(Some(raw.created_at)).unwrap();
            let deleted_at = helpers::time::string_2_datetime(raw.deleted_at);

            GeschichtlichSetzeSchema {
                id: raw.id,
                setze_id: raw.setze_id,
                result: raw.result != 0,
                created_at,
                deleted_at,
            }
        };

        Ok(result)
    }
}
