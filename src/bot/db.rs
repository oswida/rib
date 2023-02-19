use rusqlite::{Connection, OpenFlags};
use std::path::Path;


pub fn init_database() -> Connection {
    let b = Path::new("titus.db").exists();

    let conn = Connection::open_with_flags(
        "titus.db",
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )
    .unwrap();
    
    conn
}

pub fn close_database(conn: Connection) {
    conn.close();
}
