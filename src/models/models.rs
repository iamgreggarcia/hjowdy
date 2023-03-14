use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "chats")]
pub struct Chat {
    pub chat_id: i32,
    pub app_user: i32,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "messages")]
pub struct Messages {
    pub id: i32,
    pub created_on: String,
    pub role: String,
    pub content: String,
    pub chat_id_relation: i32,
}

