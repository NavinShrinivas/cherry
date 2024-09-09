use std::net::SocketAddr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, FromPrimitive)]
pub enum STUNAttributeType {
    MappedAddress = 0x0001,
    Username = 0x0006,
    MessageIntegrity = 0x0008,
    ErrorCode = 0x0009,
    UnknownAttributes = 0x000A,
    Real = 0x0014,
    Nonce = 0x0015,
    XORMappedAddress = 0x0020, //Mostly used
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum STUNAttributesContent {
    MappedAddress { address: SocketAddr },
    XORMappedAddress { address: SocketAddr }, //converts the obfuscated bin to socketAddr
                                                        //and stores it
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
        };
    }
}
