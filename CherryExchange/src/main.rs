use log::info;
use serde_yaml;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter};

mod handlers;
mod ws;

#[derive(Debug, Clone)]
pub struct Client {
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Clients = Arc<RwLock<HashMap<String, Client>>>; //we use the keys to identify the clients.

#[tokio::main]
async fn main() {
    let mut logger = SimpleLogger::new();

    logger = logger.with_level(log::LevelFilter::Info); //Setting default
    logger = logger.env();
    logger.init().unwrap();

    let env = load_yaml_env();

    let port = env["port"].as_u64().unwrap_or(3030);

    info!("Application Cherry Exchange Server : 0.0.0.0:{}", port);

    let clients: Clients = Arc::new(RwLock::new(HashMap::new())); //maintains the client state for
                                                                  //the full app.
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handlers::ws::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], port as u16)).await;
}

fn load_yaml_env() -> serde_yaml::Value {
    let mut file = File::open("./env.yaml").expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    return serde_yaml::from_str(&contents).unwrap();
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
