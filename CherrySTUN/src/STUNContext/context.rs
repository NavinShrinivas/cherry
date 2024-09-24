// Be careful about Nones, if the context value is
// required and not present enc/dec will error out
pub struct STUNContext {
    pub username: Option<String>, //Will be filled by decode if provided
    pub password: Option<String>, //Needs to be provided
    pub nonce: Option<String>, //Will be filled by decode if provided
    pub realm: Option<String>, //Will be filled bt decode if provided
}

//This context allows/makes our serde library to be a little bit smarter

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
