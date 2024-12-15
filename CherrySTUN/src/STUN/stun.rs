use crate::STUNBody::body::STUNBody;
use crate::STUNHeader::header::STUNHeader;
use crate::STUNHeader::header::{STUNMessageClass, STUNMessageMethod};

#[derive(Debug, Clone)]
pub struct STUN {
    pub header: STUNHeader,
    pub body: STUNBody,
    _private: (),                 //To protect direct building of this struct
}

#[derive(Debug)]
pub enum STUNNatMappingType{
    EndpointIndependent,
    AddressDependant,
    PortDependant
}

#[derive(Debug)]
pub enum STUNNatFilteringType{
    EndpointIndependentFiltering,
    AddressDependantFiltering,
    AddressAndPortDependantFiltering
}

impl STUN {
    pub fn new(header: STUNHeader, body: STUNBody) -> Self {
        Self {
            header,
            body,
            _private: (),
        }
    }
    ///
    ///A simple way to create a new STUN message
    ///Takes in compulsory fields:
    ///     - Message class
    ///     - Message method
    ///     - Transaction ID (can be None, in which case it is randomly generated)
    ///     - Context (can be None, in which case a new one is used)
    pub fn new_default(
        msg_class: STUNMessageClass,
        msg_method: STUNMessageMethod,
        transaction_id: Option<[u8; 12]>,
    ) -> Self {
        Self {
            header: STUNHeader::new(msg_class, msg_method, transaction_id),
            body: STUNBody::new(),
            _private: (),
        }
    }
}
