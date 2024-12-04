use tauri::State;

use crate::AppData;

use super::{chat_box_dao::*, ChatBox};

#[tauri::command]
pub fn insert_chat_box_api(state: State<AppData>) -> Result<(), String> {
    let chat_box = ChatBox::default();
    let conn = state.conn.lock().unwrap();
    insert_chat_box(&conn, chat_box).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_chat_box_api(state: State<AppData>, title: String) -> Result<Vec<ChatBox>, String> {
    let conn = state.conn.lock().unwrap();
    list_chat_box(&conn, Some(title)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_chat_box_title_api(
    state: State<AppData>,
    id: String,
    title: String,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    update_chat_box_title(&conn, i64::from_str_radix(&id, 10).unwrap_or_default(), title).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_chat_box_api(state: State<AppData>, id: String) -> Result<(), String> {
    let id = i64::from_str_radix(&id, 10).unwrap_or_default();
    let conn = state.conn.lock().unwrap();
    delete_chat_box(&conn, id).map_err(|e| e.to_string())
}
