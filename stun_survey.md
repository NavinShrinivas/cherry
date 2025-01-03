## Tech specs/Reference documents : 

- WebRTC for the curious : https://webrtcforthecurious.com/docs/01-what-why-and-how/ 
- deprecated RFC for STUN : https://datatracker.ietf.org/doc/html/rfc3489#section-11.1
- Testing NAT mapping (RFC5780) [STUN] : https://datatracker.ietf.org/doc/html/rfc5780#page-10
- Message attributes : https://datatracker.ietf.org/doc/html/rfc5389#section-15
- test vectors for STUN requests : https://datatracker.ietf.org/doc/html/rfc5769
- A good post on NAT hole punching: https://stackoverflow.com/questions/22712298/confusion-about-the-stun-server

## Reference projects : 

- https://github.com/Vagr9K/rust-stun-coder/tree/master
- https://github.com/sile/stun_codec

## Useful tools for development : 

- Hex converter : https://www.scadacore.com/tools/programming-calculators/online-hex-converter/

### NAT testing : 

   This will require at most three tests.  In test `I`, the client
   performs the UDP connectivity test.  The server will return its
   alternate address and port in OTHER-ADDRESS in the binding response.
   If OTHER-ADDRESS is not returned, the server does not support this
   usage and this test cannot be run.  The client examines the XOR-
   MAPPED-ADDRESS attribute.  If this address and port are the same as
   the local IP address and port of the socket used to send the request,
   the client knows that it is not `NATed` and the effective mapping will
   be Endpoint-Independent.

   In test `II`, the client sends a Binding Request to the alternate
   address, but primary port.  If the XOR-MAPPED-ADDRESS in the Binding
   Response is the same as test `I` the NAT currently has Endpoint-
   Independent Mapping.  If not, test `III` is performed: the client sends
   a Binding Request to the alternate address and port.  If the XOR-
   MAPPED-ADDRESS matches test `II`, the NAT currently has Address-
   Dependent Mapping; if it doesn't match it currently has Address and
   Port-Dependent Mapping.

### Authentication : 
   In the long-term credential mechanism, the client and server share a
   pre-provisioned username and password and perform a digest challenge/
   response exchange inspired by the one defined for HTTP [RFC7616] but
   differing in details.  In the short-term credential mechanism, the
   client and the server exchange a username and password through some
   out-of-band method prior to the STUN exchange.  For example, in the
   ICE usage [RFC8445], the two endpoints use out-of-band signaling to
   exchange a username and password.  These are used to integrity
   protect and authenticate the request and response.  There is no
   challenge or nonce used.

- Long-Term Credential: A username and associated password that
      represent a shared secret between client and server.  Long-term
      credentials are generally granted to the client when a subscriber
      enrolls in a service and persist until the subscriber leaves the
      service or explicitly changes the credential.

- Long-Term Password: The password from a long-term credential.

- Short-Term Credential: A temporary username and associated password
      that represent a shared secret between client and server. Short-
      term credentials are obtained through some kind of protocol
      mechanism between the client and server, preceding the STUN
      exchange.  A short-term credential has an explicit temporal scope,
      which may be based on a specific amount of time (such as 5
      minutes) or on an event (such as termination of a Session
      Initiation Protocol (SIP) [RFC3261] dialog).  The specific scope
      of a short-term credential is defined by the application usage.

 - Short-Term Password: The password component of a short-term  
      credential.
> The authentication mechanism theory: https://datatracker.ietf.org/doc/html/rfc8489#section-9

- Turns out, the sasl prep is very straight forward. The `<>` being used to represent unicode chars is only as a way to represent them. They dont actually show up in the final string rep of sasl preps.
    - This confused me a lot when trying to test sasl prep from the test vectors

### Message integrity calculation 

hex rep (of body till message integrity with length representing including) :

```
000100602112a44278ad3433c6ad72c029da412e00060012e3839ee38388e383aae38383e382afe382b900000015001c662f2f3439396b39353464364f4c33346f4c394653547679363473410014000b6578616d706c652e6f726700
```

> Note: the length in the header should be set to the length of the message including message integrity (we know this before hand as HMAC is fixed in size)
> >Note 2: This is because, its possible to have fingerprints after message integrity

unicode escaper : https://dencode.com/en/string/unicode-escape (Doesnt normalise the unicode rep)

Example :
The<U+00AD>M<U+00AA>tr<U+2168> ---> The­MªtrⅨ ---(use cyberchef normalise unicode NFKC)---> The MatrIX

> Note:
> NFKC is a not equvivalent to sasl prep. sasl prep (atleast in the above example) removes the space between `The` and `MatrIX`.
> This may have to do with the fact that NFKC is rule that preserves string length.


HMAC key: use CyberChef ----> MD5 | Input: "sasl(username):sasl(realm):sasl(password)" (without quotes)

> sasled string input to MD5: マトリックス:example.org:TheMatrIX

HMAC key value : e8ca7ad59d5eb0518e312911d2dab2a9

HMAC calculation: use Cyberchef ----> `From Hex` ----> `HMAC (key:e8ca7ad59d5eb0518e312911d2dab2a9) (SHA1)` | Input: hex rep of body (given above)

HMAC value: f67024656dd64a3e02b8e0712e85c9a28ca89666

Viola! It's done.
