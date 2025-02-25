use std::net:SocketAddr;

pub enum CandidateType{
    Host,
    ServerReflexive,
    PeerReflexive,
    Relayed,
}
//Host : 
// - Loopback is avoided 
// - And a bunch of IPv6 restrictions
// ServerReflexive : 
// A Binding response will provide the agent with only a
// server-reflexive candidate (also obtained from the mapped address).
// The base of the server-reflexive candidate is the host candidate from
// which the Allocate or Binding request was sent.
// 0.0.0.0 is not a loopback address, but it's a non-routable meta-address that can be used to indicate a non-applicable target. 127.0.0.1 is the address used for loopback traffic. 

pub trait CandidateTrait{
    fn fetch_info(ip_port : SocketAddr) -> Option<Self> where Self: Sized; //Ip and port to be provided by orchestrator, this
                                             //will be of the host
    //fn start_stun_server(&self, output_channel: ...) -> server_handle
    //fn start_media server(&self) -> media_handle
    fn punch_nat_hole(&self) -> bool { return true; } //Not used for every candidate type
}

//We support only server reflexive Candidates for now
pub struct Candidate<T: CandidateTrait>{
    candidate_type : CandidateType,
    info : T
}
