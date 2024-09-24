use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::STUNError;
use std::io::Cursor;

pub trait STUNEncode {
    //You need a mutable reference here.
    //Cursor interiorly mutates its position
    //This function returns back some struct that is Sized
    //For STUNBody the attributes sizes might not be known
    fn encode(
        &self,
        write_cursor: &mut Cursor<&mut Vec<u8>>,
        encode_context: &Option<&STUNContext>,
    ) -> Result<(), STUNError>;
}
