//The first line of the body is a header of an attribute, which contains the length of the
//attribute content. Using that we can iterate the binary and compute the delimiter for each
//attribute
use crate::STUNBody::attributes::attributes::STUNAttributeType;
use crate::STUNBody::attributes::attributes::STUNAttributesContent;
use crate::STUNBody::body::STUNBody;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use crate::STUNSerde::decode::STUNDecode;
use byteorder::{NetworkEndian, ReadBytesExt};
use std::io::{Cursor, ErrorKind};

impl STUNDecode for STUNBody {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<STUNBody, STUNError> {
        //All the way till the end will be attrs
        let mut new_body = STUNBody::new();
        loop {
            let attribute = match cursor.read_u16::<NetworkEndian>() {
                Ok(bin) => bin,
                Err(e) => {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => {
                            //reached end of buffer
                            return Ok(new_body);
                        }
                        _ => {
                            return Err(STUNError {
                                step: STUNStep::STUNDecode,
                                error_type: STUNErrorType::ReadError,
                                message: "Error reading message type when decoding :".to_string()
                                    + e.to_string().as_str(),
                            })
                        }
                    }
                }
            };

            let length = match cursor.read_u16::<NetworkEndian>() {
                Ok(bin) => bin,
                Err(e) => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::ReadError,
                        message: "Error reading attribute length when decoding :".to_string()
                            + e.to_string().as_str(),
                    })
                }
            };

            match num::FromPrimitive::from_u16(attribute) {
                Some(STUNAttributeType::MappedAddress) => {
                    let attr_content = match STUNAttributesContent::decode_mapped_address(cursor) {
                        Ok(content) => content,
                        Err(e) => return Err(e),
                    };
                    new_body.add_new_attribute(
                        attr_content,
                        STUNAttributeType::MappedAddress,
                        length,
                    )
                }
                _ => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::InvalidOrUnsupportedAttribute,
                        message: "Found invalid/unsupported attribute type when decoding.:"
                            .to_string(),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestFixtures::fixtures::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    fn roll_cursor_on_fixture(fixture_bin: &[u8]) -> Cursor<&[u8]> {
        return Cursor::new(fixture_bin);
    }

    #[test]
    fn stun_body_success_test() -> Result<(), String> {
        let response = STUNBody::decode(&mut roll_cursor_on_fixture(&STUN_RESPONSE_BODY_TEST));
        match response {
            Ok(resp) => {
                assert_eq!(resp.attributes.get(0).unwrap().length, 8 as u16);
                assert_eq!(
                    resp.attributes.get(0).unwrap().attribute_type,
                    STUNAttributeType::MappedAddress
                );
                assert_eq!(
                    resp.attributes.get(0).unwrap().value,
                    STUNAttributesContent::MappedAddress {
                        address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)), 32853)
                    }
                );
                return Ok(());
            }
            Err(e) => {
                return Err(String::from(
                    "Unexpected failure on decoding stun body".to_string() + e.message.as_str(),
                ));
            }
        };
    }

    #[test]
    fn stun_body_failure_test() -> Result<(), String> {
        let response = STUNBody::decode(&mut roll_cursor_on_fixture(&STUN_RESPONSE_BODY_FAIL_TEST));
        match response {
            Ok(_) => {
                return Err(String::from(
                    "Expected failure on wrong method type while decoding header",
                ));
            }
            Err(e) => {
                if e.error_type == STUNErrorType::InvalidOrUnsupportedAttribute {
                    return Ok(());
                } else {
                    return Err("Wrong error type, received.".to_string());
                }
            }
        };
    }
}
