use rusqlite::{params, Connection, Result};

mod init;

pub fn connect() -> rusqlite::Connection {
    let conn = Connection::open("./src/db/sqlite/mydb.sqlite").unwrap();
    conn
}

pub fn init_db(conn: &rusqlite::Connection) {
    println!("Creating tables...");
    init::create_tables(conn);
    println!("Inserting data...");
    init::insert_init_data(conn);
}

pub fn drop_db(conn: &rusqlite::Connection) {
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

// pub fn test() -> Result<()> {
//     // let conn = Connection::open_in_memory()?;
//     let conn = Connection::open("./src/db/sqlite/mydb.sqlite")?;

//     conn.execute(
//         "CREATE TABLE person (
//                   id              INTEGER PRIMARY KEY,
//                   name            TEXT NOT NULL,
//                   data            BLOB
//                   )",
//         params![],
//     )?;
//     let me = Person {
//         id: 0,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute(
//         "INSERT INTO person (name, data)
//                   VALUES (?1, ?2)",
//         params![me.name, me.data],
//     )?;

//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map(params![], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person {:?}", person.unwrap());
//     }
//     Ok(())
// }