use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_json;
use std::env;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use chat_history::ChatHistory;

mod chat_history;

#[derive(Debug, Deserialize, Clone)]  
struct ChatPromptRequestBody {
    messages: Vec<ChatCompletionMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct PromptRequestBody {
    prompt: String,
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
struct TextRequestBody {
    model: String,
    prompt: String,
    max_tokens: usize,
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
    TextCompletionPrompt {
        model: String,
        prompt: String,
        max_tokens: usize,
    },
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
            OpenAIRequest::TextCompletionPrompt {
                model,
                prompt,
                max_tokens,
            } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("model", model)?;
                map.serialize_entry("prompt", prompt)?;
                map.serialize_entry("max_tokens", max_tokens)?;
                map.end()
            }
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

async fn call_openai_api(input: OpenAIRequest<'_>, api_key: String,url: String) -> Result<String, Box<dyn Error>>
{
    // Print that we are in the function
    println!("In call_openai_api");

    let req_body = serde_json::to_string(&input)?;
    let client = Client::new();

    // Print the request body
    println!("Request body: {:?}", input);
    println!("Request body: {:?}", req_body);
    
    let request_body = match input {
        OpenAIRequest::TextCompletionPrompt {
            model: _,
            prompt,
            max_tokens
        } => {
            let prompt = TextRequestBody {
                model: "text-davinci-003".to_string(),
                prompt,
                max_tokens,
            };
            serde_json::to_vec(&prompt)?
        }
        OpenAIRequest::ChatCompletion {
            model: _,
            messages,
            temperature: _,
        } => {
            let prompt = ChatRequestBody {
                model: "gpt-3.5-turbo-0301".to_string(),
                messages: messages.clone(),
                temperature: Some(1.5),
                max_tokens: Some(300),
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

#[post("/text_completion_prompt")]
async fn text_completion_prompt(prompt_body: web::Json<PromptRequestBody>) -> impl Responder {
    let text_url = "https://api.openai.com/v1/completions".to_string();
    // Print that we are in the function
    println!("In text_completion_prompt");

    let request = OpenAIRequest::TextCompletionPrompt {
        model: "text-davinci-003".to_string(),
        prompt: prompt_body.prompt.clone(),
        max_tokens: 300,
    };

    let openai_response = match call_openai_api(request, env::var("OPENAI_API_KEY").unwrap(), text_url).await
    {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error calling OpenAI API: {}", e);
            String::from("Error calling OpenAI API")
        }
    };

    HttpResponse::Ok().body(openai_response)
}

#[post("/chat")]
async fn chat(chat_completion: web::Json<ChatPromptRequestBody>, chat_history: web::Data<Arc<Mutex<ChatHistory>>>,) -> impl Responder {

    let mut messages = Vec::new();
    let mut history = chat_history.lock().unwrap();

    for message in history.get_messages() {
        messages.push(message.clone());
    }
    
    // Add the new message to the chat_history
    for message in chat_completion.clone().messages {
        messages.push(message.clone());
        history.add_message(message);
    }

    let chat_url = "https://api.openai.com/v1/chat/completions".to_string();

    println!("In chat");
    println!("Chat completion: {:?}", chat_completion.messages);

    let request = OpenAIRequest::ChatCompletion {
        model: "gpt-3.5-turbo-0301".to_string(),
        messages: &messages,
        temperature: Some(1.5),
    };

    let openai_response = match call_openai_api(request, env::var("OPENAI_API_KEY").unwrap(), chat_url).await
    {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error calling OpenAI API: {}", e);
            String::from("Error calling OpenAI API")
        }
    };

    HttpResponse::Ok().body(openai_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_history = web::Data::new(Arc::new(Mutex::new(ChatHistory::new())));

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .app_data(chat_history.clone())
            .wrap(Cors::permissive())
            .service(text_completion_prompt)
            .service(chat)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
