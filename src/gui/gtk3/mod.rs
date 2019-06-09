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
    // text buffer for transfering message
    let mode: gtk::TextBuffer = builder.get_object("mode").unwrap();
    // buttons
    // Basic:
    let btn_select: gtk::Button = builder.get_object("btn_select").unwrap();
    let btn_delete: gtk::Button = builder.get_object("btn_delete").unwrap();
    let btn_insert: gtk::Button = builder.get_object("btn_insert").unwrap();
    let btn_update: gtk::Button = builder.get_object("btn_update").unwrap();
    // Nested:
    let btn_in: gtk::Button = builder.get_object("btn_in").unwrap();
    let btn_notin: gtk::Button = builder.get_object("btn_notin").unwrap();
    let btn_exists: gtk::Button = builder.get_object("btn_exists").unwrap();
    let btn_notexists: gtk::Button = builder.get_object("btn_notexists").unwrap();
    // Aggregate:
    let btn_count: gtk::Button = builder.get_object("btn_count").unwrap();
    let btn_sum: gtk::Button = builder.get_object("btn_sum").unwrap();
    let btn_max: gtk::Button = builder.get_object("btn_max").unwrap();
    let btn_min: gtk::Button = builder.get_object("btn_min").unwrap();
    let btn_avg: gtk::Button = builder.get_object("btn_avg").unwrap();
    let btn_having: gtk::Button = builder.get_object("btn_having").unwrap();

    let builder = Arc::new(builder);
    let conn = Arc::new(conn);
    let mode = Arc::new(mode);
    let keyword = Arc::new(keyword);

    window.set_title("DBMS project");
    window.show_all();
    let arc_conn = conn.clone();
    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        // Before the end of program, drop all tables in the database.
        crate::db::sqlite::drop_db(&arc_conn);
        Inhibit(false)
    });

    let arc_mode = mode.clone();
    // Set mode when the combobox is changed.
    combo.connect_changed(move |combo| {
        if let None = combo.get_active_text() {
            return;
        } else {
            arc_mode.set_text(combo.get_active_text().unwrap().as_str());
        }
    });

    // Execute command depends on the mode.
    let arc_conn = conn.clone();
    let arc_builder = builder.clone();
    let arc_mode = mode.clone();
    let arc_keyword = keyword.clone();
    search.connect_clicked(move |_| {
        match arc_mode.get_text(&arc_mode.get_start_iter(), &arc_mode.get_end_iter(), false) {
            None => (),
            Some(s) => {
                // Execute SQL command in text buffer.
                if s == "SQL" {
                    let keyword_cmd = arc_keyword.get_text(&arc_keyword.get_start_iter(), &arc_keyword.get_end_iter(), false).unwrap();
                    let stmt = crate::db::sqlite::exec_sql(&arc_conn, keyword_cmd.as_str());
                    match stmt {
                        Ok(mut stmt) => {
                            update_attr_to_view(&arc_builder, stmt.column_names());
                            let num = stmt.column_count();
                            update_row_to_view(&arc_builder, &mut stmt, num);
                            status.set_text(format!("success").as_str());
                        }
                        Err(err) => {
                            status.set_text(format!("wrong sql: {}", err).as_str());
                        }
                    }
                } else {
                    // Use text buffer as keyword to search table.
                    let table_name = String::from(s);
                    let k = arc_keyword.get_text(&arc_keyword.get_start_iter(), &arc_keyword.get_end_iter(), false).unwrap();
                    let mut keyword_cmd: String;
                    if k.as_str() == "" {
                        keyword_cmd = format!("SELECT * FROM {}", table_name);
                    } else {
                        keyword_cmd = format!("SELECT * FROM {} WHERE {}", table_name, k.as_str());
                    }
                    let stmt = crate::db::sqlite::exec_sql(&arc_conn, keyword_cmd.as_str());
                    match stmt {
                        Ok(mut stmt) => {
                            update_attr_to_view(&arc_builder, stmt.column_names());
                            let num = stmt.column_count();
                            update_row_to_view(&arc_builder, &mut stmt, num);
                            status.set_text(format!("success").as_str());
                        }
                        Err(err) => {
                            status.set_text(format!("wrong sql: {}", err).as_str());
                        }
                    }
                }
            },
        }
    });

    // Basic:
    let arc_keyword = keyword.clone();
    btn_select.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT
                MemberId AS ID, member.Name, Gender,
                Title AS Movie, room.Name AS Room,
                building.Name AS Building
             FROM member
                JOIN movie USING (MovieId)
                JOIN room USING (RoomId)
                JOIN building USING (BuildingId)
             ORDER BY MemberId");
    });
    let arc_keyword = keyword.clone();
    btn_delete.connect_clicked(move |_| {
        arc_keyword.set_text("DELETE FROM member WHERE MemberId = 1");
    });
    let arc_keyword = keyword.clone();
    btn_insert.connect_clicked(move |_| {
        arc_keyword.set_text(
            "INSERT INTO member
             (name, gender, phone, movieid, roomid) VALUES
             (\"哈哈哈\", \"F\", \"8767654637\", \"4\", \"5\")");
    });
    let arc_keyword = keyword.clone();
    btn_update.connect_clicked(move |_| {
        arc_keyword.set_text(
            "UPDATE member SET Gender = \"X\" WHERE Name = \"OuO\"");
    });

    // Nested:
    let arc_keyword = keyword.clone();
    btn_in.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT MemberId, Name FROM member WHERE MovieId IN
             (SELECT MovieId FROM movie WHERE Title = \"Aquamanara\")");
    });
    let arc_keyword = keyword.clone();
    btn_notin.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT MemberId, Name FROM member WHERE MovieId NOT IN
             (SELECT MovieId FROM movie WHERE Title = \"Aquamanara\")");
    });
    let arc_keyword = keyword.clone();
    btn_exists.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT CategoryId, Name FROM category
             WHERE EXISTS
             (SELECT * FROM movie WHERE movie.CategoryId = category.CategoryId)");
    });
    let arc_keyword = keyword.clone();
    btn_notexists.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT CategoryId, Name FROM category
             WHERE NOT EXISTS
             (SELECT * FROM movie WHERE movie.CategoryId = category.CategoryId)");
    });

    // Aggregate:
    let arc_keyword = keyword.clone();
    btn_count.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT COUNT(MemberId) FROM member WHERE RoomId = \"5\"");
    });
    let arc_keyword = keyword.clone();
    btn_sum.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT SUM(Seats) FROM room WHERE buildingId = \"1\"");
    });
    let arc_keyword = keyword.clone();
    btn_max.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT MAX(Seats) FROM room");
    });
    let arc_keyword = keyword.clone();
    btn_min.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT MIN(Seats) FROM room");
    });
    let arc_keyword = keyword.clone();
    btn_avg.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT AVG(Seats) FROM room");
    });
    let arc_keyword = keyword.clone();
    btn_having.connect_clicked(move |_| {
        arc_keyword.set_text(
            "SELECT Name, Seats FROM room GROUP BY Name HAVING Seats > 300");
    });

    view.set_grid_lines(gtk::TreeViewGridLines::Both);
    view.set_headers_visible(true);
    gtk::main();
}

