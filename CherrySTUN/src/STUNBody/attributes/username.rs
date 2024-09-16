use super::attributes::STUNAttributesContent;
use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use std::io::{Cursor, Read, Write};

impl STUNAttributesContent {
    pub fn new_username(username: String) -> Self {
        Self::Username { username: Some(username) }
    }

    ///Keep the username empty if you want it filled from the context
    pub fn encode_username(&self, encode_context: STUNContext) -> Result<Vec<u8>, STUNError> {
        match self {
            Self::Username { username } => {
                let bin: Vec<u8> = Vec::new();
                let mut write_cursor = Cursor::new(bin);
                match username {
                    Some(username_string) => {
                        //moving from context to processing will mostly always require a clone as
                        //all those properties are heap allocated
                        let mut username_bin = username_string.clone().into_bytes();
                        let padding = username_bin.len() % 4;
                        for _ in 0..padding {
                            username_bin.push(0 as u8); //Adding padding, can be random
                        }
                        match write_cursor.write_all(&username_bin[..]) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::WriteError,
                                    message: "Error writing username to bin rep. ".to_string()
                                        + e.to_string().as_str(),
                                })
                            }
                        };
                    }
                    None => {
                        let username_string = match encode_context.username{
                            Some(str) => {str}
                            None => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::RequiredContextMissingError,
                                    message: "Did not find required username in provided context ".to_string()
                                })
                            }
                        };
                        let mut username_bin = username_string.into_bytes();
                        let padding = username_bin.len() % 4;
                        for _ in 0..padding {
                            username_bin.push(0 as u8); //Adding padding, can be random
                        }
                        match write_cursor.write_all(&username_bin[..]) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::WriteError,
                                    message: "Error writing username to bin rep. ".to_string()
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
                    message: "Called encode function for Mapped address on non Mapped address type"
                        .to_string(),
                })
            }
        }
    }

    pub fn decode_username(
        cursor: &mut Cursor<&[u8]>,
        decode_context: Option<&mut STUNContext>,
        length: u16,
    ) -> Result<Self, STUNError> {
        let padded_username_length = (length % 4) + length;
        let mut username_with_padding: Vec<u8> =
            Vec::with_capacity(padded_username_length as usize);
        match cursor.read_exact(&mut username_with_padding[..]) {
            Ok(_) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error reading user from bin rep. ".to_string()
                        + e.to_string().as_str(),
                })
            }
        };
        let username_without_padding: Vec<u8> = username_with_padding[..length as usize]
            .iter()
            .cloned()
            .collect();
        let username_string = match String::from_utf8(username_without_padding) {
            Ok(str) => str,
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::UTF8DecodeError,
                    message: "Error reading user from bin rep. ".to_string()
                        + e.to_string().as_str(),
                })
            }
        };
        match decode_context {
            //Filling context if provided and not filled before
            Some(context) => {
                if context.username == None {
                    context.username = Some(username_string.clone());
                }
            }
            None => {}
        }
        let username_attribute = Self::Username {
            username: Some(username_string),
        };
        return Ok(username_attribute);
    }
}
