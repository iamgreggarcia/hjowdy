use serde::Deserialize;
#[derive(Debug,Default,Deserialize)]
pub struct Config {
    pub server_addr: Server,
    pub pg: deadpool_postgres::Config,
}
