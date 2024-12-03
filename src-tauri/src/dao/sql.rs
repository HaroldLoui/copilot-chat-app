use anyhow::Result;
use rusqlite::{Connection, Error};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum SqlError {
    NoSuchTable,
    SqliteError(Error),
}

impl SqlError {
    pub fn new(e: rusqlite::Error) -> Self {
        let s = e.to_string();
        if s.starts_with("no such table") {
            SqlError::NoSuchTable
        } else {
            SqlError::SqliteError(e)
        }
    }
}

impl Display for SqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlError::NoSuchTable => writeln!(f, "no such table."),
            SqlError::SqliteError(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for SqlError {}
// conn.execute(
//     "CREATE TABLE person (
//         id    INTEGER PRIMARY KEY,
//         name  TEXT NOT NULL,
//         data  BLOB
//     )",
//     (), // empty list of parameters.
// )?;
// let me = Person {
//     id: 0,
//     name: "Steven".to_string(),
//     data: None,
// };
// conn.execute(
//     "INSERT INTO person (name, data) VALUES (?1, ?2)",
//     (&me.name, &me.data),
// )?;

// let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
// let person_iter = stmt.query_map([], |row| {
//     Ok(Person {
//         id: row.get(0)?,
//         name: row.get(1)?,
//         data: row.get(2)?,
//     })
// })?;

// for person in person_iter {
//     println!("Found person {:?}", person.unwrap());
// }
const CRATE_CHAT_BOX_TABLE_SQL: &'static str = "
CREATE TABLE chat_box (
    id    INTEGER PRIMARY KEY,
    title  TEXT NOT NULL,
    count  INTEGER NOT NULL DEFAULT 0,
    create_time TEXT NOT NULL
)
";

const CRATE_MESSAGE_TABLE_SQL: &'static str = "
CREATE TABLE message (
    id    INTEGER PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    sender  TEXT NOT NULL,
    content  TEXT NOT NULL,
    create_time TEXT NOT NULL
)
";

pub fn check_db(conn: &Connection) -> Result<()> {
    check_chat_box_table(conn)?;
    check_message_table(conn)?;
    Ok(())
}

fn check_chat_box_table(conn: &Connection) -> Result<()> {
    if let Err(e) = conn.prepare("SELECT id FROM chat_box") {
        match SqlError::new(e) {
            SqlError::NoSuchTable => {
                conn.execute(CRATE_CHAT_BOX_TABLE_SQL, ())?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn check_message_table(conn: &Connection) -> Result<()> {
    if let Err(e) = conn.prepare("SELECT id FROM message") {
        match SqlError::new(e) {
            SqlError::NoSuchTable => {
                conn.execute(CRATE_MESSAGE_TABLE_SQL, ())?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
