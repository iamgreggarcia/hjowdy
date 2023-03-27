use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "chats")]
pub struct Chat {
    pub chat_id: i32,
    pub app_user: i32,
    pub created_on: DateTime<Utc>,
    pub chat_name: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "messages")]
pub struct Message {
    pub id: Option<i32>,
    pub created_on: DateTime<Utc>,
    pub role: String,
    pub content: String,
    pub chat_id_relation: i32,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "images")]
pub struct Image {
    pub id: i32,
    pub chat_id: i32,
    pub url: String,
    pub created_on: DateTime<Utc>,
}
