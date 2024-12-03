use anyhow::Result;
use rusqlite::{named_params, Connection};

use crate::entity::Message;

use super::chat_box_dao::increament_chat_box_count;

pub fn insert_message(conn: &Connection, message: Message) -> Result<()> {
    conn.execute(
        "INSERT INTO message(id, chat_id, sender, content, create_time) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &message.id,
            &message.chat_id,
            &message.sender,
            &message.content,
            &message.create_time,
        ),
    )?;
    increament_chat_box_count(conn, message.chat_id)?;
    Ok(())
}

pub fn list_message(conn: &Connection, chat_id: i64, cursor: usize) -> Result<Vec<Message>> {
    let sql = "
        SELECT id, chat_id, sender, content, create_time FROM message 
        WHERE chat_id = :chat_id 
        ORDER BY create_time DESC
        LIMIT 10 OFFSET :cursor
    ";
    let mut stmt = conn.prepare(sql)?;
    let iter = stmt.query_map(
        named_params! { ":chat_id": chat_id, ":cursor": cursor },
        |row| {
            Ok(Message {
                id: row.get(0)?,
                chat_id: row.get(1)?,
                sender: row.get(2)?,
                content: row.get(3)?,
                create_time: row.get(4)?,
            })
        },
    )?;
    let mut list = Vec::new();
    for msg in iter {
        if let Ok(msg) = msg {
            list.push(msg);
        }
    }
    list.reverse();
    Ok(list)
}
