use crate::STUNError::error::STUNError;
use crate::STUNHeader::header::STUNHeader;
use std::io::Cursor;

pub trait STUNDecode {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<STUNHeader, STUNError>;
}
