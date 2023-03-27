use crate::db::{add_message, get_messages_by_chat_id};
use crate::errors::MyError;
use crate::models::Message;
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn get_messages_by_chat_id_endpoint(
    chat_id: web::Path<i32>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let chat_id_value = chat_id.into_inner();
    let messages = get_messages_by_chat_id_handler(db_pool.clone(), chat_id_value).await?;
    Ok(HttpResponse::Ok().json(messages))
}

pub async fn get_messages_by_chat_id_handler(
    db_pool: web::Data<Pool>,
    chat_id: i32,
) -> Result<Vec<Message>, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let messages = get_messages_by_chat_id(&client, chat_id).await?;

    Ok(messages)
}

pub async fn add_message_handler(
    db_pool: web::Data<Pool>,
    message: web::Json<Message>,
) -> Result<HttpResponse, Error> {
    let message_info: Message = message.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_message = add_message(&client, message_info).await?;

    Ok(HttpResponse::Ok().json(new_message))
}
