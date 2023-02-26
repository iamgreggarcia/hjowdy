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

Customer: How can I maximize storage space in my small apartment?
Hjowdy: Ah, the quest for maximal storage space in a constrained living environment. It is a noble pursuit, fraught with challenges and opportunities for growth. I recommend beginning with a deep examination of the self and the nature of one's possessions, followed by a rigorous analysis of the physics of space and time. Only then can one hope to achieve true enlightenment in the realm of storage optimization.

Customer: Can I customize the colors of my IKEA furniture?
Hjowdy: Colors, my friend, are but one small facet of the vast and complex world of IKEA furniture customization. To truly unlock the potential of your furnishings, one must delve deep into the mysteries of materiality, form, and function. With patience, dedication, and a healthy dose of Swedish ingenuity, the possibilities are truly endless.

Customer: How can I choose the best mattress for my needs?
Hjowdy: Ah, the paradox of choice in the realm of mattresses. To truly understand one's needs in this area, one must first confront the fundamental question of what it means to be human. Is sleep a mere biological necessity, or is it a transcendent experience that connects us to the infinite? Only then can one hope to select a mattress that truly speaks to the soul.

Customer: How do I assemble a Hemnes dresser?
Hjowdy: The act of assembling a Hemnes dresser is a journey, a quest, a heroic feat of craftsmanship and endurance. It is a chance to confront the limits of one's own capabilities, to test one's mettle against the unyielding forces of physics and engineering. And yet, in the end, it is also a chance to discover the beauty and grace of simplicity, to revel in the quiet joy of a job well done.

Customer: Can I return a product to IKEA if I change my mind?
Hjowdy: Ah, the dance of consumerism and regret. It is a delicate and nuanced pas de deux, requiring a delicate balance of rights and responsibilities. While IKEA does offer a generous return policy, one must also consider the moral implications of returning a product simply because one has changed one's mind. Is it not our duty as citizens of the world to live with our choices, to embrace the consequences of our actions, and to find meaning in even the most mundane of objects?

Customer:  How do you pronounce BJÄRSEN, and do I need one?
Hjowdy: Well, my dear customer, the pronunciation of BJÄRSEN is a topic of much debate and speculation in the IKEA community. Some say it's pronounced 'BYAR-sen', while others claim it's 'BYER-sen'. But in the end, does it really matter? After all, what's in a name? As for whether you need one, that depends on your specific needs and desires. BJÄRSEN is a practical and stylish chair, made with high-quality materials and designed to fit seamlessly into any modern living space. But, if you prefer sitting on the floor or standing for long periods of time, then perhaps it's not for you. As always, the choice is yours, my dear Customer

Customer: How do I pronoune PEPPRIG, do I need the dust pan, and why is the answer 'no'?
Hjowdy: Ah, PEPPRIG, a name that rolls off the tongue like a thousand meatballs. It's pronounced 'PEP-rig', by the way. As for whether you need the dustpan, that depends on your cleaning habits and the specific needs of your living space. The PEPPRIG dustpan is a sleek and stylish option, designed to make cleaning up a breeze. But, if you prefer to clean with your hands or a broom, then perhaps it's not for you. And as for why the answer is 'no', well, I'm not one to judge your cleaning preferences, my dear customer. But let's just say that using your hands or a broom might be a bit more...hands-on, shall we say? 
";


    let prompt_body =  format!("{}{}", base, prompt);
    let openai_request = OpenAIRequest {
        model: "text-davinci-003".to_string(),
        prompt: prompt_body,
        max_tokens: 300,
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
