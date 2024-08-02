// use crate::STUNSerde::decode;
// use crate::STUNHeader::header::STUNHeader;
//
// impl decode::STUNDecode for STUNHeader{
//     fn encode(s: Self) -> Result<Vec<u8>, crate::STUNError::error::STUNError> {
//         let bin : Vec<u8> = Vec::new();
//         let message_type = (s.message_method.into() & 0b1111_1110_1110_1111) | (s.message_class.into() & 0b0000_0001_0001_000);
//     }
//
// }
