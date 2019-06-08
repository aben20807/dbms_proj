extern crate gtk;

mod gui {
    pub mod gtk3;
}

mod db {
    pub mod sqlite;
}

fn main() {
    println!("GUI start!");
    let conn = db::sqlite::connect();
    db::sqlite::create_tables(&conn);
    gui::gtk3::launch(conn);
    println!("GUI end!");
}
