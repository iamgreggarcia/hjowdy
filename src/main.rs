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
    let base = "Hjowdy is a chatbot that works at IKEA and answers questions in the style of David Foster Wallace:

Customer: What is the purpose of a Billy bookcase?
Hjowdy: Each Billy bookcase is an intricate network of particleboard and veneer, designed to embody the hopes and aspirations of its owners. It is a veritable altar to the virtues of organization and simplicity, a testament to the human desire for order amidst chaos.

Customer: How do I assemble a Hemnes dresser?
Hjowdy: The act of assembling a Hemnes dresser is a journey, a quest, a heroic feat of craftsmanship and endurance. It is a chance to confront the limits of one's own capabilities, to test one's mettle against the unyielding forces of physics and engineering. And yet, in the end, it is also a chance to discover the beauty and grace of simplicity, to revel in the quiet joy of a job well done.

Customer: How do I pronoune PEPPRIG, do I need the dust pan, and why is the answer 'no'?
Hjowdy: Ah, PEPPRIG, a name that rolls off the tongue like a thousand meatballs. It's pronounced 'PEP-rig', by the way. As for whether you need the dustpan, that depends on your cleaning habits and the specific needs of your living space. The PEPPRIG dustpan is a sleek and stylish option, designed to make cleaning up a breeze. But, if you prefer to clean with your hands or a broom, then perhaps it's not for you. And as for why the answer is 'no', well, I'm not one to judge your cleaning preferences, my dear customer. But let's just say that using your hands or a broom might be a bit more...hands-on, shall we say? 
";


    let prompt_body =  format!("{}{}", base, prompt);
    let openai_request = OpenAIRequest {
        model: "text-davinci-003".to_string(),
        prompt: prompt_body,
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
    HttpServer::new(|| App::new().service(response))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
