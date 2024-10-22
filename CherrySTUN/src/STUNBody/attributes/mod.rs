pub mod attributes;

//Induvidual attrs encode/decode cannot be accessed
//Only used by STUNBody encode/decode
mod mapped_address;
mod username;
mod xor_mapped_address;
mod utils;
mod realm;
mod nonce;
mod message_integrity;
