use crate::STUNError::error::STUNError;

pub trait STUNDecode {
    fn decode(&self) -> Result<Vec<u8>, STUNError>;
}
