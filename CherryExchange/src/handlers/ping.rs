#![allow(unused)]
#![allow(dead_code)]

use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    pub id: String,
    pub time: String,
    pub message: String,
}

impl Ping {
    pub fn new(id: String) -> Self {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        let time_str = datetime.format("%d/%m/%Y %T").to_string();
        return Ping {
            id,
            time: time_str,
            message: "pong".to_string(),
        };
    }
    fn get_ping_id(s: Self) -> String {
        return s.id;
    }
    fn set_ping_id(&mut self, id: String) {
        self.id = id;
    }
    fn set_ping_time(s: Self) -> String {
        return s.time;
    }
    fn get_ping_time(&mut self, time: String) {
        self.time = time
    }
    fn set_ping_message(s: Self) -> String {
        return s.message;
    }
    fn get_ping_message(&mut self, message: String) {
        self.message = message
    }
}
