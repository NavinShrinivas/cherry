use std::net::SocketAddr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, FromPrimitive)]
pub enum STUNAttributeType {
    MappedAddress = 0x0001, //Done
    Username = 0x0006, //Done
    MessageIntegrity = 0x0008,
    ErrorCode = 0x0009,
    UnknownAttributes = 0x000A,
    Realm = 0x0014,
    Nonce = 0x0015,
    XORMappedAddress = 0x0020, //Done
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum STUNAttributesContent {
    MappedAddress { address: SocketAddr },
    XORMappedAddress { address: SocketAddr }, //converts the obfuscated bin to socketAddr
    //and stores it
    //We need to have `STUNContext` with username and password (no None)
    //we come across `MessageIntegrity` attributes and optionally for `Username`.
    //If STUNContext attribute was provided we consider username
    //from STUNContext if String is empty in the `username` value in the attribute enc/dec
    //functions
    Username { username: Option<String> },
    Realm {realm: Option<String>},
    Nonce { nonce : Option<String> }
    //As a rule of thumb, attributes that are wrapper in Option can be automagically filled
    //to/from context
}

impl STUNAttributesContent {
    pub fn attribute_type(&self) -> STUNAttributeType {
        match self {
            STUNAttributesContent::MappedAddress { .. } => {
                return STUNAttributeType::MappedAddress;
            }
            STUNAttributesContent::XORMappedAddress { .. } => {
                return STUNAttributeType::XORMappedAddress;
            }
            STUNAttributesContent::Username { .. } => return STUNAttributeType::Username,
            STUNAttributesContent::Realm { .. } => return STUNAttributeType::Realm,
            STUNAttributesContent::Nonce { .. } => return STUNAttributeType::Nonce
        };
    }
}
