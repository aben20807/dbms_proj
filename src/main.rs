extern crate gtk;

mod gui {
    pub mod gtk3;
}

mod db {
    pub mod sqlite;
}

fn main() {
    println!("Connecting DB...");
    let conn = db::sqlite::connect();
    println!("Initializing DB...");
    db::sqlite::init_db(&conn);
    println!("GUI start!");
    gui::gtk3::launch(conn);
    println!("GUI end!");
}
