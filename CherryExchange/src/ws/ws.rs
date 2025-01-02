use crate::handlers::{error::Error, ping::Ping, room::Room, sdp::SDPOfferResponse};
use crate::{Client, Clients, RedisConnectionPool};
use futures::{FutureExt, StreamExt};
use log::error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;

///This method handles the channels received after upgrading to websockets.
pub async fn client_connection(
    ws: WebSocket,
    id: String,
    clients: Clients,
    mut client: Client,
    env: serde_yaml::Value,
    redis_conn_pool: RedisConnectionPool,
) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel(); //whatever is sent inside sender,
                                                                 //will be present in rcv

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        //we forward from rcv to
        //actual ws sender
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender); //we store the sender we have in local context for the
                                         //client
    clients.write().await.insert(id.clone(), client);

    println!("{} connected", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };
        websocket_func_muxer(&id, msg, &clients, env.clone(), redis_conn_pool.clone()).await;
    }

    clients.write().await.remove(&id);
    println!("{} disconnected", id);
}

async fn websocket_func_muxer(
    id: &str,
    msg: warp::ws::Message,
    clients: &Clients,
    env: serde_yaml::Value,
    redis_conn_pool: RedisConnectionPool,
) {
    let msg_str = match msg.to_str() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };
    let message_obj: serde_json::Value = match serde_json::from_str(msg_str) {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    let msg_type = match message_obj["message_type"].as_str() {
        Some(v) => v,
        None => {
            send_message_to_client(
                id,
                Error::new(
                    100,
                    "type is a compulsory field in requests".to_string(),
                    None,
                ),
                clients,
            )
            .await;
            return;
        }
    };

    match msg_type {
        "createRoom" => {
            /*
             * {
             *  "type": "createRoom",
             *  "room_name": "room_name"
             * }
             */
            let room_name = message_obj["room_name"]
                .as_str()
                .unwrap_or("Just another room")
                .to_string();
            let resp = match Room::create_room(room_name, id.to_string(), redis_conn_pool.clone()) {
                Ok(v) => v,
                Err(e) => {
                    let error = Error::new(101, "failed to create room".to_string(), Some(e));
                    send_message_to_client(id, error, clients).await;
                    return;
                }
            };
            send_message_to_client(id, resp, clients).await;
        }
        "joinRoom" => {
            /*
             * {
             *  "type": "joinRoom",
             *  "room_id": "room_id"
             * }
             */
            let room_id = match message_obj["room_id"].as_str() {
                Some(v) => v,
                None => {
                    send_message_to_client(
                        id,
                        Error::new(
                            100,
                            "room_id is a compulsory field in requests".to_string(),
                            None,
                        ),
                        clients,
                    )
                    .await;
                    return;
                }
            };
            let resp =
                match Room::join_room(room_id.to_string(), id.to_string(), redis_conn_pool.clone())
                {
                    Ok(v) => v,
                    Err(e) => {
                        let error = Error::new(102, "failed to join room".to_string(), Some(e));
                        send_message_to_client(id, error, clients).await;
                        return;
                    }
                };
            send_message_to_client(id, resp, clients).await;
        }
        "sdpOffer" => {
            /*
             * {
             *  "type": "sdpOffer",
             *  "to": "room_id"
             *  "offer": "sdp_offer" -- Vec<u8>
             * }
             */
            let semi_resp = SDPOfferResponse::from_request(message_obj.clone());
            let resp = match semi_resp {
                Ok(v) => {
                    v.send_offer_from_self(id.to_string())
                },
                Err(e) => {
                    send_message_to_client(id, e, clients).await;
                    return;
                }
            };
            let to = resp.to.clone();
            send_message_to_client(to.as_str(), resp, clients).await;
        }
        "sdpAnswer" => {
            /*
             * {
             *  "type": "sdpAnswer",
             *  "to": "client_id"
             *  "message": "sdp_answer"
             * }
             */
            let _message = message_obj["message"].as_str().unwrap();
            //[TODO] send the answer only to the client id given in the message_obj, also include
            //the sender client id
        }
        "ping" => {
            /*
             * {
             *  "type": "ping"
             * }
             */
            send_message_to_client(id, Ping::new(id.to_string()), clients).await;
        }
        _ => {
            //unknown message
            //We skip, we intetionally avoid global broadcasts to save on bandwidth
        }
    }
}

pub async fn send_message_to_client<T: serde::Serialize>(id: &str, message: T, clients: &Clients) {
    clients
        .read()
        .await
        .get(id)
        .unwrap()
        .sender
        .as_ref()
        .unwrap()
        .send(Ok(warp::ws::Message::text(
            serde_json::to_string(&message).unwrap(),
        )))
        .unwrap();
}
