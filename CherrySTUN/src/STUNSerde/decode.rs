use crate::STUNError::error::STUNError;
use std::io::Cursor;

pub trait STUNDecode {
    fn decode<S: Sized>(cursor: &mut Cursor<&[u8]>) -> Result<S, STUNError>;
}
