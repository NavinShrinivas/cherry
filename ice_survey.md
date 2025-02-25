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
## After offer traading : 
Once an agent has sent its offer or its answer, that agent MUST be
   prepared to receive both STUN and media packets on each candidate.
   As discussed in Section 11.1, media packets can be sent to a
   candidate prior to its appearance as the default destination for
   media in an offer or answer.

Meaning, there are n number of servers listening for stun messages for each candidate. 

## Singlaing infrastructure: 

https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Signaling_and_video_calling
