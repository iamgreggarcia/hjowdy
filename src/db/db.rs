use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, models::{Chat, Messages}};

pub async fn get_chats(client: &Client, chat_info: Chat) -> Result<Chat, MyError> {
    let _stmt = include_str!("../sql/get_chats.sql");
    let _stmt = _stmt.replace("$1", &chat_info.app_user.to_string());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
            &chat_info.app_user,
            &chat_info.created_on,
            &chat_info.chat_id,
            ],
            ).await?
        .iter()
        .map(|row| Chat::from_row_ref(row).unwrap())
        .collect::<Vec<Chat>>()
        .pop()
        .ok_or(MyError::NotFound)

}

pub async fn get_messages(client: &Client, message_info: Message) -> Result<Vec<Message>, MyError> {
    let _stmt = include_str!("../sql/get_messages.sql");
    let _stmt = _stmt.replace("$1", &message_info.chat_id.to_string());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
            &message_info.chat_id,
            ],
            ).await?
        .iter()
        .map(|row| Message::from_row_ref(row).unwrap())
        .collect::<Vec<Message>>()
        .pop()
        .ok_or(MyError::NotFound)
}













