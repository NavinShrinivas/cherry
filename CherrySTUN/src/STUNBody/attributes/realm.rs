use super::attributes::STUNAttributesContent;
use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use std::io::{Cursor, Read, Write};
use stringprep::saslprep;

impl STUNAttributesContent {
    ///To be used if you have non sasled realm...often new realm
    ///Usage of this function will low
    pub fn new_realm(realm: String) -> Self {
        Self::Realm { realm: Some(realm) }
    }
    //=> We store non sasled realm in mem, needs to be sasled before encode

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

    pub fn new_realm_from_sasled_string(sasled_realm: String) -> Result<Self, STUNError> {
        let clear_realm = match saslprep(&sasled_realm) {
            Ok(str) => str.to_string(),
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNEncode,
                    error_type: STUNErrorType::SASLPrepError,
                    message: "Error prepping SASL".to_string()
                        + e.to_string().as_str()
                        + "Attempted on: "
                        + sasled_realm.as_str(),
                })
            }
        };
        Ok(Self::Realm {
            realm: Some(clear_realm), //We only store non sasled string in memory
        })
    }

    ///Keep the realm empty if you want it filled from the context
    ///returns the non padded realm bin, use the `add_padding_to_realm_bin` to add the
    ///required padding
    pub fn encode_realm(
        &self,
        encode_context: &Option<&STUNContext>,
    ) -> Result<Vec<u8>, STUNError> {
        match self {
            Self::Realm { realm } => {
                let bin: Vec<u8> = Vec::new();
                let mut write_cursor = Cursor::new(bin);
                match realm {
                    Some(realm_string) => {
                        //moving from context to processing will mostly always require a clone as
                        //all those properties are heap allocated
                        let sasled_realm = match Self::sasl(realm_string.clone()) {
                            Ok(str) => str,
                            Err(e) => {
                                return Err(e);
                            }
                        };
                        let realm_bin = sasled_realm.clone().into_bytes();
                        match write_cursor.write_all(&realm_bin[..]) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::WriteError,
                                    message: "Error writing realm to bin rep. ".to_string()
                                        + e.to_string().as_str(),
                                })
                            }
                        };
                    }
                    None => {
                        let realm_string = match encode_context {
                            Some(str) => {
                                match &str.realm{
                                    Some(realm) => {
                                        realm
                                    },
                                    None => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::RequiredContextMissingError,
                                    message: "Found context, but no realm present in context. Either context needs to be filled with realm or it must be provided explicitly."
                                        .to_string(),
                                })

                                    }
                                }
                            },
                            None => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::RequiredContextMissingError,
                                    message: "Did not find context or realm. Any one needs to be provided."
                                        .to_string(),
                                })
                            }
                        };
                        let sasled_realm = match Self::sasl(realm_string.clone()) {
                            Ok(str) => str,
                            Err(e) => {
                                return Err(e);
                            }
                        };
                        let realm_bin = sasled_realm.into_bytes();
                        match write_cursor.write_all(&realm_bin[..]) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::WriteError,
                                    message: "Error writing realm to bin rep. ".to_string()
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
                    message: "Called encode function for Realm on non realm type".to_string(),
                })
            }
        }
    }

    pub fn decode_realm(
        cursor: &mut Cursor<&[u8]>,
        decode_context: &mut Option<&mut STUNContext>,
        length: u16,
    ) -> Result<Self, STUNError> {
        let padded_realm_length: u16;
        if length % 4 == 0 {
            padded_realm_length = length;
        } else {
            padded_realm_length = ((length as f32 / 4.0).ceil() * 4.0) as u16;
        }
        let mut realm_with_padding = vec![0; padded_realm_length as usize];
        match cursor.read_exact(realm_with_padding.as_mut_slice()) {
            Ok(_) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::ReadError,
                    message: "Error reading realm from bin rep. ".to_string()
                        + e.to_string().as_str()
                        + padded_realm_length.to_string().as_str(),
                })
            }
        };
        let realm_without_padding: Vec<u8> = realm_with_padding[..length as usize]
            .iter()
            .cloned()
            .collect();
        let realm_string = match String::from_utf8(realm_without_padding) {
            Ok(str) => str,
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNDecode,
                    error_type: STUNErrorType::UTF8DecodeError,
                    message: "Error reading realm from bin rep. ".to_string()
                        + e.to_string().as_str(),
                })
            }
        };
        //We only store sasled removed real in memory, this allows the user to input raw
        //unicode for realm values and we handle the sasling in our critical path
        let sasled_realm = match Self::sasl(realm_string.clone()) {
            Ok(str) => str,
            Err(e) => {
                return Err(e);
            }
        };
        match decode_context {
            //Filling context if provided and not filled before
            Some(context) => {
                if context.realm == None {
                    context.realm = Some(sasled_realm.clone());
                }
            }
            None => {}
        }
        let realm_attribute = Self::Realm {
            realm: Some(realm_string),
        };
        return Ok(realm_attribute);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestFixtures::fixtures::*;

    #[test]
    fn test_realm_from_sasled_string() {
        let sasled_string = String::from_utf8(REALM_BODY.to_vec()).unwrap();
        let realm_attr =
            STUNAttributesContent::new_realm_from_sasled_string(sasled_string[..11].to_string());
        //slice to avoid padding before feeding into creation
        match realm_attr {
            Ok(usern) => match usern {
                STUNAttributesContent::Realm { realm } => {
                    let expected = "example.org";
                    assert_eq!(realm, Some(expected.to_string()));
                }
                _ => {
                    panic!("Found error, unexpected");
                }
            },
            Err(e) => {
                log::error!("{:?}", e);
                panic!("Found error, unexpected");
            }
        }
        return;
    }

    #[test]
    fn test_realm_encode_normal_flow() {
        let test_encode_context = STUNContext::new();
        let option_encode_context = Some(&test_encode_context);
        let test_realm_attr = STUNAttributesContent::Realm {
            realm: Some(String::from("example.org")),
        };
        match test_realm_attr.encode_realm(&option_encode_context) {
            Ok(mut bin) => {
                STUNAttributesContent::add_padding_to_attr_bin(&mut bin);
                assert_eq!(bin, REALM_BODY);
            }
            Err(e) => {
                log::error!("{:?}", e);
                panic!("Unexpected error...");
            }
        }

        let mut test_encode_context = STUNContext::new();
        test_encode_context.realm = Some("example.org".to_string());
        let option_encode_context = Some(&test_encode_context);
        let test_realm_attr = STUNAttributesContent::Realm { realm: None };
        match test_realm_attr.encode_realm(&option_encode_context) {
            Ok(mut bin) => {
                STUNAttributesContent::add_padding_to_attr_bin(&mut bin);
                assert_eq!(bin, REALM_BODY);
            }
            Err(e) => {
                log::error!("{:?}", e);
                panic!("Unexpected error...");
            }
        }
        return;
    }

    #[test]
    fn test_realm_encode_error_flow() {
        let test_encode_context = STUNContext::new();
        let option_encode_context = Some(&test_encode_context);
        let test_realm_attr = STUNAttributesContent::Realm { realm: None };
        match test_realm_attr.encode_realm(&option_encode_context) {
            Ok(_) => {
                panic!("Expected error, but did not get one.")
            }
            Err(e) => {
                log::error!("{:?}", e);
            }
        }
        return;
    }
}
