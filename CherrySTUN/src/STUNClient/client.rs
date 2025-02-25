//Heavily inspired and based on https://github.com/vi/rust-stunclient/blob/master/src/lib.rs
use crate::STUNContext::context::STUNContext;
use crate::STUNError::error::{STUNError, STUNErrorType, STUNStep};
use crate::STUNSerde::{decode::STUNDecode, encode::STUNEncode};
use crate::STUN::stun::{STUNNatMappingType, STUN};
use log::{debug, error, info, warn};
use rand::Rng;
use std::io::Cursor;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration; // 0.8.5

/// Options for querying STUN server
pub struct StunClient {
    /// "End-to-end" timeout for the operation.
    pub timeout: Duration,
    /// How often to repeat STUN binding requests
    pub retry_interval: Duration,
    /// Address of the STUN server
    pub stun_server: SocketAddr,
    /// `SOFTWARE` attribute value in binding request
    pub software: Option<&'static str>,
}

impl StunClient {
    /// A constructor with default parameters
    pub fn new(stun_server: SocketAddr) -> Self {
        StunClient {
            timeout: Duration::from_secs(10),
            retry_interval: Duration::from_secs(1),
            stun_server,
            software: Some("SimpleRustStunClient"),
        }
    }

    /// Use hard coded STUN server `stun.l.google.com:19302`.
    ///
    /// Not for actual use, for tests, prototypes and demos.
    pub fn with_test_stun_server() -> Self {
        use std::net::ToSocketAddrs;
        let stun_server = "stunserver2025.stunprotocol.org:3478"
            .to_socket_addrs()
            .unwrap()
            .filter(|x| x.is_ipv4())
            .next()
            .unwrap();
        StunClient::new(stun_server)
    }

    pub fn client_with_addr(addr: String) -> Self {
        use std::net::ToSocketAddrs;
        let stun_server = addr
            .to_socket_addrs()
            .unwrap()
            .filter(|x| x.is_ipv4())
            .next()
            .unwrap();
        StunClient::new(stun_server)
    }

    /// Set `timeout` field, builder pattern.
    pub fn set_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Set `retry_interval` field, builder pattern.
    pub fn set_retry_interval(&mut self, retry_interval: Duration) -> &mut Self {
        self.retry_interval = retry_interval;
        self
    }

    /// Set `software` field, builder pattern.
    pub fn set_software(&mut self, software: Option<&'static str>) -> &mut Self {
        self.software = software;
        self
    }
}

