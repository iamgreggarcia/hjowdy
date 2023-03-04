use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    choices: Vec<ChatCompletionChoice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
    finish_reason: String,
    index: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChatCompletionMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct PromptRequestBody {
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    text: String,
}

#[derive(Debug)]
struct OpenAIError(String);

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum OpenAIInput {
    Prompt(String),
    ChatCompletion(ChatCompletion),
}

#[derive(Debug, Serialize)]
struct OpenAIChatCompletionPrompt {
    messages: Vec<ChatCompletionMessage>,
    engine: String,
    temperature: f32,
    max_tokens: usize,
    stop: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequestPrompt {
    model: String,
    prompt: String,
    max_tokens: usize,
    n: usize,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct OpenAIRequestChatCompletion {
    model: String,
    messages: Vec<ChatCompletionMessage>,
    engine: String,
    temperature: f32,
    max_tokens: usize,
    stop: String,
}

impl Error for OpenAIError {}

#[derive(Debug, Serialize)]
enum OpenAIRequest {
    TextCompletionPrompt {
        model: String,
        prompt: String,
        max_tokens: usize,
        n: usize,
        temperature: f32,
    },
    ChatCompletion {
        model: String,
        messages: Vec<ChatCompletionMessage>,
        temperature: f32,
    },
}

async fn call_openai_api(input: OpenAIRequest, api_key: String) -> Result<String, Box<dyn Error>> {
    let openai_url = "https://api.openai.com/v1/completions";
    let openai_api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    // Print the request body
    println!("Request body: {:?}", input);

    let other_response = client
        .post(openai_url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        )
        .json(&input)
        .send()
        .await?;

    let response_text = other_response.text().await?;

    Ok(response_text)
}

#[post("/text_completion_prompt")]
async fn text_completion_prompt(prompt_body: web::Json<PromptRequestBody>) -> impl Responder {
    let request = OpenAIRequest::TextCompletionPrompt {
        model: "text-davinci-003".to_string(),
        prompt: prompt_body.prompt.clone(),
        max_tokens: 300,
        n: 1,
        temperature: 0.9,
    };

    let openai_response = match call_openai_api(request, env::var("OPENAI_API_KEY").unwrap()).await
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
async fn chat(chat_completion: web::Json<ChatCompletion>) -> impl Responder {
    let mut messages = vec![];

    for choice in &chat_completion.choices {
        messages.push(ChatCompletionMessage {
            role: "user".to_string(),
            content: choice.message.content.clone(),
        });
        messages.push(ChatCompletionMessage {
            role: "assistant".to_string(),
            content: "".to_string(),
        });
    }
    let request = OpenAIRequest::ChatCompletion {
        model: "gpt-3.5-turbo-0301".to_string(),
        messages: messages,
        temperature: 0.9,
    };

    let openai_response = match call_openai_api(request, env::var("OPENAI_API_KEY").unwrap()).await
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
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(text_completion_prompt)
            .service(chat)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
