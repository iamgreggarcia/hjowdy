use crate::db;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use reqwest::header::{HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use reqwest::Client;
use serde_json::json;
use crate::models::Image;
use deadpool_postgres::Pool;

#[derive(Debug, Deserialize)]
pub struct ImageGenerationRequest {
    chat_id: i32,
    prompt: String,
    n: Option<u32>,
    size: Option<String>,
    response_format: Option<String>,
}

pub async fn get_images_by_chat_id(
    chat_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<impl Responder, actix_web::Error> {
    let chat_id = chat_id.into_inner();
    let client = pool.get().await.map_err(|e| {
        actix_web::error::InternalError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    let images: Vec<Image> = db::get_images_by_chat_id(&client, chat_id)
        .await
        .map_err(|e| {
            actix_web::error::InternalError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    Ok(HttpResponse::Ok().json(images))
}

pub async fn generate_image(
    image_generation_request: web::Json<ImageGenerationRequest>,
    config: web::Data<crate::config::Config>,
    pool: web::Data<deadpool_postgres::Pool>,
    ) -> Result<impl Responder, actix_web::Error> {
    println!("{:?}", image_generation_request);
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

        let json_body: serde_json::Value = serde_json::from_str(&body)?;
        let image_url = json_body["data"][0]["url"].as_str().unwrap_or_default().to_string();
        let chat_id = image_generation_request.chat_id;

        let client = pool.get().await.map_err(|e| {
            actix_web::error::InternalError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

        db::save_generated_image(&client, chat_id, image_url).await.map_err(|e| {
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
