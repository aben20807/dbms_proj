use rusqlite::{params};

pub fn create_tables(conn: &rusqlite::Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS member (
                  MemberId      INTEGER PRIMARY KEY,
                  Name          TEXT NOT NULL,
                  Gender        TEXT NOT NULL,
                  Phone         TEXT NOT NULL,
                  MovieId       INTEGER NOT NULL,
                  RoomId        INTEGER NOT NULL
                  )",
        params![],
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS movie (
                  MovieId       INTEGER PRIMARY KEY,
                  Title         TEXT NOT NULL,
                  RunningTime   TEXT NOT NULL,
                  CategoryId    INTEGER NOT NULL
                  )",
        params![],
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS category (
                  CategoryId    INTEGER PRIMARY KEY,
                  Name          TEXT NOT NULL,
                  Description   TEXT NOT NULL
                  )",
        params![],
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS room (
                  RoomId        INTEGER PRIMARY KEY,
                  Name          TEXT NOT NULL,
                  Seats         TEXT NOT NULL,
                  BuildingId    INTEGER NOT NULL
                  )",
        params![],
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS building (
                  BuildingId    INTEGER PRIMARY KEY,
                  Name          TEXT NOT NULL,
                  Address       TEXT NOT NULL
                  )",
        params![],
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS like (
                  LikeId        INTEGER PRIMARY KEY,
                  MemberId      INTEGER NOT NULL,
                  MovieId       INTEGER NOT NULL,
                  Star          TEXT NOT NULL
                  )",
        params![],
    ).unwrap();

}

pub fn insert_init_data(conn: &rusqlite::Connection) {
    // member
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
    let movie_ids = &["1", "2", "3", "2", "1", "2", "4", "5", "9", "4"];
    let room_ids = &["7", "5", "6", "7", "7", "7", "3", "8", "2", "1"];
    let mut stmt = conn.prepare(
        "INSERT INTO member (name, gender, phone, movieid, roomid) VALUES (?1, ?2, ?3, ?4, ?5)"
        ).unwrap();
    for (i, name) in names.iter().enumerate() {
        stmt.execute(&[name, genders[i], phones[i], movie_ids[i], room_ids[i]]).unwrap();
    }

    // movie
    let titles = &[
        "Avengers: Infinity War",
        "Aquamanara",
        "Solo: A Star Wars Story",
        "Fantastic Beasts: The Crimes of Grindelwald",
        "Black Panther",
        "Fifty Shades Freed",
        "Spider-Man: Into the Spider-Verse",
        "Jurassic World: Fallen Kingdom",
        "Ant-Man and the Wasp",
        "Logistics",
        ];
    let running_times = &[
        "149",
        "143",
        "135",
        "134",
        "134",
        "105",
        "117",
        "128",
        "118",
        "51420",
    ];
    let category_ids = &["1", "2", "1", "2", "1", "3", "4", "1", "5", "10"];
    let mut stmt = conn.prepare(
        "INSERT INTO movie (Title, RunningTime, CategoryId) VALUES (?1, ?2, ?3)"
        ).unwrap();
    for (i, title) in titles.iter().enumerate() {
        stmt.execute(&[title, running_times[i], category_ids[i]]).unwrap();
    }

    // category
    let names = &[
        "Sci-Fi",
        "Fantasy",
        "Romance",
        "Animation",
        "Comedy",
        "Crime",
        "Thriller",
        "Mystery",
        "Horror",
        "Sport",
        ];
    let descriptions = &[
        "a science-fiction novel/story",
        "imaginative literature",
        "a close, usually short relationship of love between two people",
        "moving images",
        "humorous part of a situation",
        "an illegal act",
        "exciting and frightening story",
        "something strange or not known",
        "an extremely strong feeling of fear and shock",
        "a game, competition, or activity needing physical effort and skill",
    ];
    let mut stmt = conn.prepare(
        "INSERT INTO category (Name, Description) VALUES (?1, ?2)"
        ).unwrap();
    for (i, name) in names.iter().enumerate() {
        stmt.execute(&[name, descriptions[i]]).unwrap();
    }

    // room
    let names = &[
        "A0", "A1", "A2", "B0", "B1", "C1", "D1", "D2", "D3", "E1",
        ];
    let seatss = &[
        "200", "400", "400", "350", "300", "200", "400", "200", "10", "20",
    ];
    let building_ids = &["1", "2", "1", "2", "1", "3", "4", "1", "5", "10"];
    let mut stmt = conn.prepare(
        "INSERT INTO room (Name, Seats, BuildingId) VALUES (?1, ?2, ?3)"
        ).unwrap();
    for (i, name) in names.iter().enumerate() {
        stmt.execute(&[name, seatss[i], building_ids[i]]).unwrap();
    }

    // building
    let names = &[
        "Burj Khalifa",
        "Shanghai Tower",
        "Makkah Royal Clock Tower",
        "Ping An Finance Center",
        "Lotte World Tower",
        "One World Trade Center",
        "Guangzhou CTF Finance Centre",
        "CITIC Tower",
        "TAIPEI 101",
        "Shanghai World Financial Center",
        ];
    let addresss = &[
        "Dubai (AE)",
        "Shanghai (CN)",
        "Mecca (SA)",
        "Shenzhen (CN)",
        "Seoul (KR)",
        "New York City (US)",
        "Guangzhou (CN)",
        "Beijing (CN)",
        "Taipei (TW)",
        "Shanghai (CN)",
    ];
    let mut stmt = conn.prepare(
        "INSERT INTO building (Name, Address) VALUES (?1, ?2)"
        ).unwrap();
    for (i, name) in names.iter().enumerate() {
        stmt.execute(&[name, addresss[i]]).unwrap();
    }

    // like
    let member_ids = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
        ];
    let movie_ids = &[
        "1", "1", "2", "2", "2", "3", "3", "3", "5", "8",
        "2", "2", "3", "7", "5", "4", "1", "2", "3", "9",
    ];
    let stars = &[
        "5", "4", "3", "5", "3", "3", "4", "4", "5", "5",
        "5", "5", "5", "4", "3", "4", "5", "4", "3", "4",
        ];
    let mut stmt = conn.prepare(
        "INSERT INTO like (MemberId, MovieId, Star) VALUES (?1, ?2, ?3)"
        ).unwrap();
    for (i, member_id) in member_ids.iter().enumerate() {
        stmt.execute(&[member_id, movie_ids[i], stars[i]]).unwrap();
    }
}