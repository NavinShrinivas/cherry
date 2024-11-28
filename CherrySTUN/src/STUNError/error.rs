#![allow(unused)]
#![allow(dead_code)]

#[derive(Debug)]
pub enum STUNStep {
    STUNEncode,
    STUNDecode,
    STUNUtils,
    STUNNetwork,
}

#[derive(Debug, PartialEq)]
pub enum STUNErrorType {
    ReadError,
    WriteError,
    MagicCookieMismatchError,
    WrongSizeError,
    InvalidClassError,
    InvalidMethodError,
    AttributeTypeMismatch,
    AttributeStructureMismatch, //When the expected structure is found
    InternalError,              //When a call to a non public function goes wrong
    InvalidOrUnsupportedAttribute,
    XORObsfucationError,
    UTF8DecodeError, //Used when we try representing the bin as a utf8 encoded string but is not
    RequiredContextMissingError,
    SASLPrepError,
    InvalidMessageBinLength,
    MessageIntegrityMismatch,
    ErrorSendingMessageToServer, 
    ErrorReceivingFromServer,
    NetworkTimeoutError,
    ErrorSettingNetworkTimeout,
}

#[derive(Debug)]
pub struct STUNError {
    pub step: STUNStep,
    pub error_type: STUNErrorType,
    pub message: String,
}

impl std::fmt::Display for STUNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} | {:?} | {}",
            self.step, self.error_type, self.message
        )
    }
}

impl STUNError {
    pub fn new(step: STUNStep, err_type: STUNErrorType, message: String) -> Self {
        return STUNError {
            step,
            error_type: err_type,
            message,
        };
    }
    fn get_stun_error_step(s: Self) -> STUNStep {
        return s.step;
    }
    fn set_stun_error_step(&mut self, step: STUNStep) {
        self.step = step;
    }
    fn set_stun_error_message(s: Self) -> String {
        return s.message;
    }
    fn get_stun_error_message(&mut self, message: String) {
        self.message = message
    }
    fn set_stun_error_type(s: Self) -> STUNErrorType {
        return s.error_type;
    }
    fn get_stun_error_type(&mut self, error_type: STUNErrorType) {
        self.error_type = error_type
    }
}
