/*
*
*The key for the HMAC depends on whether long-term or short-term
credentials are in use.  For long-term credentials, the key is 16
bytes:

key = MD5(username ":" realm ":" SASLprep(password))

That is, the 16-byte key is formed by taking the MD5 hash of the
result of concatenating the following five fields: (1) the username,
with any quotes and trailing nulls removed, as taken from the
USERNAME attribute (in which case SASLprep has already been applied);
(2) a single colon; (3) the realm, with any quotes and trailing nulls
removed; (4) a single colon; and (5) the password, with any trailing
nulls removed and after processing using SASLprep.  For example, if
the username was 'user', the realm was 'realm', and the password was
'pass', then the 16-byte HMAC key would be the result of performing
an MD5 hash on the string 'user:realm:pass', the resulting hash being
0x8493fbc53ba582fb4c044c456bdc40eb.

For short-term credentials:

key = SASLprep(password)

Note: HMAC output is 20 bytes, implying we need no padding for message integrity
*
*/

//We expect everything to be filled in the context for message integrity encode

use super::attributes::STUNAttributesContent;
use super::attributes::STUNAuthType;
use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use hmac_sha1;
use std::io::{Cursor, Read, Write};

impl STUNAttributesContent {
    ///Keep the username empty if you want it filled from the context
    ///returns the non padded username bin, use the `add_padding_to_username_bin` to add the
    ///required padding
    pub fn compute_message_integrity(
        &self,
        encode_context: &Option<&STUNContext>,
        message_bin: &[u8],
    ) -> Result<Vec<u8>, STUNError> {
        match self {
            Self::MessageIntegrity { authType } => {
                let bin: Vec<u8> = Vec::new();
                let mut message_integrity_bin = Cursor::new(bin);

                match authType {
                    STUNAuthType::LongTerm => {
                        let hmac_key = match Self::get_hmac_key(encode_context) {
                            Ok(bin) => bin,
                            Err(e) => return Err(e),
                        };
                        println!("{:?} {:?}", hmac_key, message_bin);
                        let hmac_digest: [u8; hmac_sha1::SHA1_DIGEST_BYTES] =
                            hmac_sha1::hmac_sha1(hmac_key.as_slice(), message_bin);
                        match message_integrity_bin.write_all(&hmac_digest) {
                            Ok(()) => {}
                            Err(e) => {
                                return Err(STUNError {
                                    step: STUNStep::STUNEncode,
                                    error_type: STUNErrorType::WriteError,
                                    message: "Error writing username to bin rep. ".to_string()
                                        + e.to_string().as_str(),
                                })
                            }
                        }
                    }
                    STUNAuthType::ShortTerm => {
                        //[TODO]
                        return Err(STUNError {
                            step: STUNStep::STUNEncode,
                            error_type: STUNErrorType::AttributeTypeMismatch,
                            message:
                                "Called encode function for Mapped address on non Mapped address type"
                                .to_string(),
                        });
                    }
                }

                return Ok(message_integrity_bin.get_ref().to_vec());
            }
            _ => {
                return Err(STUNError {
                    step: STUNStep::STUNEncode,
                    error_type: STUNErrorType::AttributeTypeMismatch,
                    message:
                        "Called encode function for MessageIntegrity on non MessageIntegrity type"
                            .to_string(),
                })
            }
        }
    }

    fn get_hmac_key(encode_context: &Option<&STUNContext>) -> Result<Vec<u8>, STUNError> {
        let mut hmac_key_input: String = String::new();
        match encode_context {
            Some(context) => {
                match &context.username {
                    Some(username) => hmac_key_input.push_str(&username),
                    None => {
                        return Err(STUNError {
                            step: STUNStep::STUNEncode,
                            error_type: STUNErrorType::RequiredContextMissingError,
                            message:
                                "Required context (username) to compute MessageIntegrity is missing. Not provided manually and cannot be filled automagically"
                                .to_string(),
                        });
                    }
                };
                hmac_key_input.push(':');

                match &context.realm {
                    Some(realm) => hmac_key_input.push_str(&realm),
                    None => {
                        return Err(STUNError {
                            step: STUNStep::STUNEncode,
                            error_type: STUNErrorType::RequiredContextMissingError,
                            message:
                                "Required context (realm) to compute MessageIntegrity is missing. Not provided manually and cannot be filled automagically"
                                .to_string(),
                        });
                    }
                };
                hmac_key_input.push(':');

                match &context.password {
                    Some(password) => {
                        let pass = match Self::sasl(password.to_string()) {
                            Ok(pass) => pass,
                            Err(e) => return Err(e),
                        };
                        hmac_key_input.push_str(&pass);
                    }
                    None => {
                        return Err(STUNError {
                            step: STUNStep::STUNEncode,
                            error_type: STUNErrorType::RequiredContextMissingError,
                            message:
                                "Required context (password) to compute MessageIntegrity is missing. Not provided manually."
                                .to_string(),
                        });
                    }
                };
            }
            None => {
                return Err(STUNError {
                    step: STUNStep::STUNEncode,
                    error_type: STUNErrorType::RequiredContextMissingError,
                    message:
                        "Required context to compute MessageIntegrity is missing. Not provided manually and cannot be filled automagically"
                        .to_string(),
                });
            }
        };
        println!("{:?}", hmac_key_input);
        let digest = md5::compute(hmac_key_input);
        println!("{:?}", digest);
        return Ok(digest.to_vec());
    }
    pub fn extract_hmac(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, STUNError> {
        let mut hmac_bin = vec![0; 20 as usize];
        match cursor.read_exact(hmac_bin.as_mut_slice()) {
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
        return Ok(hmac_bin);
    }
}
