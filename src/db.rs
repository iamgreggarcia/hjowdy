use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::errors::MyError;
use crate::models::{Chat, Message};

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
        )
        .await?
        .iter()
        .map(|row| Chat::from_row_ref(row).unwrap())
        .collect::<Vec<Chat>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_messages(client: &Client, message_info: Message) -> Result<Vec<Message>, MyError> {
    let _stmt = include_str!("../sql/get_messages.sql");
    let _stmt = _stmt.replace("$1", &message_info.chat_id_relation.to_string());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&message_info.chat_id_relation])
        .await?
        .iter()
        .map(|row| Message::from_row_ref(row).unwrap())
        .collect::<Vec<Message>>()
        .pop()
        .map(|last_message| {
            let mut messages = Vec::new();
            messages.push(last_message);
            messages.reverse();
            messages
        })
        .ok_or(MyError::NotFound)
}

pub async fn get_messages_by_chat_id(
    client: &Client,
    chat_id: i32,
) -> Result<Vec<Message>, MyError> {
    let stmt = client
        .prepare(include_str!("../sql/get_messages_by_chat_id.sql"))
        .await
        .unwrap();

    let messages = client
        .query(&stmt, &[&chat_id])
        .await?
        .iter()
        .map(|row| Message::from_row_ref(row).unwrap())
        .collect::<Vec<Message>>();

    Ok(messages)
}

pub async fn create_chat(client: &Client, app_user: String) -> Result<Chat, MyError> {
    let _stmt = include_str!("../sql/create_chat.sql");
    let _stmt = _stmt.replace("$1", &app_user);
    let stmt = client.prepare(&_stmt).await.unwrap();
    let row = client.query_one(&stmt, &[]).await?;

    Ok(Chat::from_row_ref(&row)?)
}

pub async fn add_message(client: &Client, message_info: Message) -> Result<Message, MyError> {
    let _stmt = include_str!("../sql/add_message.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let row = client
        .query_one(
            &stmt,
            &[
                &message_info.chat_id_relation,
                &message_info.role,
                &message_info.content,
            ],
        )
        .await?;

    Ok(Message {
        id: row.get(0),
        created_on: row.get(1),
        role: row.get(2),
        content: row.get(3),
        chat_id_relation: row.get(4),
    })
}
