use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::STUNError;
use std::io::Cursor;

pub trait STUNDecode {
    ///Option context, will be filled if provided and any value in the struct is None
    ///As in when the attribute is come across
    fn decode(
        cursor: &mut Cursor<&[u8]>,
        decode_context: &mut Option<&mut STUNContext>,
    ) -> Result<Self, STUNError>
    where
        Self: Sized;
}
