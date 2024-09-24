use super::attributes::STUNAttributesContent;
use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use std::io::{Cursor, Read, Write};
use stringprep::saslprep;

impl STUNAttributesContent {
    ///To be used if you have non sasled username...often new usernames
    pub fn new_username(username: String) -> Self {
        Self::Username {
            username: Some(username),
        }
    }
    //=> We store non sasled username in mem, needs to be sasled before encode

    // The following table provides examples of how various character data
    // is transformed by the SASLprep string preparation algorithm
    //
    // #  Input            Output     Comments
    // -  -----            ------     --------
    // 1  I<U+00AD>X       IX         SOFT HYPHEN mapped to nothing
    // 2  user             user       no transformation
    // 3  USER             USER       case preserved, will not match #2
    // 4  <U+00AA>         a          output is NFKC, input in ISO 8859-1
    // 5  <U+2168>         IX         output is NFKC, will match #1
    // 6  <U+0007>                    Error - prohibited character
    // 7  <U+0627><U+0031>            Error - bidirectional check
    // This profile is intended to prepare simple user name and password
    // strings for comparison or use in cryptographic functions (e.g.,
    // message digests).  The preparation algorithm was specifically
    // designed such that its output is canonical, and it is well-formed.
    pub fn new_username_from_sasled_string(sasled_username: String) -> Result<Self, STUNError> {
        let clear_username = match saslprep(&sasled_username) {
            Ok(str) => str.to_string(),
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNEncode,
                    error_type: STUNErrorType::SASLPrepError,
                    message: "Error prepping SASL".to_string()
                        + e.to_string().as_str()
                        + "Attempted on: "
                        + sasled_username.as_str(),
                })
            }
        };
        Ok(Self::Username {
            username: Some(clear_username), //We only store non sasled string in memory
        })
    }



    ///Keep the username empty if you want it filled from the context
    ///returns the non padded username bin, use the `add_padding_to_username_bin` to add the
    ///required padding
    pub fn encode_username(
        &self,
        encode_context: &Option<&STUNContext>,
    ) -> Result<Vec<u8>, STUNError> {
        match self {
            Self::Username { username } => {
                let bin: Vec<u8> = Vec::new();
                let mut write_cursor = Cursor::new(bin);
                match username {
                    Some(username_string) => {
                        //moving from context to processing will mostly always require a clone as
                        //all those properties are heap allocated
                        let sasled_username = match Self::sasl(username_string.clone()) {
                            Ok(str) => str,
                            Err(e) => {
                                return Err(e);
                            }
                        };
                        let username_bin = sasled_username.clone().into_bytes();
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
                        let username_string = match encode_context {
                            Some(str) => {
                                match &str.username{
                                    Some(username) => {
                                        username
                                    },
                                    None => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::RequiredContextMissingError,
                                    message: "Found context, but no user name present in context. Either context needs to be filled with username or it must be provided explicitly."
                                        .to_string(),
                                })

                                    }
                                }
                            },
                            None => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::RequiredContextMissingError,
                                    message: "Did not find context or username. Any one needs to be provided."
                                        .to_string(),
                                })
                            }
                        };
                        let sasled_username = match Self::sasl(username_string.clone()) {
                            Ok(str) => str,
                            Err(e) => {
                                return Err(e);
                            }
                        };
                        let username_bin = sasled_username.into_bytes();
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
        decode_context: &mut Option<&mut STUNContext>,
        length: u16,
    ) -> Result<Self, STUNError> {
        let padded_username_length: u16;
        if length % 4 == 0{
            padded_username_length = length;
        }else{
            padded_username_length = ((length as f32/4.0).ceil() * 4.0)as u16;
        }

        let mut username_with_padding = vec![0; padded_username_length as usize];
        match cursor.read_exact(username_with_padding.as_mut_slice()) {
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
        //We only store sasled removed username in memory, this allows the user to input raw
        //unicode for username values and we handle the sasling in our critical path
        let unsasled_username = match Self::sasl(username_string.clone()) {
            Ok(str) => str,
            Err(e) => {
                return Err(e);
            }
        };
        match decode_context {
            //Filling context if provided and not filled before
            Some(context) => {
                if context.username == None {
                    context.username = Some(unsasled_username.clone());
                }
            }
            None => {}
        }
        let username_attribute = Self::Username {
            username: Some(unsasled_username),
        };
        return Ok(username_attribute);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestFixtures::fixtures::*;
    #[test]
    fn test_username_from_sasled_string() {
        let sasled_string = String::from_utf8(USERNAME_BODY.to_vec()).unwrap();
        let username_attr =
            STUNAttributesContent::new_username_from_sasled_string(sasled_string[..18].to_string());
        match username_attr {
            Ok(usern) => match usern {
                STUNAttributesContent::Username { username } => {
                    let expected = "\u{30de}\u{30c8}\u{30ea}\u{30c3}\u{30af}\u{30b9}";
                    assert_eq!(username, Some(expected.to_string()));
                }
                _ => {
                    panic!("Found error, unexpected");
                }
            },
            Err(e) => {
                println!("{:?}", e);
                panic!("Found error, unexpected");
            }
        }

        let sasled_string = String::from_utf8(PSEUDO_PASSWORD_SASL_TEST.to_vec()).unwrap();
        let username_attr =
            STUNAttributesContent::new_username_from_sasled_string(sasled_string[..19].to_string());
        match username_attr {
            Ok(usern) => match usern {
                STUNAttributesContent::Username { username } => {
                    let expected = "The<>M<a>tr<IX>"; //SASLprep changes it a bit
                    assert_eq!(username, Some(expected.to_string()));
                }
                _ => {
                    panic!("Found error, unexpected");
                }
            },
            Err(e) => {
                println!("{:?}", e);
                panic!("Found error, unexpected");
            }
        }
        return;
    }
}
