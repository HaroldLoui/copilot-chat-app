mod controller;
mod dao;
mod entity;

use controller::send_message;
use static_init::dynamic;

use dao::sql::check_db;
use snowflake::SnowflakeIdBucket;
use std::sync::Mutex;

use rusqlite::Connection;
use tauri::{App, Manager};

#[dynamic]
pub static mut ID_WORKER: SnowflakeIdBucket = SnowflakeIdBucket::new(1, 1);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup)
        .plugin(tauri_plugin_shell::init())
        .plugin(controller::chat_box_dao_apis())
        .plugin(controller::message_dao_apis())
        .invoke_handler(tauri::generate_handler![send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AppData {
    pub conn: Mutex<Connection>,
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_path = app.path();
    let resource_path = app_path.resource_dir()?;
    let db_path = resource_path.join("data.db");
    let conn = Connection::open(db_path.clone())?;
    check_db(&conn)?;
    app.manage(AppData {
        conn: Mutex::new(conn),
    });
    Ok(())
}
