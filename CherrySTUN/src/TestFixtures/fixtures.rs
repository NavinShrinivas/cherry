/*
 * This file is a modified version of test vectors from :
 * https://datatracker.ietf.org/doc/html/rfc5769
 * This document contains test vectors for ICE usage of STUN which are commonly
 * errored in code.
 * */

#![allow(unused)] //All these fixtures are used in tests and raises unused warning in clippy
#![allow(dead_code)]
// #![cfg_attr(any(), rustfmt::skip)] //To maintain the readable binary formatting


//================header test=====================

pub const EXAMPLE_STUN_REQUEST_TRANSACTION_ID: [u8; 12] = [
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_REQUEST_BINDING_HEADER_BINARY: [u8; 20] = [
    0x00, 0x01, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_INDICATION_BINDING_HEADER_BINARY: [u8; 20] = [
    0x00, 0x11, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_SUCCESS_BINDING_RESPONSE_HEADER_BINARY: [u8; 20] = [
    0x01, 0x01, 0x00, 0x58,
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_ERROR_BINDING_RESPONSE_HEADER_BINARY: [u8; 20] = [
    0x01, 0x11, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_INCORRECT_METHOD_HEADER_BINARY: [u8; 20] = [
    0x01, 0x02, 0x00, 0x58, //0000_0001_0000_0010 -- Incorrect method, not binding
                            //(0000_000X_000X_0001)
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_INCORRECT_MAGIC_NUMBER_HEADER_BINARY: [u8; 20] = [
    0x00, 0x01, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x43, // 0x21 0x12 0xa4 0x42 --Is the corrent magic Number
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_SMALLER_HEADER_BINARY: [u8; 19] = [
    0x00, 0x01, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf,      //Header must alwaus be 20 bytes in size
];


//====test for full bodies with multiple attributes====

pub const STUN_REQUEST_BODY_BIN: [u8;88] = [
    0x80, 0x22, 0x00, 0x10,
    0x53, 0x54, 0x55, 0x4e,
    0x20, 0x74, 0x65, 0x73,
    0x74, 0x20, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74,
    0x00, 0x24, 0x00, 0x04,
    0x6e, 0x00, 0x01, 0xff,
    0x80, 0x29, 0x00, 0x08,
    0x93, 0x2f, 0xf9, 0xb1,
    0x51, 0x26, 0x3b, 0x36,
    0x00, 0x06, 0x00, 0x09,
    0x65, 0x76, 0x74, 0x6a,
    0x3a, 0x68, 0x36, 0x76,
    0x59, 0x20, 0x20, 0x20,
    0x00, 0x08, 0x00, 0x14,
    0x9a, 0xea, 0xa7, 0x0c,
    0xbf, 0xd8, 0xcb, 0x56,
    0x78, 0x1e, 0xf2, 0xb5,
    0xb2, 0xd3, 0xf2, 0x49,
    0xc1, 0xb5, 0x71, 0xa2,
    0x80, 0x28, 0x00, 0x04,
    0xe5, 0x7a, 0x3b, 0xcf,
];

pub const STUN_IPV4_XOR_MAPPED_RESPONSE_BODY_BIN: [u8;60] = [
    0x80, 0x22, 0x00, 0x0b,
    0x74, 0x65, 0x73, 0x74,
    0x20, 0x76, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x20,
    0x00, 0x20, 0x00, 0x08,
    0x00, 0x01, 0xa1, 0x47,
    0xe1, 0x12, 0xa6, 0x43,
    0x00, 0x08, 0x00, 0x14,
    0x2b, 0x91, 0xf5, 0x99,
    0xfd, 0x9e, 0x90, 0xc3,
    0x8c, 0x74, 0x89, 0xf9,
    0x2a, 0xf9, 0xba, 0x53,
    0xf0, 0x6b, 0xe7, 0xd7,
    0x80, 0x28, 0x00, 0x04,
    0xc0, 0x7d, 0x4c, 0x96,
];

pub const STUN_IPV6_XOR_MAPPED_RESPONSE_BODY_BIN: [u8;72] = [
    0x80, 0x22, 0x00, 0x0b,
    0x74, 0x65, 0x73, 0x74,
    0x20, 0x76, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x20,
    0x00, 0x20, 0x00, 0x14,
    0x00, 0x02, 0xa1, 0x47,
    0x01, 0x13, 0xa9, 0xfa,
    0xa5, 0xd3, 0xf1, 0x79,
    0xbc, 0x25, 0xf4, 0xb5,
    0xbe, 0xd2, 0xb9, 0xd9,
    0x00, 0x08, 0x00, 0x14,
    0xa3, 0x82, 0x95, 0x4e,
    0x4b, 0xe6, 0x7b, 0xf1,
    0x17, 0x84, 0xc9, 0x7c,
    0x82, 0x92, 0xc2, 0x75,
    0xbf, 0xe3, 0xed, 0x41,
    0x80, 0x28, 0x00, 0x04,
    0xc8, 0xfb, 0x0b, 0x4c,
];

pub const STUN_REQUEST_BODY_LONG_TERM_AUTH_BIN: [u8;96] = [
    0x00, 0x06, 0x00, 0x12,
    0xe3, 0x83, 0x9e, 0xe3,
    0x83, 0x88, 0xe3, 0x83,
    0xaa, 0xe3, 0x83, 0x83,
    0xe3, 0x82, 0xaf, 0xe3,
    0x82, 0xb9, 0x00, 0x00,
    0x00, 0x15, 0x00, 0x1c,
    0x66, 0x2f, 0x2f, 0x34,
    0x39, 0x39, 0x6b, 0x39,
    0x35, 0x34, 0x64, 0x36,
    0x4f, 0x4c, 0x33, 0x34,
    0x6f, 0x4c, 0x39, 0x46,
    0x53, 0x54, 0x76, 0x79,
    0x36, 0x34, 0x73, 0x41,
    0x00, 0x14, 0x00, 0x0b,
    0x65, 0x78, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x2e,
    0x6f, 0x72, 0x67, 0x00,
    0x00, 0x08, 0x00, 0x14,
    0xf6, 0x70, 0x24, 0x65,
    0x6d, 0xd6, 0x4a, 0x3e,
    0x02, 0xb8, 0xe0, 0x71,
    0x2e, 0x85, 0xc9, 0xa2,
    0x8c, 0xa8, 0x96, 0x66,
];


//================simple attribute test=============
//(Normally) Mapped address:  192.0.2.1 port 32853
pub const STUN_ATTRIBUTE_IPV4_MAPPED_ADDRESS_BIN: [u8; 8] = [
    0x00, 0x01, 0x80, 0x55, 
    0xc0, 0x00, 0x02, 0x01
];

//(XOR) Mapped address:  2001:db8:1234:5678:11:2233:4455:6677 port 32853
pub const STUN_ATTRIBUTE_IPV6_XOR_MAPPED_ADDRESS_BIN: [u8; 20] = [
    0x00, 0x02, 0xa1, 0x47, 
    0x01, 0x13, 0xa9, 0xfa,
    0xa5, 0xd3, 0xf1, 0x79, 
    0xbc, 0x25, 0xf4, 0xb5,
    0xbe, 0xd2, 0xb9, 0xd9,
];

//(XOR) Mapped address:  192.0.2.1 port 32853
pub const STUN_ATTRIBUTE_IPV4_XOR_MAPPED_ADDRESS_BIN: [u8; 8] = [
    0x00, 0x01, 0xa1, 0x47,
    0xe1, 0x12, 0xa6, 0x43
];

//=================simple body tests================

//(Normally) Mapped address:  192.0.2.1 port 32853
pub const STUN_RESPONSE_BODY_TEST: [u8;12] = [
     0x00, 0x01, 0x00, 0x08, //header
     0x00, 0x01, 0x80, 0x55,
     0xc0, 0x00, 0x02, 0x01,
];

pub const STUN_RESPONSE_BODY_FAIL_TEST: [u8;12] = [
     0x23, 0x01, 0x00, 0x08, //header, with invalid attribute type
     0x00, 0x01, 0xa1, 0x47,
     0xe1, 0x12, 0xa6, 0x43,
];
