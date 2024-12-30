use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;

///This method handles the channels received after upgrading to websockets.
pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
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
        websocket_func_muxer(&id, msg, &clients).await;
    }

    clients.write().await.remove(&id);
    println!("{} disconnected", id);
}

async fn websocket_func_muxer(id: &str, msg: warp::ws::Message, clients: &Clients) {
    let msg_str = match msg.to_str(){
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };
    let message_obj: serde_json::Value = match serde_json::from_str(msg_str){
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    match message_obj["type"].as_str().unwrap() {
        "createRoom" => {
            /*
             * {
             *  "type": "createRoom",
             *  "message": "room_name"
             * }
             */
            let _message = message_obj["message"].as_str().unwrap();
            //[TODO] Create a new room in redis, respond with room_id
        }
        "joinRoom" => {
            /*
             * {
             *  "type": "joinRoom",
             *  "message": "room_id"
             * }
             */
            let _message = message_obj["message"].as_str().unwrap();
            //[TODO] store this client id as part of the room in redis
        }
        "sdpOffer" => {
            /*
             * {
             *  "type": "sdpOffer",
             *  "to": "room_id"
             *  "message": "sdp_offer"
             * }
             */
            let _message = message_obj["message"].as_str().unwrap();
            //[TODO] send offer to all "id's" for this room in redis, along with current sender
            //client id, this is the only way we can keep track of sender during answer
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
            clients
                .read()
                .await
                .get(id)
                .unwrap()
                .sender
                .as_ref()
                .unwrap()
                .send(Ok(warp::ws::Message::text(format!(
                    "{{\"type\": \"pong\", \"id\": \"{}\"}}",
                    id
                ))))
                .unwrap();
        }
        _ => {
            //unknown message
            //We skip, we intetionally avoid global broadcasts to save on bandwidth
        }
    }
}
