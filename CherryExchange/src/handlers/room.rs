use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RoomCreateRequest {}

#[derive(Serialize, Debug)]
pub struct RoomCreateResponse {
    pub room_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomJoinRequest {
    pub room_id: String,
}

#[derive(Serialize, Debug)]
pub struct RoomJoinResponse {
    pub room_id: String,
    pub users: Vec<String>,
}
