#![allow(non_snake_case)]
mod STUN;
mod STUNBody;
mod STUNContext;
mod STUNError;
mod STUNHeader;
mod STUNSerde; //Interface for encode/decode for STUN
mod TestFixtures;
mod utils;

pub use STUN::stun as stun;
pub use STUNHeader::header as stunHeader;
pub use STUNBody::body as stunBody;
pub use STUNContext::context as stunContext;
pub use STUNSerde::encode as stunEncode;
pub use STUNSerde::decode as stunDecode;
pub use STUNBody::attributes::attributes as stunAttributes;

#[macro_use]
extern crate num_derive;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
