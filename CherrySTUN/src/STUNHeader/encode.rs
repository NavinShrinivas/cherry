use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use crate::STUNHeader::header::STUNHeader;
use crate::STUNSerde::encode::STUNEncode;
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::{Cursor, Write};

impl STUNEncode for STUNHeader {
    fn encode(
        &self,
        write_cursor: &mut Cursor<&mut Vec<u8>>,
    ) -> Result<(), crate::STUNError::error::STUNError> {
        let message_type = self.message_class as u16 | self.message_method as u16;
        let message_len = self.message_length;
        let magic_num = self.magic_number;
        let transaction_id = self.transaction_id;

        match write_cursor.write_u16::<NetworkEndian>(message_type) {
            Ok(_) => {}
            Err(e) => return Err(STUNError::new(
                STUNStep::STUNEncode,
                STUNErrorType::WriteError,
                e.to_string()
                    + ". Error writing message type to binary format while encoding STUNHeader.",
            )),
        }

        match write_cursor.write_u16::<NetworkEndian>(message_len) {
            Ok(_) => {}
            Err(e) => return Err(STUNError::new(
                STUNStep::STUNEncode,
                STUNErrorType::WriteError,
                e.to_string()
                    + "Error writing message length to binary format while encoding STUNHeader.",
            )),
        }

        match write_cursor.write_u32::<NetworkEndian>(magic_num) {
            Ok(_) => {}
            Err(e) => {
                return Err(STUNError::new(
                    STUNStep::STUNEncode,
                    STUNErrorType::WriteError,
                    e.to_string()
                        + "Error writing magic number to binary format while encoding STUNHeader.",
                ))
            }
        }

        match write_cursor.write_all(transaction_id.as_ref()) {
            Ok(_) => {}
            Err(e) => return Err(STUNError::new(
                STUNStep::STUNEncode,
                STUNErrorType::WriteError,
                e.to_string()
                    + "Error writing message length to binary format while encoding STUNHeader.",
            )),
        }

        return Ok(());
    }
}

/*
*       Examples :
*        00 01 00 58 -> 0000_0000_0000_0001 0000_000_0101_1000 | [binding, request][messsage_length: 88 bytes]
         21 12 a4 42 -> MAGIC_NUMBER
         b7 e7 a7 01 |
         bc 34 d6 86 | Transaction ID
         fa 87 df ae |
         -- Raw :
*        00 01 00 58
         21 12 a4 42
         b7 e7 a7 01
         bc 34 d6 86
         fa 87 df ae

         //Not being tested here :
         80 22 00 10
         53 54 55 4e
         20 74 65 73
         74 20 63 6c
         69 65 6e 74
         00 24 00 04
         6e 00 01 ff
         80 29 00 08
         93 2f f9 b1
         51 26 3b 36
         00 06 00 09
         65 76 74 6a
         3a 68 36 76
         59 20 20 20
         00 08 00 14
         9a ea a7 0c
         bf d8 cb 56
         78 1e f2 b5
         b2 d3 f2 49
         c1 b5 71 a2
         80 28 00 04
         e5 7a 3b cf
         -- Length of 88 bytes!
*
* */

#[cfg(test)]
mod test {
    use super::*;
    use crate::STUNHeader::header::{STUNHeader, STUNMessageClass, STUNMessageMethod};
    use crate::TestFixtures::fixtures;

    #[test]
    fn test_stun_header_decode() {
        //We create a header struct and check if the decode matches the fixtures
        //test are run in parallel and should be thread safe. They cannot not duplicate the initialization code. Hence these inits need to be duplicated over tests
        let mut stun_indication_binding_header = STUNHeader::new(
            STUNMessageClass::Indication,
            STUNMessageMethod::Binding,
            Some(fixtures::EXAMPLE_STUN_REQUEST_TRANSACTION_ID),
        );
        stun_indication_binding_header.increment_message_length(88);
        let mut bin: Vec<u8> = Vec::new();
        let mut write_cursor = Cursor::new(&mut bin);
        stun_indication_binding_header
            .encode(&mut write_cursor)
            .unwrap();
        assert_eq!(
            write_cursor.get_ref().to_vec(),
            fixtures::STUN_INDICATION_BINDING_HEADER_BINARY
        );

        let mut stun_request_binding_header = STUNHeader::new(
            STUNMessageClass::Request,
            STUNMessageMethod::Binding,
            Some(fixtures::EXAMPLE_STUN_REQUEST_TRANSACTION_ID),
        );
        stun_request_binding_header.increment_message_length(88);
        let mut bin: Vec<u8> = Vec::new();
        let mut write_cursor = Cursor::new(&mut bin);
        stun_request_binding_header
            .encode(&mut write_cursor)
            .unwrap();
        assert_eq!(
            write_cursor.get_ref().to_vec(),
            fixtures::STUN_REQUEST_BINDING_HEADER_BINARY
        );

        let mut stun_success_binding_response_header = STUNHeader::new(
            STUNMessageClass::ResponseSuccess,
            STUNMessageMethod::Binding,
            Some(fixtures::EXAMPLE_STUN_REQUEST_TRANSACTION_ID),
        );
        stun_success_binding_response_header.increment_message_length(88);
        let mut bin: Vec<u8> = Vec::new();
        let mut write_cursor = Cursor::new(&mut bin);
        stun_success_binding_response_header
            .encode(&mut write_cursor)
            .unwrap();
        assert_eq!(
            write_cursor.get_ref().to_vec(),
            fixtures::STUN_SUCCESS_BINDING_RESPONSE_HEADER_BINARY
        );

        let mut stun_error_binding_response_header = STUNHeader::new(
            STUNMessageClass::ResponseError,
            STUNMessageMethod::Binding,
            Some(fixtures::EXAMPLE_STUN_REQUEST_TRANSACTION_ID),
        );
        stun_error_binding_response_header.increment_message_length(88);
        let mut bin: Vec<u8> = Vec::new();
        let mut write_cursor = Cursor::new(&mut bin);
        stun_error_binding_response_header
            .encode(&mut write_cursor)
            .unwrap();
        assert_eq!(
            write_cursor.get_ref().to_vec(),
            fixtures::STUN_ERROR_BINDING_RESPONSE_HEADER_BINARY
        );
    }
}
