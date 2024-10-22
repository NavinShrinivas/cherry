//[TODO]: To be used, eventually
use crate::STUNBody::body::STUNBody;
use crate::STUNContext::context::STUNContext;
use crate::STUNHeader::header::STUNHeader;
pub struct STUN {
    pub header: STUNHeader,
    pub body: STUNBody,
    pub context: STUNContext,
}

impl STUN {
    pub fn new(header: STUNHeader, body: STUNBody, context: STUNContext) -> Self {
        Self {
            header,
            body,
            context,
        }
    }
}
