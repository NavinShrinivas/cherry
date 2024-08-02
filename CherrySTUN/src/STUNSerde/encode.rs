use crate::STUNError::error::STUNError;
use std::io::Cursor;

pub trait STUNDecode {
    //You need a mutable reference here.
    //Cursor interiorly mutates its posistion
    //This function returns back some struct that is Sized
    fn decode<S: Sized>(cursor: &mut Cursor<&[u8]>) -> Result<S, STUNError>;
}
