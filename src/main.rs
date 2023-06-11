#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod todolist;

use eframe::egui;
use rusqlite::Connection;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut task = String::new();

    eframe::run_simple_native("My TodoList App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |_ui| {
            let conn = Connection::open("todo.db").expect("Error: Connection to db failed");
            conn.execute(
                "CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                input TEXT NOT NULL
                )",
                [],
            ).expect("Error: Table not created");

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Todo List");
                ui.separator();
                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    let label = ui.label("Task to add: ");
                    ui.text_edit_singleline(&mut task)
                        .labelled_by(label.id);
                });
                if ui.button("Add").clicked() {
                    todolist::add_in_table(&conn, task.clone());
                }

                ui.add_space(15.0);

                let values = todolist::show(&conn);
                let mut sorted_values: Vec<_> = values.iter().collect();
                sorted_values.sort_by_key(|a| a.0);

                for (key, value) in sorted_values{
                    ui.horizontal(|ui| {
                        if ui.button("X").clicked() {
                            todolist::delete_values(&conn, *key)
                        };
                        ui.label(value);
                    });
                }
            })
        });
    })
}