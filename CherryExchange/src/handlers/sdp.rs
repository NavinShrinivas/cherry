#![allow(unused)]
#![allow(dead_code)]

use crate::CeXError::CexError::{CeXError, CeXErrorType, CeXStep};
use serde::{Deserialize, Serialize};

//respose goes to client from server (server -> client)
#[derive(Serialize, Deserialize, Debug)]
pub struct SDPOfferResponse {
    pub message_type: String,
    pub from: String,
    pub to: String,
    pub offer: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SDPAnswerResponse {
    pub message_type: String,
    pub from: String,
    pub to: String,
    pub answer: Vec<u8>,
}

impl SDPAnswerResponse {
    pub fn from_request(message_obj: serde_json::Value) -> Result<Self, CeXError> {
        let to = match message_obj["to"].as_str() {
            Some(v) => v,
            None => {
                return Err(CeXError {
                    step: CeXStep::CeXRequestProcessing,
                    error_type: CeXErrorType::SerDeError,
                    message: "to is a compulsory field in requests".to_string(),
                });
            }
        };
        let answer: Vec<u8> = match serde_json::from_value(message_obj["answer"].clone()) {
            Ok(v) => v,
            Err(e) => {
                return Err(CeXError {
                    step: CeXStep::CeXRequestProcessing,
                    error_type: CeXErrorType::SerDeError,
                    message: "answer is a compulsory field in requests: ".to_string()
                        + &e.to_string(),
                });
            }
        };
        Ok(SDPAnswerResponse {
            message_type: "sdpAnswer".to_string(),
            from: "".to_string(),
            to: to.to_string(),
            answer,
        })
    }
    pub fn send_answer_from_self(self, id: String) -> Self {
        return SDPAnswerResponse {
            message_type: "sdpAnswer".to_string(),
            from: id,
            to: self.to,
            answer: self.answer,
        };
    }
}

impl SDPOfferResponse {
    pub fn send_offer(id: String, to: String, offer: Vec<u8>) -> Self {
        return SDPOfferResponse {
            message_type: "sdpOffer".to_string(),
            from: id,
            to,
            offer,
        };
    }
    pub fn send_offer_from_self(self, id: String) -> Self {
        return SDPOfferResponse {
            message_type: "sdpOffer".to_string(),
            from: id,
            to: self.to,
            offer: self.offer,
        };
    }
    pub fn from_request(message_obj: serde_json::Value) -> Result<Self, CeXError> {
        let to = match message_obj["to"].as_str() {
            Some(v) => v,
            None => {
                return Err(CeXError {
                    step: CeXStep::CeXRequestProcessing,
                    error_type: CeXErrorType::SerDeError,
                    message: "to is a compulsory field in requests".to_string(),
                });
            }
        };
        let offer: Vec<u8> = match serde_json::from_value(message_obj["offer"].clone()) {
            Ok(v) => v,
            Err(e) => {
                return Err(CeXError {
                    step: CeXStep::CeXRequestProcessing,
                    error_type: CeXErrorType::SerDeError,
                    message: "offer is a compulsory field in requests: ".to_string()
                        + &e.to_string(),
                });
            }
        };
        Ok(SDPOfferResponse {
            message_type: "sdpOffer".to_string(),
            from: "".to_string(),
            to: to.to_string(),
            offer,
        })
    }
}
