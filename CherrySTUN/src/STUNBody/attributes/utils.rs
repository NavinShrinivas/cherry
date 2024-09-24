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
}
