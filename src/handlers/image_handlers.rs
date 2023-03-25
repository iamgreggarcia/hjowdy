use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use reqwest::header::{HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use reqwest::Client;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct ImageGenerationRequest {
    prompt: String,
    n: Option<u32>,
    size: Option<String>,
    response_format: Option<String>,
}

pub async fn generate_image(
    image_generation_request: web::Json<ImageGenerationRequest>,
    config: web::Data<crate::config::Config>,
) -> Result<impl Responder, actix_web::Error> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/images/generations";


    let request_body = json!({
        "prompt": image_generation_request.prompt,
        "n": image_generation_request.n.unwrap_or(1),
        "size": image_generation_request.size.as_ref().unwrap_or(&"1024x1024".to_string()),
        "response_format": image_generation_request.response_format.as_ref().unwrap_or(&"url".to_string()),
    });
    let response = client
        .post(url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.api_key)).map_err(|e| {
                actix_web::error::InternalError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            })?,
            )
        .header(CONTENT_TYPE, "application/json")
        .body(request_body.to_string())
        .send()
        .await
        .map_err(|e| {
            actix_web::error::InternalError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|e| {
            actix_web::error::InternalError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;
        Ok(HttpResponse::Ok().body(body))
    } else {

        let error_msg = format!("Error in image generation: {}", response.status());
        Err(actix_web::error::InternalError::new(
                error_msg,
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                ).into())
    }
}
