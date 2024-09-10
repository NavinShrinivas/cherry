/*
*
* X-Port is computed by taking the mapped port in host byte order,
  XOR'ing it with the most significant 16 bits of the magic cookie, and
  then the converting the result to network byte order.  If the IP
  address family is IPv4, X-Address is computed by taking the mapped IP
  address in host byte order, XOR'ing it with the magic cookie, and
  converting the result to network byte order.  If the IP address
  family is IPv6, X-Address is computed by taking the mapped IP address
  in host byte order, XOR'ing it with the concatenation of the magic
  cookie and the 96-bit transaction ID, and converting the result to
  network byte order.

  The rules for encoding and processing the first 8 bits of the
  attribute's value, the rules for handling multiple occurrences of the
  attribute, and the rules for processing address families are the same
  as for MAPPED-ADDRESS.

  Note: XOR-MAPPED-ADDRESS and MAPPED-ADDRESS differ only in their
  encoding of the transport address.  The former encodes the transport
  address by exclusive-or'ing it with the magic cookie.  The latter
  encodes it directly in binary.  RFC 3489 originally specified only
  MAPPED-ADDRESS.  However, deployment experience found that some NATs
  rewrite the 32-bit binary payloads containing the NAT's public IP
  address, such as STUN's MAPPED-ADDRESS attribute, in the well-meaning
  but misguided attempt at providing a generic ALG function.  Such
  behavior interferes with the operation of STUN and also causes
  failure of STUN's message-integrity checking.
*
*
* */

use super::attributes::STUNAttributesContent;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use crate::STUNHeader::header::STUN_5389_MAGIC_NUMBER_U32;
use byteorder;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

pub const STUN_5389_MAGIC_NUMBER_16MSB_U16: u16 = 0x2112;

impl STUNAttributesContent {
    fn xorObsfucationDeObsfucation_SocketAdrr(
        address: &SocketAddr,
        transaction_id: [u8; 12],
    ) -> Result<SocketAddr, STUNError> {
        //We are assuming the host byte order is same as network byte order
        let port_bin = u16::from(address.port());
        let xored_port_bin = port_bin ^ STUN_5389_MAGIC_NUMBER_16MSB_U16;
        match address {
            SocketAddr::V4(ipv4addr) => {
                let address_bin = u32::from_be_bytes(ipv4addr.ip().octets());
                let xored_address_bin = address_bin ^ STUN_5389_MAGIC_NUMBER_U32;
                return Ok(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::from(xored_address_bin.to_be_bytes())),
                    xored_port_bin,
                ));
            }
            SocketAddr::V6(ipv6addr) => {
                let address_bin = u128::from_be_bytes(ipv6addr.ip().octets());
                let magic_u8_slice = STUN_5389_MAGIC_NUMBER_U32.to_be_bytes();
                let xorer_u8 = [magic_u8_slice.as_slice(), transaction_id.as_slice()].concat();
                let ipv6_xorer_function: u128 =
                    match xorer_u8.as_slice().read_u128::<NetworkEndian>() {
                        Ok(ipv6_xorer) => ipv6_xorer,
                        Err(e) => {
                            return Err(STUNError {
                                step: STUNStep::STUNEncode,
                                error_type: STUNErrorType::XORObsfucationError,
                                message: e.to_string() + "Error computing ipv6 xorer function",
                            })
                        }
                    };
                let xored_address_bin = address_bin ^ ipv6_xorer_function;
                return Ok(SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::from(xored_address_bin.to_be_bytes())),
                    xored_port_bin,
                ));
            }
        }
    }

    ///Accepts a clean address and puts it in the struct....doesnt't xor it
    ///xoring isnt done until actually doing network stuff
    pub fn new_xor_mapped_address(address: SocketAddr) -> Self {
        Self::XORMappedAddress { address }
    }

    //Input to these function arent going to be
    //from the library user, it is going to be fed data from
    //orchestrator/drive for STUNBody
    pub fn encode_xor_mapped_address(
        &self,
        transaction_id: [u8; 12],
    ) -> Result<Vec<u8>, STUNError> {
        match self {
            Self::XORMappedAddress { address } => {
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
                let xoredAddress =
                    match Self::xorObsfucationDeObsfucation_SocketAdrr(address, transaction_id) {
                        Ok(sock) => sock,
                        Err(e) => return Err(e),
                    };
                let port = u16::from(xoredAddress.port());
                match xoredAddress {
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
            _ => return Err(STUNError {
                step: STUNStep::STUNEncode,
                error_type: STUNErrorType::AttributeTypeMismatch,
                message:
                    "Called encode function for XOR Mapped address on non XOR Mapped address type"
                        .to_string(),
            }),
        };
    }

    fn decode_xor_ip_addr_port(
        cursor: &mut Cursor<&[u8]>,
        family: u32,
        transaction_id: [u8; 12],
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
            let unobsfucated_addr = match Self::xorObsfucationDeObsfucation_SocketAdrr(
                &SocketAddr::new(IpAddr::V4(ip4_addr_obj), port),
                transaction_id,
            ) {
                Ok(u_a) => u_a,
                Err(e) => {
                    return Err(e);
                }
            };
            return Ok(unobsfucated_addr);
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
            let unobsfucated_addr = match Self::xorObsfucationDeObsfucation_SocketAdrr(
                &SocketAddr::new(IpAddr::V6(ip6_addr_obj), port),
                transaction_id,
            ) {
                Ok(u_a) => u_a,
                Err(e) => {
                    return Err(e);
                }
            };
            return Ok(unobsfucated_addr);
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

    pub fn decode_xor_mapped_address(
        cursor: &mut Cursor<&[u8]>,
        transaction_id: [u8; 12],
    ) -> Result<Self, STUNError> {
        match cursor.read_u8() {
            Ok(_) => {
                //first 8 bits can be anything
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
                    match Self::decode_xor_ip_addr_port(cursor, 1, transaction_id) {
                        Ok(socker_addr) => {
                            return Ok(Self::XORMappedAddress {
                                address: socker_addr,
                            });
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };
                } else if bin == 0b0000_0010 {
                    match Self::decode_xor_ip_addr_port(cursor, 2, transaction_id) {
                        Ok(socker_addr) => {
                            return Ok(Self::XORMappedAddress {
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

//[TODO]: Write tests similar to mapped address, atleast for encode
#[cfg(test)]
mod test {
    use super::*;
    use crate::TestFixtures::fixtures::{ STUN_ATTRIBUTE_IPV6_XOR_MAPPED_ADDRESS_BIN, TEST_TRANSACTION_ID};
    #[test]
    fn xor_mapped_address_valid_test_encode() {
        let xor_mapped_addr = STUNAttributesContent::XORMappedAddress {
            address: SocketAddr::new(
                IpAddr::V6(Ipv6Addr::new(
                    0x2001, 0xdb8, 0x1234, 0x5678, 0x11, 0x2233, 0x4455, 0x6677,
                )),
                32853,
            ),
        };
        match xor_mapped_addr.encode_xor_mapped_address(TEST_TRANSACTION_ID) {
            Ok(bin) => {
                assert_eq!(&bin[..], STUN_ATTRIBUTE_IPV6_XOR_MAPPED_ADDRESS_BIN)
            }
            Err(e) => {
                println!("{:?}", e);
                panic!("Found error, unexpected");
            }
        }
        return;
    }
}