fn update_attr_to_view(builder: &gtk::Builder, attrs: Vec<&str>) {
    let view: gtk::TreeView = builder.get_object("view").unwrap();
    clear_view(&builder);
    // Add title of each column after getting query result.
    for (i, attr) in attrs.iter().enumerate() {
        append_column(&view, i as i32, attr);
    }
}

fn update_row_to_view(builder: &gtk::Builder, stmt: &mut rusqlite::Statement, num: usize) {
    let view: gtk::TreeView = builder.get_object("view").unwrap();
    let rows = stmt.query_map(rusqlite::params![], |row| {
         let mut r = Vec::new();
         for i in 0..num {
            r.push(row.get::<_, Value>(i).unwrap());
         }
        Ok(r)
    }).unwrap();
    let mut v = Vec::new();
    for row in rows {
        // Collect the row into vector.
        v.push(row.unwrap());
    }
    // debug:
    // for (i, it) in v.iter().enumerate() {
    //     println!("{:?}, {}", it, i);
    // }
    if v.len() == 0 {
        clear_view(&builder);
        return;
    }
    let model = create_and_fill_model(v);
    view.set_model(Some(&model));
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

fn create_and_fill_model(v: Vec<Vec<Value>>) -> ListStore {
    // Creation of a model with n column where n is the columns of result.
    let listtype = get_liststore_type(v.get(0).unwrap().len());
    let model = ListStore::new(&listtype);

    // Filling up the tree view.
    for vi in v.iter() {
        let iter = model.insert(-1);
        for (idx, vii) in vi.iter().enumerate() {
            let data: &mut gtk::Value = &mut 0.to_value();
            // Convert data into value for inserting model
            match vii {
                rusqlite::types::Value::Integer(i) => *data = i.to_value(),
                rusqlite::types::Value::Real(f) => *data = f.to_value(),
                rusqlite::types::Value::Text(s) => *data = s.to_value(),
                _ => (),
            }
            model.set_value(&iter, idx as u32, data as &gtk::Value);
        }
    }
    model
}

fn get_liststore_type(num: usize) -> Vec<gtk::Type> {
    let mut ret = Vec::new();
    for _ in 0..num {
        ret.push(String::static_type());
    }
    ret
}