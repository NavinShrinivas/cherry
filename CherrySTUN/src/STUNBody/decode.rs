//The first line of the body is a header of an attribute, which contains the length of the
//attribute content. Using that we can iterate the binary and compute the delimiter for each
//attribute
use crate::STUNBody::attributes::attributes::STUNAttributeType;
use crate::STUNBody::attributes::attributes::STUNAttributesContent;
use crate::STUNBody::body::STUNBody;
use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use crate::STUNSerde::decode::STUNDecode;
use byteorder::{NetworkEndian, ReadBytesExt};
use std::io::{Cursor, ErrorKind, Read};

use crate::STUNHeader::header::STUN_HEADER_TRANSACTION_ID_START_POSITION;

//driver is responsible for copying required fields into the context as well
//Hence, most clones present here are to populate the context in mem
impl STUNDecode for STUNBody {
    fn decode(
        cursor: &mut Cursor<&[u8]>,
        decode_context: &mut Option<&mut STUNContext>,
    ) -> Result<STUNBody, STUNError> {
        //All the way till the end will be attrs
        let mut new_body = STUNBody::new();
        loop {
            let attribute_type = match cursor.read_u16::<NetworkEndian>() {
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

            match num::FromPrimitive::from_u16(attribute_type) {
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
                Some(STUNAttributeType::XORMappedAddress) => {
                    //we require transactionID to obsfucate xor mapped address
                    //Hence we do a hack to seeking to start of cursors and getting back
                    let curr_pos = cursor.position();
                    let mut transaction_id = [0; 12];
                    cursor.set_position(STUN_HEADER_TRANSACTION_ID_START_POSITION as u64);
                    match cursor.read_exact(&mut transaction_id) {
                        Ok(_) => {}
                        Err(e) => return Err(STUNError {
                            step: STUNStep::STUNDecode,
                            error_type: STUNErrorType::ReadError,
                            message:
                                "Error seeking transaction id for xoring with xor mapped address:"
                                    .to_string()
                                    + e.to_string().as_str(),
                        }),
                    }
                    cursor.set_position(curr_pos);
                    let attr_content = match STUNAttributesContent::decode_xor_mapped_address(
                        cursor,
                        transaction_id,
                    ) {
                        Ok(content) => content,
                        Err(e) => return Err(e),
                    };
                    new_body.add_new_attribute(
                        attr_content,
                        STUNAttributeType::XORMappedAddress,
                        length,
                    )
                }
                Some(STUNAttributeType::Username) => {
                    let attr_content = match STUNAttributesContent::decode_username(
                        cursor,
                        decode_context,
                        length,
                    ) {
                        Ok(content) => content,
                        Err(e) => return Err(e),
                    };
                    new_body.add_new_attribute(attr_content, STUNAttributeType::Username, length)
                }
                Some(STUNAttributeType::Realm) => {
                    let attr_content =
                        match STUNAttributesContent::decode_realm(cursor, decode_context, length) {
                            Ok(content) => content,
                            Err(e) => return Err(e),
                        };
                    new_body.add_new_attribute(attr_content, STUNAttributeType::Realm, length);
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
    use crate::STUNHeader::header::STUN_HEADER_ENDING_POSITION;
    use crate::TestFixtures::fixtures::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    fn roll_cursor_on_fixture(fixture_bin: &[u8]) -> Cursor<&[u8]> {
        return Cursor::new(fixture_bin);
    }

    #[test]
    fn stun_body_decode_success_test() -> Result<(), String> {
        let mut response_cursor = &mut roll_cursor_on_fixture(&STUN_RESPONSE_BODY_TEST);
        response_cursor.set_position(STUN_HEADER_ENDING_POSITION as u64); //20 is the end of headers
        let mut test_encode_context = STUNContext::new();
        let mut option_encode_context = Some(&mut test_encode_context);
        let response = STUNBody::decode(&mut response_cursor, &mut option_encode_context);
        match response {
            Ok(resp) => {
                assert_eq!(resp.attributes.len(), 4); //number of variables
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

                assert_eq!(resp.attributes.get(1).unwrap().length, 20 as u16);
                assert_eq!(
                    resp.attributes.get(1).unwrap().attribute_type,
                    STUNAttributeType::XORMappedAddress
                );
                assert_eq!(
                    resp.attributes.get(1).unwrap().value,
                    STUNAttributesContent::XORMappedAddress {
                        address: SocketAddr::new(
                            IpAddr::V6(Ipv6Addr::new(
                                0x2001, 0xdb8, 0x1234, 0x5678, 0x11, 0x2233, 0x4455, 0x6677
                            )),
                            32853
                        )
                    }
                );

                assert_eq!(resp.attributes.get(2).unwrap().length, 18 as u16);
                assert_eq!(
                    resp.attributes.get(2).unwrap().attribute_type,
                    STUNAttributeType::Username
                );
                let expected_username = "\u{30de}\u{30c8}\u{30ea}\u{30c3}\u{30af}\u{30b9}";
                //This string is unaffected by sasl
                assert_eq!(
                    resp.attributes.get(2).unwrap().value,
                    STUNAttributesContent::Username {
                        username: Some(expected_username.to_string())
                    }
                );

                assert_eq!(resp.attributes.get(3).unwrap().length, 11 as u16);
                assert_eq!(
                    resp.attributes.get(3).unwrap().attribute_type,
                    STUNAttributeType::Realm
                );
                let expected_realm = "example.org";
                //This string is unaffected by sasl
                assert_eq!(
                    resp.attributes.get(3).unwrap().value,
                    STUNAttributesContent::Realm {
                        realm: Some(expected_realm.to_string())
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
    fn stun_body_decode_failure_test() -> Result<(), String> {
        let response = STUNBody::decode(
            &mut roll_cursor_on_fixture(&STUN_RESPONSE_BODY_FAIL_TEST),
            &mut None,
        );
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
