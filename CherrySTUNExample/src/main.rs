extern crate CherrySTUN;

use CherrySTUN::*;
use CherrySTUN::stunEncode::STUNEncode;
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let mut encode_ctx = stunContext::STUNContext::new();
    encode_ctx.username = Some("test".to_string());
    encode_ctx.password = Some("definitely_working".to_string());
    let mut stun_msg = stun::STUN::new_default(
        stunHeader::STUNMessageClass::ResponseSuccess,
        stunHeader::STUNMessageMethod::Binding,
        None,
    );

    stun_msg.body.add_new_attribute(stunAttributes::STUNAttributesContent::XORMappedAddress{
        address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,0, 2, 1)),32853)
    }, stunAttributes::STUNAttributeType::XORMappedAddress, 0);

    let mut encoded_stun_msg = Vec::new();
    let mut write_encoded_stun_msg = Cursor::new(&mut encoded_stun_msg);
    match stun_msg.encode(&mut write_encoded_stun_msg, &Some(&encode_ctx)){
        Ok(_) => {
            println!("encoded output: {:X?}", encoded_stun_msg);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
    println!("Hello, world!");
}
