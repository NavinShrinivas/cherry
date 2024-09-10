use super::attributes::STUNAttributesContent;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

impl STUNAttributesContent {
    pub fn new_mapped_address(address: SocketAddr) -> Self {
        Self::MappedAddress { address }
    }
    //Input to these function arent going to be
    //from the library user, it is going to be fed data from
    //orchestrator/drive for STUNBody
    pub fn encode_mapped_address(&self) -> Result<Vec<u8>, STUNError> {
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
                println!("{}", port);
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

    fn decode_ip_addr_port(
        cursor: &mut Cursor<&[u8]>,
        family: u32,
    ) -> Result<SocketAddr, STUNError> {
        //1 for ipv4
        //2 for ipv6
        let port = match cursor.read_u16::<NetworkEndian>() {
            Ok(bin) => bin,
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error decoding mapped address. Error reading port".to_string()
                        + e.to_string().as_str(),
                });
            }
        };
        if family == 1 {
            //1 is ipv4
            let ip4_addr_u32_bin = match cursor.read_u32::<NetworkEndian>() {
                Ok(bin) => bin,
                Err(e) => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::ReadError,
                        message: "Error decoding mapped address. Error reading ipv4 addr"
                            .to_string()
                            + e.to_string().as_str(),
                    });
                }
            };
            let ip4_addr_obj = Ipv4Addr::from(ip4_addr_u32_bin.to_be_bytes());
            return Ok(SocketAddr::new(IpAddr::V4(ip4_addr_obj), port));
        } else if family == 2 {
            //2 is ipv6
            let ip6_addr_u128_bin = match cursor.read_u128::<NetworkEndian>() {
                Ok(bin) => bin,
                Err(e) => {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::ReadError,
                        message: "Error decoding mapped address. Error reading ipv6 addr"
                            .to_string()
                            + e.to_string().as_str(),
                    });
                }
            };
            let ip6_addr_obj = Ipv6Addr::from(ip6_addr_u128_bin.to_be_bytes());
            return Ok(SocketAddr::new(IpAddr::V6(ip6_addr_obj), port));
        } else {
            return Err({
                STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::InternalError,
                    message: "Internal error, Called function with invalid faimly type".to_string(),
                }
            });
        }
    }

    pub fn decode_mapped_address(cursor: &mut Cursor<&[u8]>) -> Result<Self, STUNError> {
        match cursor.read_u8() {
            Ok(bin) => {
                if bin != 0b0000_0000 {
                    return Err(STUNError {
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::AttributeStructureMismatch,
                        message:
                            "Did not find required 0b0000_0000 when trying to decode mapped address"
                                .to_string(),
                    });
                }
            }
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error decoding mapped address".to_string() + e.to_string().as_str(),
                });
            }
        };
        match cursor.read_u8() {
            Ok(bin) => {
                //Family
                if bin == 0b0000_0001 {
                    match Self::decode_ip_addr_port(cursor, 1) {
                        Ok(socker_addr) => {
                            return Ok(Self::MappedAddress {
                                address: socker_addr,
                            });
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };
                } else if bin == 0b0000_0010 {
                    match Self::decode_ip_addr_port(cursor, 2) {
                        Ok(socker_addr) => {
                            return Ok(Self::MappedAddress {
                                address: socker_addr,
                            });
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };
                } else {
                    return Err(STUNError{
                        step: STUNStep::STUNDecode,
                        error_type: STUNErrorType::AttributeStructureMismatch,
                        message: "Did not find required a valid ip address family type when trying to decode mapped address".to_string(),
                    });
                }
            }
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error decoding mapped address".to_string() + e.to_string().as_str(),
                });
            }
        }
        // Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestFixtures::fixtures::STUN_ATTRIBUTE_IPV4_MAPPED_ADDRESS_BIN;
    #[test]
    fn mapped_address_valid_test_encode() {
        let mapped_addr = STUNAttributesContent::MappedAddress {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)), 32853),
        };
        match mapped_addr.encode_mapped_address() {
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

    //[TODO]
    // fn mapped_address_valid_test_decode() {}
}
