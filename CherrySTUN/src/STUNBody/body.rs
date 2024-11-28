/*
*       0                   1                   2                   3
      0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |         Type                  |            Length             |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                         Value (variable)                ....
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  Each attribute
  MUST be TLV encoded, with a 16-bit type, 16-bit length, and value.
  Each STUN attribute MUST end on a 32-bit boundary
  ================================type=================================
  As of 5389 message types:
  Comprehension-required range (0x0000-0x7FFF):
    0x0000: (Reserved)
    0x0001: MAPPED-ADDRESS
    0x0002: (Reserved; was RESPONSE-ADDRESS)
    0x0003: (Reserved; was CHANGE-ADDRESS)
    0x0004: (Reserved; was SOURCE-ADDRESS)
    0x0005: (Reserved; was CHANGED-ADDRESS)
    0x0006: USERNAME
    0x0007: (Reserved; was PASSWORD)
    0x0008: MESSAGE-INTEGRITY
    0x0009: ERROR-CODE
    0x000A: UNKNOWN-ATTRIBUTES
    0x000B: (Reserved; was REFLECTED-FROM)
    0x0014: REALM
    0x0015: NONCE
    0x0020: XOR-MAPPED-ADDRESS

  Comprehension-optional range (0x8000-0xFFFF)
    0x8022: SOFTWARE
    0x8023: ALTERNATE-SERVER
    0x8028: FINGERPRINT

  ================================length=================================
  The value in the length field MUST contain the length of the Value
  part of the attribute, prior to padding, measured in bytes.  Since
  STUN aligns attributes on 32-bit boundaries, attributes whose content
  s not a multiple of 4 bytes are padded with 1, 2, or 3 bytes of
  padding so that its value contains a multiple of 4 bytes.  The
  padding bits are ignored, and may be any value.

  ================================MORE INFO=================================
  To learn more about these attribute look into :
  https://datatracker.ietf.org/doc/html/rfc5389#section-15
*
*
* */

use crate::STUNBody::attributes::attributes::STUNAttributeType;
use crate::STUNBody::attributes::attributes::STUNAttributesContent;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::{Cursor, Read, Write};

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct STUNAttributes {
    pub length: u16, //len in equal bin rep, only filled by the decode function. Not expected to be
    //filled by user.
    pub attribute_type: STUNAttributeType,
    pub value: STUNAttributesContent, //Contains mapping to type
    _private: (),                     //To prevent direct construction of this struct
}

#[derive(Debug)]
pub struct STUNBody {
    pub attributes: Vec<STUNAttributes>,
}

impl STUNBody {
    //[TODO] being able to "push" attributes to the vector
    //Also means we need to provide some way to build the attributes
    //And also being able to add MESSAGE-INTEGRITY and FINGERPRINT based on current body
    //And some way to block addition of attributes after MESSAGE-INTEGRITY and FINGERPRINT?
    pub fn new() -> Self {
        STUNBody {
            attributes: Vec::new(),
        }
    }
    pub fn add_new_attribute(
        &mut self,
        new_attribute: STUNAttributesContent,
        attribute_type: STUNAttributeType,
        bin_len: u16,
    ) {
        self.attributes.push(STUNAttributes {
            length: bin_len,
            attribute_type,
            value: new_attribute,
            _private: (),
        });
    }

    ///To be called from encode flows/driver
    pub fn write_attribute_header_to_body_encode(
        content_body: &[u8],
        write_cursor: &mut Cursor<&mut Vec<u8>>,
        attribute_type: STUNAttributeType,
    ) -> Result<(), STUNError> {
        //We can write header, only after we know the `size` of the attribute content
        match write_cursor.write_u16::<NetworkEndian>(attribute_type as u16) {
            Ok(_) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::WriteError,
                    message: e.to_string()
                        + ". Error writing header attribute type when trying to encode.",
                })
            }
        }

        match write_cursor.write_u16::<NetworkEndian>(content_body.len() as u16) {
            Ok(_) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::WriteError,
                    message: e.to_string()
                        + ". Error writing header attribute type when trying to encode.",
                })
            }
        }

        return Ok(());
    }

    pub fn write_current_message_length_to_header(
        write_cursor: &mut std::io::Cursor<&mut Vec<u8>>,
    ) -> Result<(), STUNError> {
        let current_pos = write_cursor.position();
        if current_pos < 20 {
            return Err(STUNError {
                step: STUNStep::STUNUtils,
                error_type: STUNErrorType::InvalidMessageBinLength,
                message: String::from(
                    "Length of observed message bin is less than 20....that just impossible!",
                ),
            });
        }
        write_cursor.set_position(2);
        let len_byte_rep: [u8; 2] = (current_pos as u16 - 20 as u16).to_be_bytes();
        match write_cursor.write_all(&len_byte_rep) {
            //write 2 bytes....as length
            Ok(_) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNUtils,
                    error_type: STUNErrorType::WriteError,
                    message: String::from(
                        "Error writing modified length to cursor.".to_string()
                            + e.to_string().as_str(),
                    ),
                });
            }
        }
        write_cursor.set_position(current_pos);
        return Ok(());
    }

    pub fn add_pseudo_message_length_to_header(
        write_cursor: &mut std::io::Cursor<&mut Vec<u8>>,
        length_delta: u16,
    ) -> Result<(), STUNError> {
        let current_pos = write_cursor.position();
        write_cursor.set_position(2);
        let mut curr_len = [0; 2];
        match write_cursor.read_exact(&mut curr_len) {
            Ok(()) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNUtils,
                    error_type: STUNErrorType::InvalidMessageBinLength,
                    message: e.to_string() + "Error reading message len to add delta",
                });
            }
        }
        write_cursor.set_position(2);
        let len_byte_rep: [u8; 2] = (u16::from_be_bytes(curr_len) + length_delta).to_be_bytes();
        match write_cursor.write_all(&len_byte_rep) {
            Ok(()) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNUtils,
                    error_type: STUNErrorType::InvalidMessageBinLength,
                    message: e.to_string() + "Error writing message len to add delta",
                });
            }
        }
        write_cursor.set_position(current_pos);
        return Ok(());
    }
    pub fn add_pseudo_message_length_from_current_pos_to_header(
        write_cursor: &mut std::io::Cursor<&mut Vec<u8>>,
        length_delta: u16,
    ) -> Result<(), STUNError> {
        let current_pos = write_cursor.position();
        let current_len = current_pos + 1;
        write_cursor.set_position(2);
        let len_byte_rep: [u8; 2] = (current_len as u16 + length_delta as u16).to_be_bytes();
        match write_cursor.write_all(&len_byte_rep) {
            Ok(()) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNUtils,
                    error_type: STUNErrorType::InvalidMessageBinLength,
                    message: e.to_string() + "Error writing message len to add delta",
                });
            }
        }
        write_cursor.set_position(current_pos);
        return Ok(());
    }
    pub fn padded_len_calculator(length: u16) -> u16 {
        let padded_username_length: u16;
        if length % 4 == 0 {
            padded_username_length = length;
        } else {
            padded_username_length = ((length as f32 / 4.0).ceil() * 4.0) as u16;
        }
        return padded_username_length;
    }
}
