use std::sync::{Mutex, MutexGuard};

use color_eyre::eyre::{Context, Result};
use once_cell::sync::Lazy;
use rusqlite::Connection;

use crate::db::{SEED_SCHWIRIGKEIT_LISTE, SchwirigkeitListeBulkInsert, SchwirigkeitListeSchema};

const DB_NAME: &str = "anki_satze.sql";

pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    println!("Iniciamos conexion");
    let conn = Connection::open(DB_NAME).expect("No se puede abrir/crear la base de datos SQLite");

    Mutex::new(conn)
});

pub fn get_conn() -> MutexGuard<'static, Connection> {
    DB_CONN.lock().expect("Mutex envenenado en DB_CONN")
}

pub fn init_schemas() -> Result<()> {
    let conn = get_conn();
    // Activar las llaves foráneas
    conn.execute("PRAGMA foreign_keys = ON", [])
        .context("[init_schemas] - banderas para llaver foraneas")?;

    // Dificultad
    conn.execute(
        "
            CREATE TABLE IF NOT EXISTS schwirigkeit_liste (
                id                  INTEGER PRIMARY KEY AUTOINCREMENT,
                schwirigkeit        TEXT,
                created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
                deleted_at          TEXT
            )",
        [],
    )
    .context("[init_schemas] - Creación tabla schwirigkeit_liste")?;

    // Oraciones
    conn.execute(
        "
            CREATE TABLE IF NOT EXISTS setze (
                id                  INTEGER PRIMARY KEY AUTOINCREMENT,
                setze_spanisch      TEXT NOT NULL,
                setze_deutsch       TEXT NOT NULL,
                thema               TEXT NOT NULL,
                schwirigkeit_id     INTEGER NOT NULL,
                created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
                deleted_at          TEXT,
                FOREIGN KEY(schwirigkeit_id) REFERENCES schwirigkeit_liste(id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )",
        [],
    )
    .context("[init_schemas] - Creación tabla setze")?;

    conn.execute_batch(
        "
            CREATE INDEX IF NOT EXISTS idx_setze_setze_spanisch ON setze(setze_spanisch);
            CREATE INDEX IF NOT EXISTS idx_setze_setze_deutsch ON setze(setze_deutsch);
            CREATE INDEX IF NOT EXISTS idx_setze_thema ON setze(thema);
            CREATE INDEX IF NOT EXISTS idx_setze_schwirigkeit_id ON setze(schwirigkeit_id);
        ",
    )
    .context("[init_schemas] - Creación particiones setze")?;

    // Historico de oraciones
    conn.execute(
        "
            CREATE TABLE IF NOT EXISTS geschichtlich_setze (
                id                  INTEGER PRIMARY KEY AUTOINCREMENT,
                setze_id            INTEGER NOT NULL,
                result              BOOL NOT NULL,
                created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
                deleted_at          TEXT,
                FOREIGN KEY(setze_id) REFERENCES setze(id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )",
        [],
    )
    .context("[init_schemas] - Creación tabla geschichtlich_setze")?;

    conn.execute_batch(
        "
            CREATE INDEX IF NOT EXISTS idx_geschichtlich_setze_created_at ON geschichtlich_setze(created_at);
        ",
    )
    .context("[init_schemas] - Creación partiaciones geschichtlich_setze")?;

    Ok(())
}

pub fn init_seeds() -> Result<()> {
    SchwirigkeitListeBulkInsert(&SEED_SCHWIRIGKEIT_LISTE)?;

    // for data in SEED_SCHWIRIGKEIT_LISTE {
    //     conn.execute(
    //         "INSERT INTO schwirigkeit_liste (id, schwirigkeit)
    //     VALUES (?1, ?2)",
    //         data,
    //     )?;
    // }
    Ok(())
}

pub fn init_data() {
    SchwirigkeitListeSchema::init_data();
}
