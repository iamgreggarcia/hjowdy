use crate::db::{
    add_message, create_chat, delete_chat, get_chats, get_messages, get_messages_by_chat_id,
    update_chat_name,
};
use crate::errors::MyError;
use crate::models::Message;
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateChatName {
    chat_id: i32,
    new_chat_name: String,
}

pub async fn delete_chat_handler(
    db_pool: web::Data<Pool>,
    chat_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    delete_chat(&client, chat_id.into_inner()).await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_chat_name_handler(
    db_pool: web::Data<Pool>,
    update_chat_info: web::Json<UpdateChatName>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let chat_id = update_chat_info.chat_id;
    let new_chat_name = update_chat_info.new_chat_name.clone();

    update_chat_name(&client, chat_id, new_chat_name).await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_chats_handler(
    app_user: web::Path<i32>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_chat = get_chats(&client, *app_user).await?;

    Ok(HttpResponse::Ok().json(new_chat))
}

pub async fn get_messages_handler(
    message: web::Json<Message>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let message_info: Message = message.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_message = get_messages(&client, message_info).await?;

    Ok(HttpResponse::Ok().json(new_message))
}

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

pub async fn create_chat_handler(
    db_pool: web::Data<Pool>,
    app_user: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    //  let new_chat = create_chat(&client, app_user.to_string()).await?;
    match create_chat(&client, *app_user).await {
        Ok(new_chat) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(new_chat)),
        Err(e) => {
            eprintln!("Error creating chat: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
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
