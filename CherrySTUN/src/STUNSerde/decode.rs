use crate::STUNError::error::STUNError;
use std::io::Cursor;

pub trait STUNDecode {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, STUNError>
    where
        Self: Sized;
}
