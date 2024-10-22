//This file contains the structure of STUNHeader messages

use rand::prelude::*;

/*
* All STUN messages comprise a 20-byte header followed by zero or more
  attributes.  The STUN header contains a STUN message type, message
  length, magic cookie, and transaction ID.

     0                   1                   2                   3
     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    |0 0|     STUN Message Type     |         Message Length        |
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    |                         Magic Cookie                          |
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    |                                                               |
    |                     Transaction ID (96 bits)                  |
    |                                                               |
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                 Figure 2: Format of STUN Message Header

  The most significant 2 bits of every STUN message MUST be zeroes.
  This can be used to differentiate STUN packets from other protocols
  when STUN is multiplexed with other protocols on the same port.

  The message type defines the message class (request, success
  response, error response, or indication) and the message method (the
  primary function) of the STUN message.  Although there are four
  message classes, there are only two types of transactions in STUN:
  request/response transactions (which consist of a request message and
  a response message) and indication transactions (which consist of a
  single indication message).  Response classes are split into error
  and success responses to aid in quickly processing the STUN message.
                      0                 1
                      2  3  4 5 6 7 8 9 0 1 2 3 4 5
                     +--+--+-+-+-+-+-+-+-+-+-+-+-+-+
                0  0 |M |M |M|M|M|C|M|M|M|C|M|M|M|M|
                     |11|10|9|8|7|1|6|5|4|0|3|2|1|0|
                     +--+--+-+-+-+-+-+-+-+-+-+-+-+-+

                C = 00 (Request)
                C = 01 (Indication)
                C = 10 (ResponseSuccess)
                C = 11 (ResponseError)
*
*
* */
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive)]
pub enum STUNMessageClass {
    //First two bits are the fixed bits `00`
    Request = 0b0000_0000_0000_0000,
    Indication = 0b0000_0000_0001_0000,
    ResponseSuccess = 0b0000_0001_0000_0000,
    ResponseError = 0b0000_0001_0001_0000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive)]
pub enum STUNMessageMethod {
    Binding = 0b0000_0000_0000_0001,
}

/*
*
*  The Magic Cookie field MUST contain the fixed value 0x2112A442 in
  network byte order.  In [RFC3489], the 32 bits comprising the Magic
  Cookie field were part of the transaction ID; placing the magic
  cookie in this location allows a server to detect if the client will
  understand certain attributes that were added to STUN by [RFC5389].
  In addition, it aids in distinguishing STUN packets from packets of
  other protocols when STUN is multiplexed with those other protocols
  on the same port.
*
* */

pub const STUN_5389_MAGIC_NUMBER_U32: u32 = 0x2112A442;
pub const STUN_HEADER_TRANSACTION_ID_START_POSITION: u32 = 8;
pub const STUN_HEADER_ENDING_POSITION: u32 = 20;

/*
*   The message length MUST contain the size of the message in bytes, not
  including the 20-byte STUN header.  Since all STUN attributes are
  padded to a multiple of 4 bytes, the last 2 bits of this field are
  always zero.  This provides another way to distinguish STUN packets
  from packets of other protocols.
* */
#[derive(Debug, PartialEq)]
pub struct STUNHeader {
    pub message_class: STUNMessageClass,
    pub message_method: STUNMessageMethod,
    pub message_length: u16, //Filled majorly when body attributes are added
    pub magic_number: u32,
    pub transaction_id: [u8; 12], //12 byte transactionID
    _private: () //To protect direct building of this struct
}

impl STUNHeader {
    pub fn new(
        class: STUNMessageClass,
        method: STUNMessageMethod,
        transaction_id: Option<[u8; 12]>,
    ) -> Self {
        //Checking for transactionID. if None, randomly generate.
        let tid: [u8; 12] = match transaction_id {
            Some(ID) => ID,
            None => {
                let mut rng = thread_rng();
                rng.gen()
            }
        };
        return Self {
            message_class: class,
            message_method: method,
            message_length: 0,
            magic_number: STUN_5389_MAGIC_NUMBER_U32,
            transaction_id: tid,
            _private: ()
        };
    }
    pub fn increment_message_length(&mut self, new_attribute_size: u16) {
        self.message_length += new_attribute_size;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing_stun_header_new_method() {
        let random_id_header =
            STUNHeader::new(STUNMessageClass::Request, STUNMessageMethod::Binding, None);
        assert!(
            matches!(random_id_header, STUNHeader{message_class, message_method, message_length, magic_number,..}
             if message_class == STUNMessageClass::Request
                && message_method == STUNMessageMethod::Binding
                && message_length == 0
                && magic_number == STUN_5389_MAGIC_NUMBER_U32
            )
        );

        let mut rng = thread_rng();
        let id = rng.gen();
        let mut known_id_header = STUNHeader::new(
            STUNMessageClass::Request,
            STUNMessageMethod::Binding,
            Some(id),
        );
        known_id_header.increment_message_length(0b0100); //Incrementing by 4
        assert!(
            matches!(known_id_header, STUNHeader{message_class, message_method, message_length, magic_number, transaction_id, _private:()}
             if message_class == STUNMessageClass::Request
                && message_method == STUNMessageMethod::Binding
                && message_length == 4
                && magic_number == STUN_5389_MAGIC_NUMBER_U32
                && transaction_id == id
            )
        );
    }
}
