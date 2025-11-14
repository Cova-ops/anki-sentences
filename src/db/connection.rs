use std::sync::{Mutex, MutexGuard};

use color_eyre::eyre::{Context, Result};
use once_cell::sync::Lazy;
use rusqlite::Connection;

use crate::{
    ctx,
    db::{
        SEED_SCHWIRIGKEIT_LISTE, SchwirigkeitListeBulkInsert,
        schemas::{geschichtlich_setze, schwirigkeit_liste, setze},
    },
};

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
        .context(ctx!())?;

    // Dificultad
    conn.execute(schwirigkeit_liste::CREATE_STR_TABLE_SCHWIRIGKEIT_LISTE, [])
        .context(ctx!())?;

    // Oraciones
    conn.execute(setze::CREATE_STR_TABLE_SETZE, [])
        .context(ctx!())?;
    conn.execute_batch(setze::CREATE_STR_INDEX_SETZE)
        .context(ctx!())?;

    // Historico de oraciones
    conn.execute(
        geschichtlich_setze::CREATE_STR_TABLE_GESCHICHTLICH_SETZE,
        [],
    )
    .context(ctx!())?;
    conn.execute_batch(geschichtlich_setze::CREATE_STR_INDEX_GESCHICHTLICH_SETZE)
        .context(ctx!())?;

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
    schwirigkeit_liste::SchwirigkeitListeSchema::init_data();
}
