use std::sync::{Mutex, MutexGuard};

use once_cell::sync::Lazy;
use rusqlite::Connection;

const DB_NAME: &str = "anki_satze.sql";

pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open(DB_NAME).expect("No se puede abrir/crear la base de datos SQLite");
    Mutex::new(conn)
});

pub fn get_conn() -> MutexGuard<'static, Connection> {
    DB_CONN.lock().expect("Mutex envenenado en DB_CONN")
}
