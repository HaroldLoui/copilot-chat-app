use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ID_WORKER;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatBox {
    pub id: i64,
    pub title: String,
    pub count: usize,
    #[serde(with = "super::my_date_format")]
    pub create_time: NaiveDateTime,
}

impl Default for ChatBox {
    fn default() -> Self {
        Self {
            id: ID_WORKER.write().get_id(),
            title: "默认对话".to_string(),
            count: 0,
            create_time: Utc::now().naive_local(),
        }
    }
}
