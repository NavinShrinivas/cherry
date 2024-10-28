pub fn two_vector_are_identical(v1: Vec<u8>, v2: Vec<u8>) -> bool {
    for (i, v) in v1.iter().enumerate() {
        let second_value = match v2.get(i) {
            Some(v) => v,
            None => return false,
        };
        if v == second_value {
            continue;
        }
    }
    return true;
}
