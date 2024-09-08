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
  To learn more about these atrributes look into :
  https://datatracker.ietf.org/doc/html/rfc5389#section-15
*
*
* */

use crate::STUNBody::attributes::attributes::STUNAttributeType;
use crate::STUNBody::attributes::attributes::STUNAttributesContent;

pub struct STUNAttributes {
    length: u16, //len in equvivalent bin rep
    attribute_type: STUNAttributeType,
    value: STUNAttributesContent, //Contains mapping to type
}

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
        });
    }
}
