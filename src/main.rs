use actix_web::HttpServer;
use dotenv::dotenv;
use hjowdy::create_app;
use tokio_postgres::NoTls;
extern crate chrono;
extern crate serde;

use crate::config::Config;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    HttpServer::new(move || create_app(pool.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