impl StunClient {
    pub fn send_request(
        &self,
        udp: &UdpSocket,
        stun_message: STUN,
        mut context: STUNContext,
    ) -> Result<STUN, STUNError> {
        let stun_server = self.stun_server;

        let mut encoded_stun_msg = Vec::new();
        let mut write_encoded_stun_msg = Cursor::new(&mut encoded_stun_msg);
        match stun_message.encode(&mut write_encoded_stun_msg, &Some(&context)) {
            Ok(_) => {
                debug!("encoded output: {:X?}", encoded_stun_msg);
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }

        match udp.send_to(&encoded_stun_msg[..], stun_server) {
            Ok(_) => {}
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNNetwork,
                    error_type: STUNErrorType::ErrorSendingMessageToServer,
                    message: "Error making UDP connection to server OR sending message to server: "
                        .to_string()
                        + e.to_string().as_str(),
                })
            }
        }

        let mut buf = [0; 256];

        let old_read_timeout = match udp.read_timeout() {
            Ok(x) => x,
            Err(e) => {
                return Err(STUNError {
                    step: STUNStep::STUNNetwork,
                    error_type: STUNErrorType::NetworkTimeoutError,
                    message: "Error reading timeout from UDP socket".to_string()
                        + e.to_string().as_str(),
                })
            }
        };
        let mut previous_timeout = None;

        use std::time::Instant;

        let deadline = Instant::now() + self.timeout;
        loop {
            let now = Instant::now();
            if now >= deadline {
                match udp.set_read_timeout(old_read_timeout) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(STUNError {
                            step: STUNStep::STUNNetwork,
                            error_type: STUNErrorType::ErrorSettingNetworkTimeout,
                            message: "Error resetting network timeout".to_string()
                                + e.to_string().as_str(),
                        })
                    }
                }
                return Err(STUNError {
                    step: STUNStep::STUNNetwork,
                    error_type: STUNErrorType::NetworkTimeoutError,
                    message: "Network timed out waiting for response".to_string(),
                });
            }
            let mt = self.retry_interval.min(deadline - now);
            if Some(mt) != previous_timeout {
                previous_timeout = Some(mt);
                match udp.set_read_timeout(previous_timeout) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(STUNError {
                            step: STUNStep::STUNNetwork,
                            error_type: STUNErrorType::ErrorSettingNetworkTimeout,
                            message: "Error resetting network timeout".to_string()
                                + e.to_string().as_str(),
                        })
                    }
                }
            }

            let (len, addr) = match udp.recv_from(&mut buf[..]) {
                Ok(x) => x,
                Err(ref e)
                    if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                {
                    //Retrying to send on error
                    match udp.send_to(&encoded_stun_msg[..], stun_server){
                        Ok(_) => {},
                        Err(e) => {
                            return Err(STUNError {
                                step: STUNStep::STUNNetwork,
                                error_type: STUNErrorType::ErrorSendingMessageToServer,
                                message: "Error making UDP connection to server OR sending message to server: ".to_string()
                                    + e.to_string().as_str(),
                            })
                        }
                    }
                    continue;
                }
                Err(e) => {
                    return Err(STUNError {
                        step: STUNStep::STUNNetwork,
                        error_type: STUNErrorType::ErrorReceivingFromServer,
                        message: "Error receiving message back from STUN server: ".to_string()
                            + e.to_string().as_str(),
                    })
                }
            };
            let buf = &buf[0..len];

            debug!("Received reply from {:?} {:?}", addr, buf);
            if addr != stun_server {
                continue;
            }

            let mut response_binary = Cursor::new(buf);
            let response_stun_msg =
                match STUN::decode(&mut response_binary, &mut Some(&mut context)) {
                    Ok(x) => x,
                    Err(e) => {
                        return Err(STUNError {
                            step: STUNStep::STUNDecode,
                            error_type: STUNErrorType::ErrorReceivingFromServer,
                            message: "Error decoding server response".to_string()
                                + e.to_string().as_str(),
                        })
                    }
                };

            match udp.set_read_timeout(old_read_timeout) {
                Ok(_) => {}
                Err(e) => {
                    return Err(STUNError {
                        step: STUNStep::STUNNetwork,
                        error_type: STUNErrorType::ErrorSettingNetworkTimeout,
                        message: "Error resetting network timeout".to_string()
                            + e.to_string().as_str(),
                    })
                }
            }
            return Ok(response_stun_msg);
        }
    }
}
// This will require at most three tests.  In test I, the client
// performs the UDP connectivity test.  The server will return its
// alternate address and port in OTHER-ADDRESS in the binding response.
// If OTHER-ADDRESS is not returned, the server does not support this
// usage and this test cannot be run.  The client examines the XOR-
// MAPPED-ADDRESS attribute.  If this address and port are the same as
// the local IP address and port of the socket used to send the request,
// the client knows that it is not NATed and the effective mapping will
// be Endpoint-Independent.

// In test II, the client sends a Binding Request to the alternate
// address, but primary port.  If the XOR-MAPPED-ADDRESS in the Binding
// Response is the same as test I the NAT currently has Endpoint-
// Independent Mapping.  If not, test III is performed: the client sends
// a Binding Request to the alternate address and port.  If the XOR-
// MAPPED-ADDRESS matches test II, the NAT currently has Address-
// Dependent Mapping; if it doesn't match it currently has Address and
// Port-Dependent Mapping.
//  This will also require at most three tests.  These tests are
//  sensitive to prior state on the NAT.
//
//  In test I, the client performs the UDP connectivity test.  The server
//  will return its alternate address and port in OTHER-ADDRESS in the
//  binding response.  If OTHER-ADDRESS is not returned, the server does
//  not support this usage and this test cannot be run.
//
//  In test II, the client sends a binding request to the primary address
//  of the server with the CHANGE-REQUEST attribute set to change-port
//  and change-IP.  This will cause the server to send its response from
//  its alternate IP address and alternate port.  If the client receives
//  a response, the current behavior of the NAT is Endpoint-Independent
//  Filtering.
//
//  If no response is received, test III must be performed to distinguish
//  between Address-Dependent Filtering and Address and Port-Dependent
//  Filtering.  In test III, the client sends a binding request to the
//  original server address with CHANGE-REQUEST set to change-port.  If
//  the client receives a response, the current behavior is Address-
//  Dependent Filtering; if no response is received, the current behavior
//  is Address and Port-Dependent Filtering.
//
// o  Removed the usage of STUN for NAT type detection and binding
//    lifetime discovery.  These techniques have proven overly brittle
//    due to wider variations in the types of NAT devices than described
//    in this document.  Removed the RESPONSE-ADDRESS, CHANGED-ADDRESS,
//    CHANGE-REQUEST, SOURCE-ADDRESS, and REFLECTED-FROM attributes.

