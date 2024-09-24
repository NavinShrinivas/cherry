/*
 * nonce contains a sequence of qdtext or quoted-pair, which are defined in
 * RFC 3261 [RFC3261].  Note that this means that the NONCE attribute
 * will not contain actual quote characters.
 * This pretty much mean that we don't have to worry about any thing, rust string parser should
 * handle all the escaped chars
 */

use super::attributes::STUNAttributesContent;
use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use std::io::{Cursor, Read, Write};

impl STUNAttributesContent {
    pub fn new_nonce(nonce: Option<String>) -> Self {
        Self::Nonce { nonce }
    }

    pub fn encode_nonce(
        &self,
        encode_context: &Option<&STUNContext>,
    ) -> Result<Vec<u8>, STUNError> {
        match self {
            Self::Nonce { nonce } => {
                let bin: Vec<u8> = Vec::new();
                let mut write_cursor = Cursor::new(bin);
                match nonce {
                    Some(nonce_string) => {
                        //moving from context to processing will mostly always require a clone as
                        //all those properties are heap allocated

                        let nonce_bin = nonce_string.clone().into_bytes();
                        match write_cursor.write_all(&nonce_bin[..]) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::WriteError,
                                    message: "Error writing nonce to bin rep. ".to_string()
                                        + e.to_string().as_str(),
                                })
                            }
                        };
                    }
                    None => {
                        let nonce_string = match encode_context {
                            Some(str) => {
                                match &str.nonce{
                                    Some(nonce) => {
                                        nonce
                                    },
                                    None => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::RequiredContextMissingError,
                                    message: "Found context, but no nonce present in context. Either context needs to be filled with nonce or it must be provided explicitly to this function."
                                        .to_string(),
                                })

                                    }
                                }
                            },
                            None => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::RequiredContextMissingError,
                                    message: "Did not find context or nonce. Any one needs to be provided."
                                        .to_string(),
                                })
                            }
                        };
                        let nonce_bin = nonce_string.clone().into_bytes();
                        match write_cursor.write_all(&nonce_bin[..]) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::WriteError,
                                    message: "Error writing nonce to bin rep. ".to_string()
                                        + e.to_string().as_str(),
                                })
                            }
                        };
                    }
                };
                return Ok(write_cursor.get_ref().to_vec());
            }
            _ => {
                return Err(STUNError {
                    step: STUNStep::STUNEncode,
                    error_type: STUNErrorType::AttributeTypeMismatch,
                    message: "Called encode function for Nonce on non Nonce type".to_string(),
                })
            }
        }
    }

    pub fn decode_nonce(
        cursor: &mut Cursor<&[u8]>,
        decode_context: &mut Option<&mut STUNContext>,
        length: u16,
    ) -> Result<Self, STUNError> {
        let padded_nonce_length: u16;
        if length % 4 == 0 {
            padded_nonce_length = length;
        } else {
            padded_nonce_length = ((length as f32 / 4.0).ceil() * 4.0) as u16;
        }

        let mut nonce_with_padding = vec![0; padded_nonce_length as usize];
        match cursor.read_exact(nonce_with_padding.as_mut_slice()) {
            Ok(_) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error reading nonce from bin rep. ".to_string()
                        + e.to_string().as_str(),
                })
            }
        };
        let nonce_without_padding: Vec<u8> = nonce_with_padding[..length as usize]
            .iter()
            .cloned()
            .collect();
        let nonce_string = match String::from_utf8(nonce_without_padding) {
            Ok(str) => str,
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::UTF8DecodeError,
                    message: "Error decoding nonce to string utf8. ".to_string()
                        + e.to_string().as_str(),
                })
            }
        };
        match decode_context {
            Some(ctx) => {
                if ctx.nonce == None {
                    ctx.nonce = Some(nonce_string.clone())
                }
            }
            None => {}
        }
        return Ok(Self::Nonce {
            nonce: Some(nonce_string),
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestFixtures::fixtures::*;
    
    #[test]
    fn test_nonce_encode_normal_flow(){
        let test_encode_context = STUNContext::new();
        let option_encode_context = Some(&test_encode_context);
        let test_nonce_attr = STUNAttributesContent::Nonce{
            nonce: Some(String::from("f//499k954d6OL34oL9FSTvy64sA"))
        };
        match test_nonce_attr.encode_nonce(&option_encode_context){
            Ok(mut bin) => {
                STUNAttributesContent::add_padding_to_attr_bin(&mut bin);
                assert_eq!(bin, NONCE_BODY);
            }, 
            Err(e) => {
                println!("{:?}", e);
                panic!("Unexpected error...");
            }
        }


        let mut test_encode_context = STUNContext::new();
        test_encode_context.nonce = Some("f//499k954d6OL34oL9FSTvy64sA".to_string());
        let  option_encode_context = Some(&test_encode_context);
        let test_nonce_attr = STUNAttributesContent::Nonce{
            nonce: None //Asking to be filled from context
        };
        match test_nonce_attr.encode_nonce(&option_encode_context){
            Ok(mut bin) => {
                STUNAttributesContent::add_padding_to_attr_bin(&mut bin);
                assert_eq!(bin, NONCE_BODY);
            }, 
            Err(e) => {
                println!("{:?}", e);
                panic!("Unexpected error...");
            }
        }
        return;
    }

    #[test]
    fn test_nonce_encode_error_flow(){
        let test_encode_context = STUNContext::new();
        let  option_encode_context = Some(&test_encode_context);
        let test_realm_attr = STUNAttributesContent::Nonce{
            nonce: None //Asking to be filled from context, but context also empty
        };
        match test_realm_attr.encode_nonce(&option_encode_context){
            Ok(_) => {
                panic!("Expected error, but did not get one.")
            }, 
            Err(e) => {
                println!("{:?}", e);
            }
        }
        return;
    }
}
