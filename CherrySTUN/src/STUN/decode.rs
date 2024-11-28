use crate::STUNSerde::decode::STUNDecode;
use crate::STUN::stun::STUN;
use crate::STUNBody::body;
use crate::STUNHeader::header;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};

impl STUNDecode for STUN {

    fn decode(
        cursor: &mut std::io::Cursor<&[u8]>,
        decode_context: &mut Option<&mut crate::stunContext::STUNContext>,
    ) -> Result<Self, crate::STUNError::error::STUNError>
    where
        Self: Sized {
            let stun_header = match header::STUNHeader::decode(cursor, decode_context){
                Ok(x) => x,
                Err(e) => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::ReadError,
                        message: "Error decoding sutn message header".to_string()
                            + e.to_string().as_str(),
                    })
                }
            };
            let stun_body = match body::STUNBody::decode(cursor, decode_context){
                Ok(x) => x,
                Err(e) => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::ReadError,
                        message: "Error decoding sutn message body".to_string()
                            + e.to_string().as_str(),
                    })
                }

            };
            return Ok(Self::new(stun_header, stun_body));
    }
}
