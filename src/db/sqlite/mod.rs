use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub gender: String,
}

pub fn connect() -> rusqlite::Connection {
    let conn = Connection::open("./src/db/sqlite/mydb.sqlite").unwrap();
    conn
}

pub fn init_db(conn: &rusqlite::Connection) {
    println!("Creating tables...");
    create_tables(conn);
    println!("Inserting data...");
    insert_init_data(conn);
}

pub fn drop_db(conn: &rusqlite::Connection) {
    // Drop tables
    conn.execute("DROP TABLE IF EXISTS member", params![]).unwrap();
}

fn create_tables(conn: &rusqlite::Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS member (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  gender          TEXT NOT NULL,
                  phone           TEXT NOT NULL
                  )",
        params![],
    ).unwrap();
}

fn insert_init_data(conn: &rusqlite::Connection) {
    let names = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus", "Ben", "OuO", "XiongJJ", "Hello World"];
    let genders = &["M", "F", "M", "F", "M", "M", "M", "?", "M", "F"];
    let phones = &[
        "2899198718",
        "7912791298",
        "9298188020",
        "2910281820",
        "8208092830",
        "1297180287",
        "2628761980",
        "0283029372",
        "1652765256",
        "2382863829",
    ];
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        gender: "M".to_string(),
    };
    let mut stmt = conn.prepare("INSERT INTO member (name, gender, phone) VALUES (?1, ?2, ?3)").unwrap();
    for (i, name) in names.iter().enumerate() {
        stmt.execute(&[name, genders[i], phones[i]]).unwrap();
        // model.insert_with_values(None, &[0, 1, 2], &[&(i as u32 + 1), &entry, &phone[i]]);
    }
    // conn.execute(
    //     "INSERT INTO person (name, gender)
    //               VALUES (?1, ?2)",
    //     params![me.name, me.gender],
    // ).unwrap();
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