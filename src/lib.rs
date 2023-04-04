mod handlers {
    pub mod chat_handlers;
    pub mod message_handlers;
    pub mod image_handlers;
}
use handlers::chat_handlers;
use handlers::message_handlers;
use handlers::image_handlers;

use actix_cors::Cors;
use actix_web::body::BoxBody;
use actix_web::body::EitherBody;
use actix_web::dev::ServiceFactory;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::{post, web, App, HttpResponse, Responder};
use chrono::Utc;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_json;
use std::error::Error as StdError;
use std::fmt;
extern crate chrono;
extern crate serde;

pub mod config;
pub mod db;
pub mod errors;
pub mod models;

#[derive(Debug, Deserialize, Clone)]
struct ChatPromptRequestBody {
    messages: Vec<ChatCompletionMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessage {
    role: String,
    content: String,
}

#[derive(Debug)]
struct OpenAIError(String);

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize)]
struct ChatRequestBody {
    model: String,
    messages: Vec<ChatCompletionMessage>,
    temperature: Option<f32>,
    max_tokens: Option<usize>,
}

#[derive(Debug, Serialize)]
struct OpenAIRequestChatCompletion {
    model: String,
    messages: Vec<ChatCompletionMessage>,
    temperature: Option<f32>,
    max_tokens: usize,
}

impl StdError for OpenAIError {}

#[derive(Debug, Clone)]
enum OpenAIRequest<'a> {
    ChatCompletion {
        model: String,
        messages: &'a Vec<ChatCompletionMessage>,
        temperature: Option<f32>,
    },
}

impl<'a> Serialize for OpenAIRequest<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OpenAIRequest::ChatCompletion {
                model,
                messages,
                temperature,
            } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("model", model)?;
                map.serialize_entry("messages", messages)?;
                map.serialize_entry("temperature", temperature)?;
                map.end()
            }
        }
    }
}

async fn call_openai_api(
    input: OpenAIRequest<'_>,
    api_key: String,
    url: String,
) -> Result<String, Box<dyn StdError>> {
    // Print that we are in the function
    println!("In call_openai_api");

    let req_body = serde_json::to_string(&input)?;
    let client = Client::new();

    // Print the request body
    println!("Request body: {:?}", input);
    println!("Request body: {:?}", req_body);

    let request_body = match input {
        OpenAIRequest::ChatCompletion {
            model: _,
            messages,
            temperature: _,
        } => {
            let prompt = ChatRequestBody {
                model: "gpt-4".to_string(),
                messages: messages.clone(),
                temperature: Some(1.2),
                max_tokens: Some(1000),
            };
            serde_json::to_vec(&prompt)?
        }
    };

    let other_response = client
        .post(url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        )
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await?;

    let response_text = other_response.text().await?;

    Ok(response_text)
}

async fn add_and_save_message(
    message: &ChatCompletionMessage,
    chat_id_value: i32,
    db_pool: &web::Data<deadpool_postgres::Pool>,
) -> Result<(), Box<dyn StdError>> {
    // Convert the message to the Message format
    let new_message = models::Message {
        id: None,
        created_on: Utc::now(),
        role: message.role.clone(),
        content: message.content.clone(),
        chat_id_relation: chat_id_value,
    };

    // Call the add_message_handler function
    message_handlers::add_message_handler(db_pool.clone(), web::Json(new_message)).await?;

    Ok(())
}

async fn get_consolidated_messages(
    chat_id_value: i32,
    db_pool: &web::Data<deadpool_postgres::Pool>,
    ) -> Result<Vec<ChatCompletionMessage>, Box<dyn StdError>> {
    let messages = message_handlers::get_messages_by_chat_id_handler(db_pool.clone(), chat_id_value).await?;

    Ok(messages
       .into_iter()
       .map(|msg| ChatCompletionMessage {
           role: msg.role,
           content: msg.content,
       })
       .collect())
}

#[post("/chat/{chat_id}")]
async fn chat(
    chat_id: web::Path<i32>,
    chat_completion: web::Json<ChatPromptRequestBody>,
    db_pool: web::Data<deadpool_postgres::Pool>,
    ) -> impl Responder {
    let chat_id_value = chat_id.into_inner();

    // Get the last message from the request
    if let Some(message) = chat_completion.messages.last() {
        if let Err(e) = add_and_save_message(message, chat_id_value, &db_pool).await {
            eprintln!("Error while adding and saving the message: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }

    let openai_messages = match get_consolidated_messages(chat_id_value, &db_pool).await {
        Ok(messages) => messages,
        Err(e) => {
            eprintln!("Error getting messages: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let chat_url = "https://api.openai.com/v1/chat/completions".to_string();

    println!("In chat");
    println!("Chat completion: {:?}", chat_completion.messages);

    let request = OpenAIRequest::ChatCompletion {
        model: "gpt-4".to_string(),
        messages: &openai_messages,
        temperature: Some(1.5),
    };

    let config = config::Config::from_env().unwrap();

    let openai_response = match call_openai_api(request, config.api_key, chat_url).await {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error calling OpenAI API: {}", e);
            return HttpResponse::InternalServerError().body("Error calling OpenAI API");
        }
    };

    let response_json: serde_json::Value = match serde_json::from_str(&openai_response) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return HttpResponse::InternalServerError().body("Error parsing JSON");
        }
    };

    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| {
            eprintln!("Error getting response from OpenAI API");
            HttpResponse::InternalServerError().body("Error getting response from OpenAI API")
        });

    let ai_message = models::Message {
        id: None,
        created_on: Utc::now(),
        role: "assistant".to_string(),
        content: content.unwrap().to_string(),
        chat_id_relation: chat_id_value,
    };

    let ai_message_result = message_handlers::add_message_handler(db_pool.clone(), web::Json(ai_message)).await;
    println!("AI message result: {:?}", ai_message_result);

    HttpResponse::Ok().body(openai_response)
}

pub fn create_app(
    pool: deadpool_postgres::Pool,
    config: config::Config,
    ) -> App<
impl ServiceFactory<
ServiceRequest,
Config = (),
Response = ServiceResponse<EitherBody<BoxBody>>,
Error = Error,
InitError = (),
>,
> {
    App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(config))
        .wrap(Cors::permissive())
        .service(chat)
        .route(
            "/create_chat/{app_user}",
            web::post().to(chat_handlers::create_chat_handler),
            )
        .route("/chats/{app_user}", web::get().to(chat_handlers::get_chats_handler))
        .route(
            "/chats/{chat_id}/messages",
            web::get().to(message_handlers::get_messages_by_chat_id_endpoint),
            )
        .route("/update_chat_name", web::put().to(chat_handlers::update_chat_name_handler))
        .route(
            "/delete_chat/{chat_id}",
            web::delete().to(chat_handlers::delete_chat_handler),
            )
        .route(
            "/images/generations",
            web::post().to(image_handlers::generate_image),
            )
}
