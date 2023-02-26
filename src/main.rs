use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt;

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
async fn response(body: web::Json<PromptRequestBody>) -> impl Responder {
    let openai_response = match call_openai_api(body.prompt.clone()).await {
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
            .wrap(
                actix_cors::Cors::permissive())
            .service(response)
    })
    .bind("127.0.0.1:8080")?
        .run()
        .await
}
