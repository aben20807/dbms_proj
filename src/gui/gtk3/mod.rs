use gtk::{self};
use gtk::prelude::*;
use gtk::{
    CellRendererText, ListStore, TreeView, TreeViewColumn,
};

use std::sync::Arc;

fn append_column(view: &TreeView, id: i32, title: &str) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    column.set_title(title);
    view.append_column(&column);
}

pub fn launch(conn: rusqlite::Connection) {
    gtk::init().unwrap_or_else(|_| panic!("panic!"));
    let builder = gtk::Builder::new_from_string(include_str!("app.ui"));
    let window: gtk::Window = builder.get_object("app").unwrap();
    let combo: gtk::ComboBox = builder.get_object("combobox").unwrap();
    let view: gtk::TreeView = builder.get_object("view").unwrap();
    let search: gtk::Button = builder.get_object("search").unwrap();

    let conn = Arc::new(conn);
    let conn1 = conn.clone();
    let conn2 = conn.clone();

    window.set_title("DBMS project");
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    combo.connect_changed(move |_| {
        let mut attrs = Vec::new();
        attrs.push("a");
        attrs.push("b");
        update_attr_to_view(&builder, attrs);
        crate::db::sqlite::exec_sql(&conn1, "SELECT id, name, data FROM person");
    });

    search.connect_clicked(move |_| {
        crate::db::sqlite::exec_sql(&conn2, "SELECT id, name, data FROM person");
    });

    append_column(&view, 0, "id");
    append_column(&view, 1, "name");
    append_column(&view, 2, "phone");
    let model = ListStore::new(&[u32::static_type(), String::static_type(), String::static_type()]);
    // Filling up the tree view.
    let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
    let phone = &["09", "08", "07", "06", "02", "006"];
    for (i, entry) in entries.iter().enumerate() {
        model.insert_with_values(None, &[0, 1, 2], &[&(i as u32 + 1), &entry, &phone[i]]);
    }
    // for (i, entry) in phone.iter().enumerate() {
    //     model.insert_with_values(None, &[0, 1], &[&(i as u32 + 1), &entry]);
    // }
    view.set_model(Some(&model));
    view.set_grid_lines(gtk::TreeViewGridLines::Both);
    view.set_headers_visible(true);
    gtk::main();
}

pub fn update_attr_to_view(builder: &gtk::Builder, attrs: Vec<&str>) {
    let view: gtk::TreeView = builder.get_object("view").unwrap();
    // view.clear();
    let rms = view.get_columns();
    for rm in rms.iter() {
        view.remove_column(rm);
    }
    for (i, attr) in attrs.iter().enumerate() {
        append_column(&view, i as i32, attr);
    }
    let model = ListStore::new(&[u32::static_type(), String::static_type(), String::static_type()]);
    view.set_model(Some(&model));
}