use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use crate::STUNSerde::encode;
use byteorder::{NetworkEndian, ReadBytesExt};
use std::io::{Cursor, Read};

impl encode::STUNEncode for STUNAttributesContent {
    fn encode(&self) -> Result<Vec<u8>, STUNError> {
        match self {}
    }
}
