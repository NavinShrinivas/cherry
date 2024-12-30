#![allow(unused)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CeXStep {
    CeXRedis,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CeXErrorType {
    ReadError,
    WriteError,
    ConnectError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CeXError {
    pub step: CeXStep,
    pub error_type: CeXErrorType,
    pub message: String,
}

impl std::fmt::Display for CeXError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} | {:?} | {}",
            self.step, self.error_type, self.message
        )
    }
}

impl CeXError {
    pub fn new(step: CeXStep, err_type: CeXErrorType, message: String) -> Self {
        return CeXError {
            step,
            error_type: err_type,
            message,
        };
    }
    fn get_cex_error_step(s: Self) -> CeXStep {
        return s.step;
    }
    fn set_cex_error_step(&mut self, step: CeXStep) {
        self.step = step;
    }
    fn set_cex_error_message(s: Self) -> String {
        return s.message;
    }
    fn get_cex_error_message(&mut self, message: String) {
        self.message = message
    }
    fn set_stun_error_type(s: Self) -> CeXErrorType {
        return s.error_type;
    }
    fn get_cex_error_type(&mut self, error_type: CeXErrorType) {
        self.error_type = error_type
    }
}
