use std::{thread::sleep, time::Duration};

use tauri::{AppHandle, Emitter, State};

use crate::AppData;

use super::{message_dao::*, Message};

#[tauri::command(rename_all = "snake_case")]
pub fn list_message_api(
    state: State<AppData>,
    chat_id: String,
    page_num: usize,
) -> Result<Vec<Message>, String> {
    let conn = state.conn.lock().unwrap();
    let cursor = (page_num - 1) * 10;
    list_message(&conn, i64::from_str_radix(&chat_id, 10).unwrap_or_default(), cursor).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_me_message_api(
    state: State<AppData>,
    chat_id: String,
    content: String,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    let message = Message {
        chat_id: i64::from_str_radix(&chat_id, 10).unwrap_or_default(),
        content,
        ..Message::me()
    };

    insert_message(&conn, message).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_ai_message_api(
    state: State<AppData>,
    chat_id: String,
    content: String,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    let message = Message {
        chat_id: i64::from_str_radix(&chat_id, 10).unwrap_or_default(),
        content,
        ..Message::ai()
    };
    insert_message(&conn, message).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn send_message(
    app: AppHandle,
    state: State<AppData>,
    chat_id: String,
    content: String,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    let message = Message {
        chat_id: i64::from_str_radix(&chat_id, 10).unwrap_or_default(),
        content,
        ..Message::me()
    };
    println!("{:?}", message);
    let r = insert_message(&conn, message).map_err(|e| e.to_string());
    println!("{:?}", r);

    for i in 0..10 {
        if i == 0 {
            let _ = app.emit("chat:message://received", "\n");
        } else if i < 5 {
            let _ = app.emit("chat:message://received", "#");
        } else if i == 5 {
            let _ = app.emit("chat:message://received", " received: ");
        } else {
            let _ = app.emit("chat:message://received", i);
        }
        sleep(Duration::from_millis(500));
    }
    
    Ok(())
}