use std::net::SocketAddr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, FromPrimitive)]
pub enum STUNAttributeType {
    MappedAddress = 0x0001, //Done
    Username = 0x0006,      //Done
    MessageIntegrity = 0x0008, //Done
    ErrorCode = 0x0009,
    UnknownAttributes = 0x000A, //[TODO]
    Realm = 0x0014,            //Done
    Nonce = 0x0015,            //Done
    XORMappedAddress = 0x0020, //Done
    Fingerprint = 0x8028, //[TODO]
    Software = 0x8022, //[TODO]
    AlternateServer = 0x8023, //[TODO]
}

//To track type of authentication
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum STUNAuthType {
    ShortTerm,
    LongTerm,
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
    Realm { realm: Option<String> },
    Nonce { nonce: Option<String> },
    //As a rule of thumb, attributes that are wrapper in Option can be automagically filled
    //to/from context
    MessageIntegrity { authType: STUNAuthType }, //its used to check validity/fill in the Message Integrity for new messages
                                                 //But the encode/decode function compulsorily needs the STUNContext to be
                                                 //provided
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
            STUNAttributesContent::Nonce { .. } => return STUNAttributeType::Nonce,
            STUNAttributesContent::MessageIntegrity { .. } => {
                return STUNAttributeType::MessageIntegrity
            }
        };
    }
}
