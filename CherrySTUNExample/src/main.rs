extern crate CherrySTUN;

use CherrySTUN::*;
// use CherrySTUN::stunEncode::STUNEncode;
// use std::io::Cursor;
// use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use log::{info, error, warn};

use simple_logger::SimpleLogger;

fn main() {

    let mut logger = SimpleLogger::new();
    logger = logger.with_level(log::LevelFilter::Info); //Setting default
    logger = logger.env();
    logger.init().unwrap();
    info!("Application Starting...");

    // // Example 1:
    // let mut encode_ctx = stunContext::STUNContext::new();
    // encode_ctx.username = Some("test".to_string());
    // encode_ctx.password = Some("definitely_working".to_string());
    // let mut stun_msg = stun::STUN::new_default(
    //     stunHeader::STUNMessageClass::Request,
    //     stunHeader::STUNMessageMethod::Binding,
    //     None,
    // );

    // stun_msg.body.add_new_attribute(stunAttributes::STUNAttributesContent::XORMappedAddress{
    //     address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,0, 2, 1)),32853)
    // }, stunAttributes::STUNAttributeType::XORMappedAddress, 0);

    // let mut encoded_stun_msg = Vec::new();
    // let mut write_encoded_stun_msg = Cursor::new(&mut encoded_stun_msg);
    // match stun_msg.encode(&mut write_encoded_stun_msg, &Some(&encode_ctx)){
    //     Ok(_) => {
    //         println!("encoded output: {:X?}", encoded_stun_msg);
    //     },
    //     Err(e) => {
    //         println!("{:?}", e);
    //     }
    // }
    // let client = stunClient::StunClient::with_test_stun_server();
    // let local_addr : SocketAddr = "0.0.0.0:3200".parse().unwrap();
    // let udp = UdpSocket::bind(local_addr).unwrap();
    // println!("local {}", udp.local_addr().unwrap());
    // match client.send_request(&udp,stun_msg, encode_ctx){
    //     Ok(x) => {
    //         println!("{:?}", x)
    //     }, 
    //     Err(e) => {
    //         println!("{:?}", e)
    //     }
    // }

    
    // //Example 2: NAT Mapping test:
    // match stunClient::StunClient::test_nat_mapping_type(true, None){
    //     Ok(x) => {
    //         info!("{:?}", x)
    //     }, 
    //     Err(e) => {
    //         error!("{:?}", e)
    //     }
    // }

    //Example 3: Fetching server reflexive address
    match stunClient::StunClient::get_server_reflexive_address(8081){
        Ok(addr) => {
            info!("Server reflexive address/public: {:?}", addr);
            warn!("Note: NAT hole to actual peers have not been made...should be done after ICE exchange.");
            warn!("And ICE exchange is done through a singnaling server.");
            warn!("This is just a test to see if the server reflexive address can be fetched.");
            warn!("This is not a complete ICE implementation.");
        }
        Err(e) => error!("{:?}", e)
    }
    match stunClient::StunClient::NATHolePunching(8081, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,0, 2, 1)),2003)){
        Ok(())=>{},
        Err(e) => error!("{:?}", e),
    }
}
