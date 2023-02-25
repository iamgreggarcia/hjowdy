use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt;

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

    let openai_request = OpenAIRequest {
        model: "text-davinci-003".to_string(),
        prompt: prompt,
        max_tokens: 50,
        n: 1,
        temperature: 0.7,
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

#[get("/Response/{request_prompt}")]
async fn response(request_prompt:web::Path<String>) -> impl Responder {
    let openai_response = match call_openai_api(request_prompt.as_str().to_string()).await {
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
