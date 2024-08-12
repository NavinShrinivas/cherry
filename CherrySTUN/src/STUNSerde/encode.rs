use crate::STUNError::error::STUNError;

pub trait STUNEncode {
    //You need a mutable reference here.
    //Cursor interiorly mutates its posistion
    //This function returns back some struct that is Sized
    //For STUNBody the attributes sizes might not be known
    fn encode(&self) -> Result<Vec<u8>, STUNError>;
}
