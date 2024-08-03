#[derive(Debug)]
pub enum STUNStep {
    STUNEncode,
    STUNDecode,
}

#[derive(Debug)]
pub enum STUNErrorType {
    ReadError,
    WriteError,
}

#[derive(Debug)]
pub struct STUNError {
    pub step: STUNStep,
    pub error_type: STUNErrorType,
    pub message: String,
}

impl STUNError {
    pub fn new(step: STUNStep, err_type: STUNErrorType, message: String) -> Self{
        return STUNError{
            step, 
            error_type: err_type, 
            message
        }
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
