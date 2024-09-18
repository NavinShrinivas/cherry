use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use crate::STUNHeader::header::{
    STUNHeader, STUNMessageClass, STUNMessageMethod, STUN_5389_MAGIC_NUMBER_U32,
};
use crate::STUNSerde::decode::STUNDecode;
use byteorder::{NetworkEndian, ReadBytesExt};
use std::io::{Cursor, ErrorKind, Read};

impl STUNDecode for STUNHeader {
    ///decode_context is never filled by the header
    fn decode(
        cursor: &mut Cursor<&[u8]>,
        _: &mut Option<&mut STUNContext>,
    ) -> Result<STUNHeader, STUNError> {
        //We assume the cursor position is sent to this function from first bit
        let message_type = match cursor.read_u16::<NetworkEndian>() {
            Ok(bin) => bin,
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error reading message type when decoding :".to_string()
                        + e.to_string().as_str(),
                })
            }
        };
        let message_class: STUNMessageClass =
            match num::FromPrimitive::from_u16(message_type & 0b0000_0001_0001_0000) {
                Some(class) => class,
                None => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::InvalidClassError,
                        message: "Invalid message class found while decoding :".to_string(),
                    })
                }
            };

        let message_method: STUNMessageMethod =
            match num::FromPrimitive::from_u16(message_type & 0b0011_1110_1110_1111) {
                Some(class) => class,
                None => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::InvalidMethodError,
                        message: "Invalid message method found while decoding :".to_string(),
                    })
                }
            };

        let body_length = match cursor.read_u16::<NetworkEndian>() {
            Ok(bin) => bin,
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error reading body length when decoding :".to_string()
                        + e.to_string().as_str(),
                })
            }
        };

        //magic cookie check :
        match cursor.read_u32::<NetworkEndian>() {
            Ok(bin) => {
                if bin != STUN_5389_MAGIC_NUMBER_U32 {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::MagicCookieMismatchError,
                        message: "Magic cookie bits from binary did not match expected."
                            .to_string(),
                    });
                }
            }
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error reading magic cookie when decoding :".to_string()
                        + e.to_string().as_str(),
                })
            }
        };

        let mut transaction_id = [0; 12];
        match cursor.read_exact(&mut transaction_id) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    //What if the cursor (even if it is of the whole message) has a slice lesser than 20 bytes
                    return Err(STUNError{
                            step: STUNStep::STUNDecode,
                            error_type: STUNErrorType::WrongSizeError,
                            message: "Length of entier message is not 20 bytes. (Expected Header alone to be 20 bytes)".to_string()
                        }
                    );
                }
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error reading transaction ID when decoding :".to_string()
                        + e.to_string().as_str(),
                });
            }
        };
        let mut header = STUNHeader::new(message_class, message_method, Some(transaction_id));
        header.increment_message_length(body_length);

        return Ok(header);
    }
}

//[TODO]: Write tests for deocde
// Error branches :
// - Error on wrong class
// - Error on wrong magic number
// - Error on smaller header size

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestFixtures::fixtures::*;
    fn roll_cursor_on_fixture(fixture_bin: &[u8]) -> Cursor<&[u8]> {
        return Cursor::new(fixture_bin);
    }

    #[test]
    fn stun_header_decode_wrong_class() -> Result<(), String> {
        let response = STUNHeader::decode(
            &mut roll_cursor_on_fixture(&STUN_INCORRECT_METHOD_HEADER_BINARY),
            &mut None,
        );
        match response {
            Ok(_) => {
                return Err(String::from(
                    "Expected failure on wrong method type while decoding header",
                ));
            }
            Err(e) => {
                if e.error_type == STUNErrorType::InvalidMethodError {
                    return Ok(());
                } else {
                    return Err("Wrong error type, received.".to_string());
                }
            }
        };
    }

    #[test]
    fn stun_header_decode_wrong_magic_number() -> Result<(), String> {
        let response = STUNHeader::decode(
            &mut roll_cursor_on_fixture(&STUN_INCORRECT_MAGIC_NUMBER_HEADER_BINARY),
            &mut None,
        );
        match response {
            Ok(_) => {
                return Err(String::from(
                    "Expected failure on wrong magic number type decoding header",
                ));
            }
            Err(e) => {
                if e.error_type == STUNErrorType::MagicCookieMismatchError {
                    return Ok(());
                } else {
                    return Err("Wrong error type, received.".to_string());
                }
            }
        };
    }

    #[test]
    fn stun_header_decode_smaller_header() -> Result<(), String> {
        let response = STUNHeader::decode(
            &mut roll_cursor_on_fixture(&STUN_SMALLER_HEADER_BINARY),
            &mut None,
        );
        match response {
            Ok(_) => {
                return Err(String::from(
                    "Expected failure on wrong magic number type decoding header",
                ));
            }
            Err(e) => {
                if e.error_type == STUNErrorType::WrongSizeError {
                    return Ok(());
                } else {
                    println!("{:?}", e);
                    return Err("Wrong error type, received.".to_string());
                }
            }
        };
    }

    #[test]
    fn stun_header_decode_binding_request() {
        let response = STUNHeader::decode(
            &mut roll_cursor_on_fixture(&STUN_REQUEST_BINDING_HEADER_BINARY),
            &mut None,
        );
        match response {
            Ok(header_obj) => {
                let mut stun_request_binding_header_obj = STUNHeader::new(
                    STUNMessageClass::Request,
                    STUNMessageMethod::Binding,
                    Some(EXAMPLE_STUN_REQUEST_TRANSACTION_ID),
                );
                stun_request_binding_header_obj.increment_message_length(88);
                assert_eq!(header_obj, stun_request_binding_header_obj);
            }
            Err(e) => {
                println!("{:?}", e);
                panic!("received error, expected success on decoding header");
            }
        };
    }
}
