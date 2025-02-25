use super::candidates::CandidateTrait;
use crate::CherrySTUN::stunClient;
use log::{error, info, warn};
use std::net::{SocketAddr, ToSocketAddrs};

pub struct ServerReflexiveCandidate {
    host_addr: SocketAddr,
    server_reflexive_addr: SocketAddr,
    stun_turn_server: SocketAddr,
}

impl CandidateTrait for ServerReflexiveCandidate {
    fn fetch_info(ip_port: SocketAddr) -> Option<Self> {
        let stun_server = String::from("stunserver2025.stunprotocol.org:3748");
        match stunClient::StunClient::get_server_reflexive_address_custom_stun_server(
            ip_port.port() as u32,
            stun_server,
        ) {
            Ok(addr) => {
                info!("Server reflexive address/public: {:?}", addr);
                warn!("Note: NAT hole to actual peers have not been made...should be done after ICE exchange.");
                warn!("And ICE exchange is done through a singnaling server.");
                warn!("This is just a test to see if the server reflexive address can be fetched.");
                warn!("This is not a complete ICE implementation.");
                let stun_server_sock = addr
                    .to_socket_addrs()
                    .unwrap()
                    .filter(|x| x.is_ipv4())
                    .next()
                    .unwrap();
                Some(ServerReflexiveCandidate {
                    host_addr: ip_port,
                    server_reflexive_addr: addr,
                    stun_turn_server: stun_server_sock,
                })
            }
            Err(e) => {
                error!("{:?}", e);
                None
            }
        }
    }
    fn punch_nat_hole(&self) -> bool {
        // match stunClient::StunClient::NATHolePunching(
        //     8081,
        //     SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)), 2003),
        // ) {
        //     Ok(()) => {}
        //     Err(e) => error!("{:?}", e),
        // }
        todo!()
    }
}