impl StunClient {
    pub fn test_nat_mapping_type(
        verbose: bool,
        port: Option<u32>,
    ) -> Result<STUNNatMappingType, STUNError> {
        let encode_ctx = crate::STUNContext::context::STUNContext::new();
        let stun_msg = crate::STUN::stun::STUN::new_default(
            crate::stunHeader::STUNMessageClass::Request,
            crate::stunHeader::STUNMessageMethod::Binding,
            None,
        );

        let client = self::StunClient::with_test_stun_server();
        let dst_port1 = client.stun_server.port().to_string();
        let src_port = match port {
            Some(s) => s,
            None => rand::thread_rng().gen_range(16834..32768),
        };
        let local_addr: SocketAddr = format!("{}:{}", "0.0.0.0", src_port).parse().unwrap();
        let udp = UdpSocket::bind(local_addr).unwrap();
        info!("client addr : {:?}", local_addr);

        let mut other_addr: SocketAddr =
            "0.0.0.0:0".parse().expect("Error initializing other_Addr");
        let mut client_addr1 = "".to_string();
        //Assuming every connection is behind a NAT
        //test II:
        debug!("first call : {:?}", client.stun_server.to_string());
        match client.send_request(&udp, stun_msg.clone(), encode_ctx.clone()) {
            Ok(res) => {
                for i in res.body.attributes.iter() {
                    match i.value {
                        crate::stunAttributes::STUNAttributesContent::OtherAddress { address } => {
                            debug!("other address: {}", address.to_string());
                            other_addr = address;
                        }
                        crate::stunAttributes::STUNAttributesContent::XORMappedAddress {
                            address,
                        } => {
                            client_addr1 = address.to_string();
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
        let server_addr2 = format!("{}:{}", other_addr.ip().to_string(), dst_port1);
        debug!("second call : {:?}", server_addr2);
        let client2 = self::StunClient::client_with_addr(server_addr2);
        let mut client_addr2 = "".to_string();
        match client2.send_request(&udp, stun_msg.clone(), encode_ctx.clone()) {
            Ok(res) => {
                for i in res.body.attributes.iter() {
                    match i.value {
                        crate::stunAttributes::STUNAttributesContent::XORMappedAddress {
                            address,
                        } => {
                            client_addr2 = address.to_string();
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
        if client_addr1 == client_addr2 {
            info!("Endpoint-Independent mapping! on: {:?}", client_addr2);
            return Ok(STUNNatMappingType::EndpointIndependent);
        }
        //test III:
        let server_addr3 = format!(
            "{}:{}",
            other_addr.ip().to_string(),
            other_addr.port().to_string()
        );

        if verbose {
            info!("third call : {:?}", server_addr3);
        }

        let client3 = self::StunClient::client_with_addr(server_addr3);
        let mut client_addr3 = "".to_string();
        match client3.send_request(&udp, stun_msg, encode_ctx) {
            Ok(res) => {
                for i in res.body.attributes.iter() {
                    match i.value {
                        crate::stunAttributes::STUNAttributesContent::XORMappedAddress {
                            address,
                        } => {
                            client_addr3 = address.to_string();
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
        if client_addr2 == client_addr3 {
            info!("~ Address-Dependent mapping on : {:?}", client_addr3);
            return Ok(STUNNatMappingType::AddressDependant);
        }

        info!(":( Port-Dependent mapping on : {:?}", client_addr3);

        return Ok(STUNNatMappingType::PortDependant);
    }

    /// Get external (server-reflexive transport address) IP address and port of specified UDP socket
    pub fn get_server_reflexive_address(src_port: u32) -> Result<SocketAddr, STUNError> {
        let encode_ctx = crate::STUNContext::context::STUNContext::new();
        let stun_msg = crate::STUN::stun::STUN::new_default(
            crate::stunHeader::STUNMessageClass::Request,
            crate::stunHeader::STUNMessageMethod::Binding,
            None,
        );

        let local_addr: SocketAddr = format!("{}:{}", "0.0.0.0", src_port).parse().unwrap();
        let udp = UdpSocket::bind(local_addr).unwrap();
        info!("client addr : {:?}", local_addr);
        let client = self::StunClient::with_test_stun_server();

        info!(
            "making stun server call : {:?}",
            client.stun_server.to_string()
        );
        match client.send_request(&udp, stun_msg.clone(), encode_ctx.clone()) {
            Ok(res) => {
                for i in res.body.attributes.iter() {
                    match i.value {
                        crate::stunAttributes::STUNAttributesContent::XORMappedAddress {
                            address,
                        } => {
                            return Ok(address);
                        }
                        crate::stunAttributes::STUNAttributesContent::MappedAddress { address } => {
                            return Ok(address);
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        };
        return Err(STUNError {
            step: STUNStep::STUNNetwork,
            error_type: STUNErrorType::DidNotFindExpectedAttribute,
            message: "Did not find XOR mapped or mapped address from server response".to_string(),
        });
    }

    /// Get external (server-reflexive transport address) IP address and port of specified UDP socket with stun server of your choice
    pub fn get_server_reflexive_address_custom_stun_server(
        src_port: u32,
        stun_server: String,
    ) -> Result<SocketAddr, STUNError> {
        let encode_ctx = crate::STUNContext::context::STUNContext::new();
        let stun_msg = crate::STUN::stun::STUN::new_default(
            crate::stunHeader::STUNMessageClass::Request,
            crate::stunHeader::STUNMessageMethod::Binding,
            None,
        );

        let local_addr: SocketAddr = format!("{}:{}", "0.0.0.0", src_port).parse().unwrap();
        let udp = UdpSocket::bind(local_addr).unwrap();
        info!("client addr : {:?}", local_addr);
        let client = self::StunClient::client_with_addr(stun_server);

        info!(
            "making stun server call : {:?}",
            client.stun_server.to_string()
        );
        match client.send_request(&udp, stun_msg.clone(), encode_ctx.clone()) {
            Ok(res) => {
                for i in res.body.attributes.iter() {
                    match i.value {
                        crate::stunAttributes::STUNAttributesContent::XORMappedAddress {
                            address,
                        } => {
                            return Ok(address);
                        }
                        crate::stunAttributes::STUNAttributesContent::MappedAddress { address } => {
                            return Ok(address);
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        };
        return Err(STUNError {
            step: STUNStep::STUNNetwork,
            error_type: STUNErrorType::DidNotFindExpectedAttribute,
            message: "Did not find XOR mapped or mapped address from server response".to_string(),
        });
    }

    ///Should be called after ICE exchange.
    pub fn NATHolePunching(src_port: u32, dst_address: SocketAddr) -> Result<(), STUNError> {
        //Its reccomened to test NAT types for the potential port we are going to use.
        match Self::test_nat_mapping_type(false, Some(src_port)) {
            Ok(STUNNatMappingType::EndpointIndependent) => {
                info!("Detected EndpointIndependent type NATs. Attempting NAT hole punching.")
            }
            Ok(STUNNatMappingType::PortDependant) => {
                warn!("At this time, we dont support NAT hole punching for non EndpointIndependent type NATs");
                return Err(STUNError {
                    step: STUNStep::STUNNetwork,
                    error_type: STUNErrorType::UnsupportedNATType,
                    message: "Cannot punch hole through NAT for Port Dependent types".to_string(),
                });
            }
            Ok(STUNNatMappingType::AddressDependant) => {
                warn!("At this time, we dont support NAT hole punching for non EndpointIndependent type NATs");
                return Err(STUNError {
                    step: STUNStep::STUNNetwork,
                    error_type: STUNErrorType::UnsupportedNATType,
                    message: "Cannot punch hole through NAT for Address Dependent types"
                        .to_string(),
                });
            }
            Err(e) => {
                error!("{:?}", e);
                return Err(e);
            }
        }
        //[TODO]
        return Ok(());
    }
}
