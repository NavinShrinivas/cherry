use crate::STUNSerde::encode::STUNEncode;
use crate::STUN::stun::STUN;

impl STUNEncode for STUN {
    ///Used for encode a message (struct -> binary)
    ///Accepts the following: 
    ///     - A mutable ref of a write cursor wrapped around a vector (to write outputs to)
    ///     - A optional context (If provided, will use values from it)
    fn encode(
        &self,
        write_cursor: &mut std::io::Cursor<&mut Vec<u8>>,
        encode_context: &Option<&crate::STUNContext::context::STUNContext>,
    ) -> Result<(), crate::STUNError::error::STUNError> {
        //trigger header encode
        //trigger body encode
        //we do not handled short term message integrity yet and not fingerprint
        match self.header.encode(write_cursor, encode_context) {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        };
        match self.body.encode(write_cursor, encode_context) {
            Ok(()) => {}
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}
