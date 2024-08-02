use crate::STUNError::error::STUNError;

pub trait STUNDecode {
    fn encode(s: Self) -> Result<Vec<u8>, STUNError>;
}
