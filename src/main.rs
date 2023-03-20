use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use deadpool_postgres::Pool;
use dotenv::dotenv;
use handlers::{
    add_message_handler, create_chat_handler, get_chats_handler, get_messages_by_chat_id_handler,get_messages_by_chat_id_endpoint,
    get_messages_handler, update_chat_name_handler,delete_chat_handler,
};
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_json;
use std::env;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;
extern crate chrono;
extern crate serde;

use crate::config::Config;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

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

impl Error for OpenAIError {}

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
) -> Result<String, Box<dyn Error>> {
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
                model: "gpt-3.5-turbo-0301".to_string(),
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


#[post("/chat/{chat_id}")]
async fn chat(
    chat_id: web::Path<i32>,
    chat_completion: web::Json<ChatPromptRequestBody>,
    db_pool: web::Data<deadpool_postgres::Pool>,
) -> impl Responder {
    let chat_id_value = chat_id.into_inner();

    // Get the last message from the request
    if let Some(message) = chat_completion.messages.last() {
        // Convert the message to the Message format
        let new_message = models::Message {
            id: None,
            created_on: Utc::now(),
            role: message.role.clone(),
            content: message.content.clone(),
            chat_id_relation: chat_id_value,
        };

        // Call the add_message_handler function
        let message_result = add_message_handler(db_pool.clone(), web::Json(new_message)).await;
        println!("Message result: {:?}", message_result);
    }

    let messages = match get_messages_by_chat_id_handler(db_pool.clone(), chat_id_value).await {
        Ok(messages) => messages,
        Err(e) => {
            eprintln!("Error getting messages: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let openai_messages = messages
        .into_iter()
        .map(|msg| ChatCompletionMessage {
            role: msg.role,
            content: msg.content,
        })
        .collect::<Vec<ChatCompletionMessage>>();

    let chat_url = "https://api.openai.com/v1/chat/completions".to_string();

    println!("In chat");
    println!("Chat completion: {:?}", chat_completion.messages);

    let request = OpenAIRequest::ChatCompletion {
        model: "gpt-3.5-turbo-0301".to_string(),
        messages: &openai_messages,
        temperature: Some(1.5),
    };

    let openai_response =
        match call_openai_api(request, env::var("OPENAI_API_KEY").unwrap(), chat_url).await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error calling OpenAI API: {}", e);
                String::from("Error calling OpenAI API")
            }
        };


    let response_json: serde_json::Value = match serde_json::from_str(&openai_response) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Error getting content")
        .unwrap_or_else(|_| "No summary found.");

    let ai_message = models::Message {
        id: None,
        created_on: Utc::now(),
        role: "assistant".to_string(),
        content: content.to_string(),
        chat_id_relation: chat_id_value,
    };

    let ai_message_result = add_message_handler(db_pool.clone(), web::Json(ai_message)).await;
    println!("AI message result: {:?}", ai_message_result);

    HttpResponse::Ok().body(openai_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();


    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .service(chat)
            .route("/create_chat/{app_user}", web::post().to(create_chat_handler))
            .route("/chats/{app_user}", web::get().to(get_chats_handler))
            .route("/chats/{chat_id}/messages", web::get().to(get_messages_by_chat_id_endpoint))
            .route("/update_chat_name", web::put().to(update_chat_name_handler))
            .route("/delete_chat/{chat_id}", web::delete().to(delete_chat_handler))
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
