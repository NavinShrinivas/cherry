use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SDPOffer {
    pub message_type: String,
    pub from: String,
    pub to: String,
    pub offer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SDPAnswer {
    pub message_type: String,
    pub from: String,
    pub to: String,
    pub answer: String,
}

impl SDPOffer {
    pub fn send_offer(id: String, to: String, offer: String) -> Self {
        return SDPOffer {
            message_type: "offer".to_string(),
            from,
            to,
            offer,
        };
    }
}
