pub mod STUNBody;
pub mod STUNError;
pub mod STUNHeader;
pub mod STUNSerde; //Interface for encode/decode for STUN
pub mod STUN;
mod TestFixtures;

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
