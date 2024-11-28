//Heavily inspired and based on https://github.com/vi/rust-stunclient/blob/master/src/lib.rs
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use crate::STUN::stun::STUN;
use crate::STUNError::error::{STUNStep, STUNErrorType, STUNError};
use crate::STUNSerde::{decode::STUNDecode, encode::STUNEncode};
use crate::STUNContext::context::STUNContext;
use std::io::Cursor;

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
    pub fn with_google_stun_server() -> Self {
        use std::net::ToSocketAddrs;
        let stun_server = "stun.12voip.com:3478"
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

    /// Get external (server-reflexive transport address) IP address and port of specified UDP socket
    pub fn send_request(&self, udp: &UdpSocket, stun_message: STUN, mut context: STUNContext) -> Result<STUN, STUNError> {
        let stun_server = self.stun_server;

        let mut encoded_stun_msg = Vec::new();
        let mut write_encoded_stun_msg = Cursor::new(&mut encoded_stun_msg);
        match stun_message.encode(&mut write_encoded_stun_msg, &Some(&context)){
            Ok(_) => {
                println!("encoded output: {:X?}", encoded_stun_msg);
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }

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

        let mut buf = [0; 256];

        let old_read_timeout = match udp.read_timeout(){
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
                match udp.set_read_timeout(old_read_timeout){
                    Ok(_) => {},
                    Err(e) => {
                        return Err(STUNError {
                            step: STUNStep::STUNNetwork,
                            error_type: STUNErrorType::ErrorSettingNetworkTimeout,
                            message: "Error resetting network timeout".to_string()
                                + e.to_string().as_str(),
                        })
                    }
                }
                return Err(STUNError{
                    step: STUNStep::STUNNetwork,
                    error_type: STUNErrorType::NetworkTimeoutError,
                    message: "Network timed out waiting for response".to_string()
                });
            }
            let mt = self.retry_interval.min(deadline - now);
            if Some(mt) != previous_timeout {
                previous_timeout = Some(mt);
                match udp.set_read_timeout(previous_timeout){
                    Ok(_) => {},
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

            println!("Received reply from {:?} {:?}", addr, buf);
            if addr != stun_server {
                continue;
            }
            
            let mut response_binary = Cursor::new(buf);
            let response_stun_msg = match STUN::decode(&mut response_binary, &mut Some(&mut context)){
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

            match udp.set_read_timeout(old_read_timeout){
                Ok(_) => {},
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
