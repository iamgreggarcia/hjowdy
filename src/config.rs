use serde::Deserialize;

use dotenv::dotenv;
use std::env;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Config {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let server_addr = env::var("SERVER_ADDR")?;
        let api_key = env::var("OPENAI_API_KEY").unwrap();
        let pg = deadpool_postgres::Config {
            user: Some(env::var("PG.USER")?),
            password: Some(env::var("PG.PASSWORD")?),
            host: Some(env::var("PG.HOST")?),
            port: Some(env::var("PG.PORT")?.parse::<u16>()?),
            dbname: Some(env::var("PG.DBNAME")?),
            pool: Some(deadpool_postgres::PoolConfig {
                max_size: env::var("PG.POOL.MAX_SIZE")?.parse::<usize>()?,
                ..Default::default()
            }),
            ..Default::default()
        };
        Ok(Self {
            server_addr,
            pg,
            api_key,
        })
    }
}
