use crate::entity::ChatBox;
use anyhow::Result;
use rusqlite::Connection;

pub fn list_chat_box(conn: &Connection, title: Option<String>) -> Result<Vec<ChatBox>> {
    let title = title.unwrap_or_default();
    let mut sql = String::from("SELECT id, title, count, create_time FROM chat_box");
    if title.len() > 0 {
        sql += &format!(" WHERE title LIKE '%{}%'", title);
    }
    let mut stmt = conn.prepare(&sql)?;
    let chat_iter = stmt.query_map([], |row| {
        Ok(ChatBox {
            id: row.get(0)?,
            title: row.get(1)?,
            count: row.get(2)?,
            create_time: row.get(3)?,
        })
    })?;
    let mut list = Vec::new();
    for chat_res in chat_iter {
        if let Ok(chat) = chat_res {
            list.push(chat);
        }
    }
    Ok(list)
}

pub fn insert_chat_box(conn: &Connection, chat_box: ChatBox) -> Result<()> {
    conn.execute(
        "INSERT INTO chat_box(id, title, create_time)",
        (&chat_box.id, &chat_box.title, &chat_box.create_time),
    )?;
    Ok(())
}

pub fn update_chat_box_title(conn: &Connection, id: i64, title: String) -> Result<()> {
    conn.execute("UPDATE chat_box SET title = ?1 WHERE id = ?2", (&title, id))?;
    Ok(())
}

pub fn increament_chat_box_count(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("UPDATE chat_box SET count = count + 1 WHERE id = ?1", (id,))?;
    Ok(())
}

pub fn delete_chat_box(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM chat_box WHERE id = ?1", (id,))?;
    Ok(())
}
