use chrono::{DateTime, Utc};
use deadpool_postgres::{Client, PoolError};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::errors::MyError;
use crate::models::{Chat, Message, Image};

pub async fn delete_chat(client: &Client, chat_id: i32) -> Result<(), MyError> {
    let stmt = client
        .prepare(include_str!("../sql/delete_chat.sql"))
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    client
        .execute(&stmt, &[&chat_id])
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    Ok(())
}

pub async fn get_images_by_chat_id(client: &Client, chat_id: i32) -> Result<Vec<Image>, MyError> {
    let statement = client
        .prepare("SELECT id, chat_id, url, created_on FROM generated_images WHERE chat_id = $1")
        .await?;

    let rows = client.query(&statement, &[&chat_id]).await?;

    let images = rows
        .iter()
        .map(|row| Image {
            id: row.get(0),
            chat_id: row.get(1),
            url: row.get(2),
            created_on: row.get(3),
        })
        .collect::<Vec<Image>>();

    Ok(images)
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

pub async fn get_chats(client: &Client, app_user: i32) -> Result<Vec<Chat>, MyError> {
    let _stmt = include_str!("../sql/get_chats.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    let chats = client
        .query(&stmt, &[&app_user])
        .await?
        .iter()
        .map(|row| Chat::from_row_ref(row).unwrap())
        .collect::<Vec<Chat>>();
    Ok(chats)
}

pub async fn create_chat(client: &Client, app_user: i32) -> Result<Chat, MyError> {
    let _stmt = include_str!("../sql/create_chat.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    let created_on: DateTime<Utc> = Utc::now();

    let row = client
        .query_one(&stmt, &[&app_user, &created_on])
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    Ok(Chat {
        chat_id: row.get(0),
        app_user: row.get(1),
        created_on: row.get(2),
        chat_name: row.get(3),
    })
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

pub async fn update_chat_name(
    client: &Client,
    chat_id: i32,
    new_chat_name: String,
) -> Result<(), MyError> {
    let stmt = client
        .prepare(include_str!("../sql/update_chat_name.sql"))
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    client
        .execute(&stmt, &[&new_chat_name, &chat_id])
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    Ok(())
}


pub async fn save_generated_image(client: &Client, chat_id: i32, url: String) -> Result<Image, MyError> {
    let _stmt = include_str!("../sql/save_generated_image.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    let created_on: DateTime<Utc> = Utc::now();

    let row = client
        .query_one(&stmt, &[&chat_id, &url, &created_on])
        .await
        .map_err(|e| MyError::PoolError(PoolError::Backend(e)))?;

    Ok(Image {
        id: row.get(0),
        chat_id: row.get(1),
        url: row.get(2),
        created_on: row.get(3),
    })
}
