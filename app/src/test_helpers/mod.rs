use rusqlite::Connection;

pub fn new_test_conn() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn
}
