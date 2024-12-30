#![allow(unused)]
#![allow(dead_code)]

use crate::CeXError::CexError::CeXError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub stack: Option<CeXError>,
}


impl Error{
    pub fn new(code: u16, message: String, stack: Option<CeXError>) -> Self {
        Error {
            status: "false".to_string(),
            code,
            message,
            stack,
        }
    }
    pub fn get_error_code(s: Self) -> u16 {
        return s.code;
    }
    pub fn set_error_code(&mut self, code: u16) {
        self.code = code;
    }
    pub fn get_error_message(s: Self) -> String {
        return s.message;
    }
    pub fn set_error_message(&mut self, message: String) {
        self.message = message;
    }
    pub fn get_error_stack(s: Self) -> Option<CeXError> {
        return s.stack;
    }
}
