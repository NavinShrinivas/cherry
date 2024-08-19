//====From here on we have indivdual attributes for testing, along with the header and content====
//These do not have the attribute header, only the value

//(Normally) Mapped address:  192.0.2.1 port 32853
pub const STUN_ATTRIBUTE_IPV4_MAPPED_ADDRESS_BIN: [u8;8] = [
     0x00, 0x01, 0x80, 0x55,
     0xc0, 0x00, 0x02, 0x01,
];

//(XOR) Mapped address:  2001:db8:1234:5678:11:2233:4455:6677 port 32853
pub const STUN_ATTRIBUTE_IPV6_XOR_MAPPED_ADDRESS_BIN: [u8;20] = [
     0x00, 0x02, 0xa1, 0x47,
     0x01, 0x13, 0xa9, 0xfa,
     0xa5, 0xd3, 0xf1, 0x79,
     0xbc, 0x25, 0xf4, 0xb5,
     0xbe, 0xd2, 0xb9, 0xd9,
];

//(XOR) Mapped address:  192.0.2.1 port 32853
pub const STUN_ATTRIBUTE_IPV4_XOR_MAPPED_ADDRESS_BIN: [u8;8] = [
     0x00, 0x01, 0xa1, 0x47,
     0xe1, 0x12, 0xa6, 0x43,
];
