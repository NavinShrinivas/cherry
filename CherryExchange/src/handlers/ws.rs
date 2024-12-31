use crate::ws::ws::client_connection;
use crate::{Clients, RedisConnectionPool};
use warp::{Rejection, Reply};

///This method is called on the initial websocket upgrade.
pub async fn ws_handler(
    ws: warp::ws::Ws,
    clients: Clients,
    env: serde_yaml::Value,
    redis_conn_pool: RedisConnectionPool,
) -> Result<impl Reply, Rejection> {
    //Dummy empty client
    let id = uuid::Uuid::new_v4().to_string();
    let c = crate::Client { sender: None };
    Ok(
        ws.on_upgrade(move |socket| {
            client_connection(socket, id, clients, c, env, redis_conn_pool)
        }),
    )
}
