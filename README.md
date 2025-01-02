# cherry
The Rust WebRTC thing. This is a mono repo containing the WebRTC stack to achieve self-hosted video/chat platforms! 

As of now, only 1 (the initial protocol) is written, it's called STUN. Present in CherrySTUN.

## CherrySTUN

A un-marshal/marshal implementation for the STUN protocol along with client. Helps finding out type of NAT and whether p2p is possible or not.

## CherryExchange 

A room management and signaling server. Doesn't maintain any state on disk, required to be alive at all times.

## CherryICE 

Contains the ICE agent logic along with other SDP attributes logic.
