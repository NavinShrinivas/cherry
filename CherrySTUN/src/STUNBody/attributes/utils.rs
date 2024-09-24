use super::attributes::STUNAttributesContent;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use stringprep::saslprep;

//Used in a lot of user inputted fields that show up in the protocol
impl STUNAttributesContent {
    pub fn sasl(str: String) -> Result<String, STUNError> {
        let inverted_username = match saslprep(&str) {
            Ok(str) => str.to_string(),
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNEncode,
                    error_type: STUNErrorType::SASLPrepError,
                    message: "Error prepping SASL".to_string()
                        + e.to_string().as_str()
                        + "Attempted on: "
                        + str.as_str(),
                })
            }
        };
        return Ok(inverted_username);
    }

    pub fn add_padding_to_attr_bin(attr_bin: &mut Vec<u8>) {
        let length = attr_bin.len() as u16;
        let padded_attr_length: u16;
        if length % 4 == 0{
            padded_attr_length = length as u16;
        }else{
            padded_attr_length = ((length as f32/4.0).ceil() * 4.0)as u16;
        }
        let padding = padded_attr_length - length;
        for _ in 0..padding {
            attr_bin.push(0 as u8); //Adding padding, can be random
        }
    }
}
