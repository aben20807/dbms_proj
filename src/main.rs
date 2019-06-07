extern crate gtk;

mod gui {
    pub mod gtk3;
}

mod db {
    pub mod sqlite;
}

fn main() {
    println!("Hello, world!");
    let view: gtk::TreeView = gui::gtk3::launch();
    let mut attrs = Vec::new();
    attrs.push("a");
    attrs.push("b");
    gui::gtk3::update_attr_to_view(&view, attrs);
    db::sqlite::test();
}
