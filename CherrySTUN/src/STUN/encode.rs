use crate::STUNSerde::encode::STUNEncode; 
use crate::STUN::stun::STUN;

impl STUNEncode for STUN{
    fn encode(
        &self,
        write_cursor: &mut std::io::Cursor<&mut Vec<u8>>,
        encode_context: &Option<&crate::STUNContext::context::STUNContext>,
    ) -> Result<(), crate::STUNError::error::STUNError> {
        todo!()
        //trigger header encode
        //trigger body encode 
        //Add messasge integrity if it exists in the list of requested attributes 
        //Add fingerprint if is exits in the list of attrs
    }
}
