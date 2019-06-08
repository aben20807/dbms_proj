use gtk::{self};
use gtk::prelude::*;
use gtk::{
    CellRendererText, ListStore, TreeView, TreeViewColumn,
};
use rusqlite::types::Value as Value;
use std::sync::Arc;

pub fn launch(conn: rusqlite::Connection) {
    gtk::init().unwrap_or_else(|_| panic!("panic!"));
    let builder = gtk::Builder::new_from_string(include_str!("app.ui"));
    let window: gtk::Window = builder.get_object("app").unwrap();
    let combo: gtk::ComboBoxText = builder.get_object("combo").unwrap();
    let status: gtk::Label = builder.get_object("status").unwrap();
    let view: gtk::TreeView = builder.get_object("view").unwrap();
    // search button
    let search: gtk::Button = builder.get_object("search").unwrap();
    // keyword or search command
    let keyword: gtk::TextBuffer = builder.get_object("keyword_buf").unwrap();
    let mode: gtk::TextBuffer = builder.get_object("mode").unwrap();

    let builder = Arc::new(builder);
    let conn = Arc::new(conn);
    let mode = Arc::new(mode);

    window.set_title("DBMS project");
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let mode1 = mode.clone();
    combo.connect_changed(move |combo| {
        if let None = combo.get_active_text() {
            return;
        } else {
            mode1.set_text(combo.get_active_text().unwrap().as_str());
        }
    });

    let conn2 = conn.clone();
    let builder2 = builder.clone();
    let mode2 = mode.clone();
    search.connect_clicked(move |_| {
        match mode2.get_text(&mode2.get_start_iter(), &mode2.get_end_iter(), false) {
            None => (),
            Some(s) => {
                if s == "SQL" {
                    let keyword_cmd = keyword.get_text(&keyword.get_start_iter(), &keyword.get_end_iter(), false).unwrap();
                    let stmt = crate::db::sqlite::exec_sql(&conn2, keyword_cmd.as_str());
                    match stmt {
                        Ok(mut stmt) => {
                            update_attr_to_view(&builder2, stmt.column_names());
                            let num = stmt.column_count();
                            update_row_to_view(&builder2, &mut stmt, num);
                            status.set_text(format!("success").as_str());
                        }
                        Err(err) => {
                            status.set_text(format!("wrong sql: {}", err).as_str());
                        }
                    }
                } else if s == "ID" {
                    let keyword_cmd = "SELECT id FROM person";
                    let stmt = crate::db::sqlite::exec_sql(&conn2, keyword_cmd);
                    match stmt {
                        Ok(mut stmt) => {
                            update_attr_to_view(&builder2, stmt.column_names());
                            let num = stmt.column_count();
                            update_row_to_view(&builder2, &mut stmt, num);
                            status.set_text(format!("success").as_str());
                        }
                        Err(err) => {
                            status.set_text(format!("wrong sql: {}", err).as_str());
                        }
                    }
                }
            },
        }
        // if mode2.borrow().unwrap() == None {

        // } else if mode2.borrow() == Some("SQL".to_string()) {
        //     let keyword_cmd = keyword.get_text(&keyword.get_start_iter(), &keyword.get_end_iter(), false).unwrap();
        //     let stmt = crate::db::sqlite::exec_sql(&conn2, keyword_cmd.as_str());
        //     match stmt {
        //         Ok(mut stmt) => {
        //             update_attr_to_view(&builder2, stmt.column_names());
        //             let num = stmt.column_count();
        //             update_row_to_view(&builder2, &mut stmt, num);
        //             status.set_text(format!("success").as_str());
        //         }
        //         Err(err) => {
        //             status.set_text(format!("wrong sql: {}", err).as_str());
        //         }
        //     }
        // }
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

    view.set_model(Some(&model));
    view.set_grid_lines(gtk::TreeViewGridLines::Both);
    view.set_headers_visible(true);
    gtk::main();
}

fn update_attr_to_view(builder: &gtk::Builder, attrs: Vec<&str>) {
    let view: gtk::TreeView = builder.get_object("view").unwrap();
    clear_view(&builder);
    for (i, attr) in attrs.iter().enumerate() {
        append_column(&view, i as i32, attr);
    }
    let model = ListStore::new(&[u32::static_type(), String::static_type(), String::static_type()]);
    view.set_model(Some(&model));
}

fn update_row_to_view(builder: &gtk::Builder, stmt: &mut rusqlite::Statement, num: usize) {
     println!("{}", num);
     let person_iter = stmt.query_map(rusqlite::params![], |row| {
         let mut r = Vec::new();
         for i in 0..num {
            r.push(row.get::<_, Value>(i).unwrap());
         }
         println!("{:?}", r);
        Ok(r)
    }).unwrap();
    for it in person_iter {
        println!("{:?}, {}", it, num);
    }
}

fn clear_view (builder: &gtk::Builder) {
    let view: gtk::TreeView = builder.get_object("view").unwrap();
    let rms = view.get_columns();
    for rm in rms.iter() {
        view.remove_column(rm);
    }
}

fn append_column(view: &TreeView, id: i32, title: &str) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    column.set_title(title);
    view.append_column(&column);
}