use crate::CeXError::CexError::{CeXError, CeXErrorType, CeXStep};
use crate::{RedisConnectionPool, redis};
use log::{error, info};
use r2d2_redis::redis::{from_redis_value, FromRedisValue, Commands, ToRedisArgs, ErrorKind};
use serde::{Deserialize, Serialize};

///Room is the storage unit in redis for the clients in the room.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub clients: Vec<String>,
    pub room_name: String,
    pub room_id: String,
}

#[derive(Serialize, Debug)]
pub struct RoomCreateResponse {
    pub status: String,
    pub room_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomJoinRequest {
    pub room_id: String,
}

#[derive(Serialize, Debug)]
pub struct RoomJoinResponse {
    pub status: String,
    pub room_id: String,
    pub users: Vec<String>,
}

impl Room {

    pub fn create_room(
        room_name: String,
        id: String, //Creator of the room
        redis_conn_pool: RedisConnectionPool,
    ) -> Result<RoomCreateResponse, CeXError> {
        let room_id = uuid::Uuid::new_v4().to_string();
        let room = Room {
            clients: vec![id.clone()],
            room_name,
            room_id: room_id.clone()
        };
        let mut connection = match redis::connection::get_con(&redis_conn_pool) {
            Ok(con) => con,
            Err(e) => {
                error!("Error getting redis connection: {}", e);
                return Err(CeXError::new(
                    CeXStep::CeXRedis,
                    CeXErrorType::ConnectError,
                    e.to_string(),
                ));
               
            }
        };
        match connection.set::<String, Room, String>(room_id.clone(), room) {
            Ok(_) => {}
            Err(e) => {
                error!("Error setting room in redis: {}", e);
                return Err(CeXError::new(
                    CeXStep::CeXRedis,
                    CeXErrorType::WriteError,
                    e.to_string(),
                ));
            }
        }
        Ok(RoomCreateResponse { status: "true".to_string(), room_id })
    }
}

impl FromRedisValue for Room {
    fn from_redis_value(v: &r2d2_redis::redis::Value) -> r2d2_redis::redis::RedisResult<Self> {
        let v: String = from_redis_value(v)?;
        let room: Room = match serde_json::from_str(&v){
            Ok(r) => r,
            Err(e) => {
                error!("Error deserializing room response from redis: {}", e);
                return Err((ErrorKind::TypeError, "Unexpected response").into());
                
            }
        };
        Ok(room)
    }
}

impl ToRedisArgs for Room{
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + r2d2_redis::redis::RedisWrite {
        let s = match serde_json::to_string(self){
            Ok(v) => v,
            Err(e) => {
                error!("Error serializing room to redis: {}", e);
                return;
            }
        };
        out.write_arg(s.as_bytes());
    }
}
