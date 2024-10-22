//To track type of authentication
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum STUNAuthType{
    ShortTerm, 
    LongTerm
}

/*
 *
 *The key for the HMAC depends on whether long-term or short-term
   credentials are in use.  For long-term credentials, the key is 16
   bytes:

            key = MD5(username ":" realm ":" SASLprep(password))

   That is, the 16-byte key is formed by taking the MD5 hash of the
   result of concatenating the following five fields: (1) the username,
   with any quotes and trailing nulls removed, as taken from the
   USERNAME attribute (in which case SASLprep has already been applied);
   (2) a single colon; (3) the realm, with any quotes and trailing nulls
   removed; (4) a single colon; and (5) the password, with any trailing
   nulls removed and after processing using SASLprep.  For example, if
   the username was 'user', the realm was 'realm', and the password was
   'pass', then the 16-byte HMAC key would be the result of performing
   an MD5 hash on the string 'user:realm:pass', the resulting hash being
   0x8493fbc53ba582fb4c044c456bdc40eb.

   For short-term credentials:

                          key = SASLprep(password)

Note: HMAC output is 20 bytes, implying we need no padding for message integrity
 *
 */

//We expect everything to be filled in the context for message integrity encode

