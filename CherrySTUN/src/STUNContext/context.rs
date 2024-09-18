// Be careful about Nones, if the context value is
// required and not present enc/dec will error out
pub struct STUNContext {
    pub username: Option<String>, //Will be filled by decode if provided
    pub password: Option<String>,
    pub nonce: Option<Vec<u8>>, //Will be filled by decode if provided
    pub realm: Option<Vec<u8>>, //Will be filled bt decode if provided
}

//To get username, none and realm filled by the decode functions, you must:
//1. Provide &mut to the decode function
//2. value in STUNContext must be None
//3. Must exists in the bin vector being given from network to decode (obv)

impl STUNContext {
    pub fn new() -> Self {
        return Self {
            username: None,
            password: None,
            nonce: None,
            realm: None,
        };
    }
}
