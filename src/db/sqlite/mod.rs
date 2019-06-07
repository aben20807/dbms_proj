// use rusqlite::types::ToSql;
use rusqlite::{params, Connection, Result};
// use time::Timespec;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    // time_created: Timespec,
    data: Option<Vec<u8>>,
}

pub fn connect() -> rusqlite::Connection {
    let conn = Connection::open("./src/db/sqlite/mydb.sqlite").unwrap();
    conn
}

pub fn create_tables(conn: &rusqlite::Connection) {
    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        params![],
    ).unwrap();

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data)
                  VALUES (?1, ?2)",
        params![me.name, me.data],
    ).unwrap();
}

pub fn exec_sql(conn: &rusqlite::Connection, command: &str) {
    let mut stmt = conn.prepare(command).unwrap();
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}

pub fn test() -> Result<()> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open("./src/db/sqlite/mydb.sqlite")?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        params![],
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data)
                  VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}