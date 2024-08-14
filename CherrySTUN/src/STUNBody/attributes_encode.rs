use super::attributes::STUNAttributesContent;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::{Cursor, Write};

impl STUNAttributesContent {
    //Contain decode functions specific to each attribute type content
    //Start with mapped address, XOR mapped address, Fingerprint, Message Integrity
    fn encode_mapped_address(&self) -> Result<Vec<u8>, STUNError> {
        match self {
            Self::MappedAddress { address } => {
                let bin: Vec<u8> = Vec::new();
                let mut header_cursor = Cursor::new(bin);
                header_cursor.write_u8(0x0000_0000);
                let ip = address.ip().to_bits();
                match address {
                    std::net::SocketAddr::V4(ipv4add) => {
                        match header_cursor.write_u8(0x0000_0001){
                            Ok(_) => {},
                            Err(e) => {
                                return Err(
                                    STUNError{
                                        step: STUNStep::STUNEncode,
                                        error_type: STUNErrorType::WriteError,
                                        message: "Error writing address type while encoding Mapped Address attribute: ".to_string() + e
                                    }
                                )
                            }
                        }

                        match header_cursor.write_u16::<NetworkEndian>(ipv4add.port()) {
                            Ok(_) => {}
                            Err(e) => return Err(STUNError {
                                step: STUNStep::STUNEncode,
                                error_type: STUNErrorType::WriteError,
                                message:
                                    "Error writing port while encoding Mapped Address attribute: "
                                        .to_string()
                                        + e,
                            }),
                        }

                        match header_cursor.write_u32::<NetworkEndian>(u32::from(ipv4add.ip().octets()).to_be()){
                            Ok(_) => {},
                            Err(e) => {
                                return Err(
                                    STUNError{
                                        step: STUNStep::STUNEncode,
                                        error_type: STUNErrorType::WriteError,
                                        message: "Error writing address type while encoding Mapped Address attribute: ".to_string() + e
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
                                        message: "Error writing address type while encoding Mapped Address attribute: ".to_string() + e
                                    }
                                )
                            }
                        }

                        match header_cursor.write_u16::<NetworkEndian>(ipv6addr.port()) {
                            Ok(_) => {}
                            Err(e) => return Err(STUNError {
                                step: STUNStep::STUNEncode,
                                error_type: STUNErrorType::WriteError,
                                message:
                                    "Error writing port while encoding Mapped Address attribute: "
                                        .to_string()
                                        + e,
                            }),
                        }

                        match header_cursor.write_u128::<NetworkEndian>(u128::from(ipv6addr.ip().octets()).to_be()){
                            Ok(_) => {},
                            Err(e) => {
                                return Err(
                                    STUNError{
                                        step: STUNStep::STUNEncode,
                                        error_type: STUNErrorType::WriteError,
                                        message: "Error writing address type while encoding Mapped Address attribute: ".to_string() + e
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
