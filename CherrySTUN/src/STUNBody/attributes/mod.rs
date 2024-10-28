pub mod attributes;

//Induvidual attrs encode/decode cannot be accessed
//Only used by STUNBody encode/decode
mod mapped_address;
mod message_integrity;
mod nonce;
mod realm;
mod username;
mod utils;
mod xor_mapped_address;
