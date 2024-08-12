mod STUNError;
mod STUNHeader;
mod STUNSerde;
mod TestFixtures;
mod STUNBody;

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
