use std::sync::{Mutex, MutexGuard};

use color_eyre::eyre::{Context, Result};
use once_cell::sync::Lazy;
use rusqlite::Connection;

use crate::db::schemas::schwirigkeit_liste;

const DB_NAME: &str = "anki_satze.sql";

pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    println!("Iniciamos conexion");
    let conn = Connection::open(DB_NAME).expect("No se puede abrir/crear la base de datos SQLite");

    Mutex::new(conn)
});

pub fn get_conn() -> MutexGuard<'static, Connection> {
    DB_CONN.lock().expect("Mutex envenenado en DB_CONN")
}

pub fn init_data() {
    schwirigkeit_liste::SchwirigkeitListeSchema::init_data();
}
