use super::attributes::STUNAttributesContent;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::{Cursor, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

impl STUNAttributesContent {
    //Contain decode functions specific to each attribute type content
    //Start with mapped address, XOR mapped address, Fingerprint, Message Integrity
    fn encode_mapped_address(&self) -> Result<Vec<u8>, STUNError> {
        match self {
            Self::MappedAddress { address } => {
                let bin: Vec<u8> = Vec::new();
                let mut header_cursor = Cursor::new(bin);
                match header_cursor.write_u8(0x0000_0000) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(STUNError {
                            step: STUNStep::STUNEncode,
                            error_type: STUNErrorType::WriteError,
                            message: e.to_string()
                                + "Error writing padding bits to mapped address.",
                        })
                    }
                }
                let port = u16::from(address.port());
                println!("{}",port);
                match address {
                    std::net::SocketAddr::V4(ipv4add) => {
                        match header_cursor.write_u8(0x0000_0001){
                            Ok(_) => {},
                            Err(e) => {
                                return Err(
                                    STUNError{
                                        step: STUNStep::STUNEncode,
                                        error_type: STUNErrorType::WriteError,
                                        message: e.to_string() + "Error writing address type while encoding Mapped Address attribute: "
                                    }
                                )
                            }
                        }

                        match header_cursor.write_u16::<NetworkEndian>(port) {
                            Ok(_) => {}
                            Err(e) => return Err(STUNError {
                                step: STUNStep::STUNEncode,
                                error_type: STUNErrorType::WriteError,
                                message:
                                    e.to_string() + "Error writing port while encoding Mapped Address attribute: "
                            }),
                        }

                        match header_cursor.write_u32::<NetworkEndian>(u32::from_be_bytes(ipv4add.ip().octets())){
                            Ok(_) => {},
                            Err(e) => {
                                return Err(
                                    STUNError{
                                        step: STUNStep::STUNEncode,
                                        error_type: STUNErrorType::WriteError,
                                        message: e.to_string() + "Error writing address type while encoding Mapped Address attribute."
                                    }
                                )
                            }
                        }
                    }
                    std::net::SocketAddr::V6(ipv6addr) => {
                        match header_cursor.write_u8(0x0000_0002){
                            Ok(_) => {},
                            Err(e) => {
                                return Err(
                                    STUNError{
                                        step: STUNStep::STUNEncode,
                                        error_type: STUNErrorType::WriteError,
                                        message: e.to_string() + "Error writing address type while encoding Mapped Address attribute."
                                    }
                                )
                            }
                        }

                        match header_cursor.write_u16::<NetworkEndian>(port) {
                            Ok(_) => {}
                            Err(e) => return Err(STUNError {
                                step: STUNStep::STUNEncode,
                                error_type: STUNErrorType::WriteError,
                                message: e.to_string()
                                    + "Error writing port while encoding Mapped Address attribute.",
                            }),
                        }

                        match header_cursor.write_u128::<NetworkEndian>(u128::from_be_bytes(ipv6addr.ip().octets())){
                            Ok(_) => {},
                            Err(e) => {
                                return Err(
                                    STUNError{
                                        step: STUNStep::STUNEncode,
                                        error_type: STUNErrorType::WriteError,
                                        message: e.to_string() + "Error writing address type while encoding Mapped Address attribute."
                                    }
                                )
                            }
                        }
                    }
                }
                return Ok(header_cursor.get_ref().to_vec());
            }
            _ => {
                return Err(STUNError {
                    step: STUNStep::STUNEncode,
                    error_type: STUNErrorType::AttributeTypeMismatch,
                    message: "Called encode function for Mapped address on non Mapped address type"
                        .to_string(),
                })
            }
        };
    }
}

//[TODO next]: write test for normally mapped addresses
#[cfg(test)]
mod test {
    use super::*;
    use crate::STUNBody::test_const::*;
    #[test]
    fn mapped_address_valid_test() {
        let mapped_addr = STUNAttributesContent::MappedAddress {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)), 32853),
        };
        match mapped_addr.encode_mapped_address(){
            Ok(bin) => {
                assert_eq!(&bin[..], STUN_ATTRIBUTE_IPV4_MAPPED_ADDRESS_BIN)
            }
            Err(e) => {
                println!("{:?}", e);
                panic!("Found error, unexpected");
            }
        }
        return;
    }
}
