## Tech specs/reference documents: 
RFC rfc5245 [ICE Specs]: https://datatracker.ietf.org/doc/html/rfc5245


## Agent roles: 
Consequently, ICE assigns one of the agents in the role of the
   CONTROLLING AGENT, and the other of the CONTROLLED AGENT.  The
   controlling agent gets to nominate which candidate pairs will get
   used for media amongst the ones that are valid.  It can do this in
   one of two ways -- using REGULAR NOMINATION or AGGRESSIVE NOMINATION.

   With regular nomination, the controlling agent lets the checks
   continue until at least one valid candidate pair for each media
   stream is found.  Then, it picks amongst those that are valid, and
   sends a second STUN request on its NOMINATED candidate pair, but this
   time with a flag set to tell the peer that this pair has been
   nominated for use.  This is shown in Figure 4.

## Singlaing infrastructure: 

https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Signaling_and_video_calling
