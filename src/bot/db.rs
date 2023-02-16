use rusqlite::{Connection, OpenFlags};

pub fn init_database() -> Connection {
    Connection::open_with_flags(
        "titus.db",
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )
    .unwrap()
}

pub fn close_database(conn: Connection) {
    conn.close();
}
