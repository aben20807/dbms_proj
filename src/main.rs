extern crate gtk;

mod gui {
    pub mod gtk3;
}

mod db {
    pub mod sqlite;
}

fn main() {
    println!("Hello, world!");
    let conn = db::sqlite::connect();
    db::sqlite::create_tables(&conn);
    gui::gtk3::launch(conn);
    // println!("Hello, world!!!!");
    // let mut attrs = Vec::new();
    // attrs.push("a");
    // attrs.push("b");
    // gui::gtk3::update_attr_to_view(&builder, attrs);
    // db::sqlite::test();
}
