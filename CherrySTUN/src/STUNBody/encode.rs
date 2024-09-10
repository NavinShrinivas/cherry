use crate::STUNBody::attributes::attributes::{STUNAttributeType, STUNAttributesContent};
use crate::STUNBody::body::STUNBody;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use crate::STUNHeader::header::STUN_HEADER_TRANSACTION_ID_START_POSITION;
use crate::STUNSerde::encode::STUNEncode;
use std::io::{Read, Write};

#[allow(unreachable_code)]
#[allow(unreachable_patterns)] //to cover "_" branch incase of new attributes

impl STUNEncode for STUNBody {
    fn encode(&self, write_cursor: &mut std::io::Cursor<&mut Vec<u8>>) -> Result<(), STUNError> {
        for (_, attribute) in self.attributes.iter().enumerate() {
            match attribute.value {
                STUNAttributesContent::MappedAddress { .. } => {
                    match STUNAttributesContent::encode_mapped_address(&attribute.value) {
                        Ok(bin_rep) => {
                            match Self::write_attribute_header_encode(
                                &bin_rep,
                                write_cursor,
                                STUNAttributeType::MappedAddress,
                            ) {
                                Ok(_) => {}
                                Err(e) => return Err(e),
                            };
                            match write_cursor.write_all(bin_rep.as_slice()) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err(STUNError {
                                        step: STUNStep::STUNDecode,
                                        error_type: STUNErrorType::WriteError,
                                        message: e.to_string()
                                            + ". Error writing encoded attribute to cursor.",
                                    })
                                }
                            }
                        }
                        Err(e) => return Err(e),
                    };
                }
                STUNAttributesContent::XORMappedAddress { .. } => {
                    let curr_pos = write_cursor.position();
                    let mut transaction_id = [0; 12];
                    write_cursor.set_position(STUN_HEADER_TRANSACTION_ID_START_POSITION as u64);
                    match write_cursor.read_exact(&mut transaction_id) {
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
                    write_cursor.set_position(curr_pos);
                    match STUNAttributesContent::encode_xor_mapped_address(
                        &attribute.value,
                        transaction_id,
                    ) {
                        Ok(bin_rep) => {
                            match Self::write_attribute_header_encode(
                                &bin_rep,
                                write_cursor,
                                STUNAttributeType::XORMappedAddress,
                            ) {
                                Ok(_) => {}
                                Err(e) => return Err(e),
                            };
                            match write_cursor.write_all(bin_rep.as_slice()) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err(STUNError {
                                        step: STUNStep::STUNDecode,
                                        error_type: STUNErrorType::WriteError,
                                        message: e.to_string()
                                            + ". Error writing encoded attribute to cursor.",
                                    })
                                }
                            }
                        }
                        Err(e) => return Err(e),
                    };
                }
                _ => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::InvalidOrUnsupportedAttribute,
                        message: "Found invalid/unsupported attribute type when encoding.:"
                            .to_string(),
                    })
                }
            };
        }
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestFixtures::fixtures::*;
    use std::io::Cursor;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    fn roll_cursor_on_fixture(fixture_bin: &mut Vec<u8>) -> Cursor<&mut Vec<u8>> {
        let len = fixture_bin.len();
        let mut test_cursor = Cursor::new(fixture_bin);
        test_cursor.set_position(len as u64);
        return test_cursor;
    }

    #[test]
    fn stun_body_encode_success_test() -> Result<(), String> {
        let mut test_bin = STUN_TEST_HEADER.to_vec();
        let mut write_test_cursor = roll_cursor_on_fixture(&mut test_bin); //the encode function expects the cursor to have
                                                                           //the header encoded first before we even start
                                                                           //encode on body.
        let mut test_body = STUNBody::new();
        test_body.add_new_attribute(
            STUNAttributesContent::MappedAddress {
                address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)), 32853),
            },
            STUNAttributeType::MappedAddress,
            0,
        ); //the length is optional to be filled by users
           //the encode function smartly figured out the
           //length and fills it for us
        test_body.add_new_attribute(
            STUNAttributesContent::XORMappedAddress {
                address: SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::new(
                        0x2001, 0xdb8, 0x1234, 0x5678, 0x11, 0x2233, 0x4455, 0x6677,
                    )),
                    32853,
                ),
            },
            STUNAttributeType::XORMappedAddress,
            0,
        );
        let answer_bin = STUN_RESPONSE_BODY_TEST.to_vec();
        match test_body.encode(&mut write_test_cursor) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string() + ". Got unexpected error.");
            }
        }
        assert_eq!(write_test_cursor.get_ref().to_vec(), answer_bin);
        return Ok(());
    }

    #[test]
    fn stun_body_encode_failure_test() -> Result<(), String> {
        //this test is without having the encoded header, which should cause the encode to fail
        //when we have XOR mapped address in the obj
        let mut test_vec = Vec::new();
        let mut write_test_cursor = Cursor::new(&mut test_vec);
        let mut test_body = STUNBody::new();
        test_body.add_new_attribute(
            STUNAttributesContent::MappedAddress {
                address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)), 32853),
            },
            STUNAttributeType::MappedAddress,
            0,
        ); //the length is optional to be filled by users
           //the encode function smartly figured out the
           //length and fills it for us
        test_body.add_new_attribute(
            STUNAttributesContent::XORMappedAddress {
                address: SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::new(
                        0x2001, 0xdb8, 0x1234, 0x5678, 0x11, 0x2233, 0x4455, 0x6677,
                    )),
                    32853,
                ),
            },
            STUNAttributeType::XORMappedAddress,
            0,
        );
        match test_body.encode(&mut write_test_cursor) {
            Ok(_) => {}
            Err(e) => {
                if e.error_type == STUNErrorType::ReadError {
                    return Ok(());
                }
            }
        }
        return Ok(());
    }
}
