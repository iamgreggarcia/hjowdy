use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt;

// Add chat completion resonse struct 
#[derive(Debug, Deserialize)]
struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    choices: Vec<ChatCompletionChoice>,
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    prompt: String,
    max_tokens: usize,
    n: usize,
    temperature: f32,
}

#[derive(Debug)]
struct OpenAIError(String);

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for OpenAIError {}

async fn call_openai_api(prompt: String) -> Result<String, Box<dyn Error>> {
    let openai_url = "https://api.openai.com/v1/completions";
    // Set model to your fine-tuned or preferred model. See https://beta.openai.com/docs/developer-quickstart for more information.
    // Note: if using e.g. text-davinci-003, consider making a base prompt to warm up the model.
    // The append the user prompt to the base prompt
    let model = "davinci:ft-personal-2023-02-26-04-40-43";


    let openai_request = OpenAIRequest {
        model: model.to_string(),
        prompt:prompt,
        max_tokens: 300,
        n: 1,
        temperature: 0.9,
    };

    let openai_api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    let other_response: reqwest::Response = client
        .post(openai_url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", openai_api_key))?,
        )
        .json(&openai_request)
        .send()
        .await?;

    let openai_response = other_response.json::<OpenAIResponse>().await?;
    let mut response_text = String::new();
    for choice in openai_response.choices {
        response_text += &choice.text;
    }

    Ok(response_text)
}


#[post("/response")]
async fn response(
    body: web::Either<web::Json<PromptRequestBody>, web::Json<ChatCompletion>>,
) -> impl Responder {
    let prompt = match body {
        web::Either::Left(prompt_body) => prompt_body.prompt.clone(),
        web::Either::Right(chat_completion) => chat_completion
            .choices
            .get(0)
            .and_then(|choice| Some(choice.message.content.clone()))
            .unwrap_or_else(String::new),
    };

    let openai_response = match call_openai_api(prompt).await {
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
    HttpServer::new(|| App::new().service(response))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
