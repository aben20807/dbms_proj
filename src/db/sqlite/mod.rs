use rusqlite::{params, Connection, Result};

mod init;

pub fn connect() -> rusqlite::Connection {
    let _ = std::fs::create_dir_all("./resources/");
    let conn = Connection::open("./resources/mydb.sqlite").unwrap();
    conn
}

pub fn init_db(conn: &rusqlite::Connection) {
    println!("Creating tables...");
    init::create_tables(conn);
    println!("Inserting data...");
    init::insert_init_data(conn);
}

pub fn drop_db(conn: &rusqlite::Connection) {
    println!("Droping DB...");
    // Drop tables
    conn.execute("DROP TABLE IF EXISTS member", params![]).unwrap();
    conn.execute("DROP TABLE IF EXISTS movie", params![]).unwrap();
    conn.execute("DROP TABLE IF EXISTS category", params![]).unwrap();
    conn.execute("DROP TABLE IF EXISTS room", params![]).unwrap();
    conn.execute("DROP TABLE IF EXISTS building", params![]).unwrap();
    conn.execute("DROP TABLE IF EXISTS like", params![]).unwrap();
}

pub fn exec_sql<'a>(conn: &'a rusqlite::Connection, command: &str) -> Result<rusqlite::Statement<'a>> {
    conn.prepare(command)
}