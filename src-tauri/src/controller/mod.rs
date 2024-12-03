mod chat_box_controller;
mod message_controller;

pub use crate::dao::{chat_box_dao, message_dao};
pub use crate::entity::{ChatBox, Message};

use chat_box_controller::*;
use message_controller::*;

pub use message_controller::send_message;

use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

pub fn chat_box_dao_apis<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("chat_box_controller")
        .invoke_handler(tauri::generate_handler![
            insert_chat_box_api,
            list_chat_box_api,
            update_chat_box_title_api,
            delete_chat_box_api,
        ])
        .build()
}

pub fn message_dao_apis<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("message_controller")
        .invoke_handler(tauri::generate_handler![
            list_message_api,
            insert_me_message_api,
            insert_ai_message_api,
        ])
        .build()
}
