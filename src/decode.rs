use libc::{self, abs, memcpy, memset};

use crate::quirc::*;
use crate::version_db::*;

pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;

/* ***********************************************************************
 * Decoder algorithm
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct datastream {
    pub raw: [uint8_t; 8896],
    pub data_bits: libc::c_int,
    pub ptr: libc::c_int,
    pub data: [uint8_t; 8896],
}

/* ***********************************************************************
 * Galois fields
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct galois_field {
    pub p: libc::c_int,
    pub log: *const uint8_t,
    pub exp: *const uint8_t,
}
static mut gf16_exp: [uint8_t; 16] = [
    0x1i32 as uint8_t,
    0x2i32 as uint8_t,
    0x4i32 as uint8_t,
    0x8i32 as uint8_t,
    0x3i32 as uint8_t,
    0x6i32 as uint8_t,
    0xci32 as uint8_t,
    0xbi32 as uint8_t,
    0x5i32 as uint8_t,
    0xai32 as uint8_t,
    0x7i32 as uint8_t,
    0xei32 as uint8_t,
    0xfi32 as uint8_t,
    0xdi32 as uint8_t,
    0x9i32 as uint8_t,
    0x1i32 as uint8_t,
];
static mut gf16_log: [uint8_t; 16] = [
    0i32 as uint8_t,
    0xfi32 as uint8_t,
    0x1i32 as uint8_t,
    0x4i32 as uint8_t,
    0x2i32 as uint8_t,
    0x8i32 as uint8_t,
    0x5i32 as uint8_t,
    0xai32 as uint8_t,
    0x3i32 as uint8_t,
    0xei32 as uint8_t,
    0x9i32 as uint8_t,
    0x7i32 as uint8_t,
    0x6i32 as uint8_t,
    0xdi32 as uint8_t,
    0xbi32 as uint8_t,
    0xci32 as uint8_t,
];
static mut gf16: galois_field = unsafe {
    galois_field {
        p: 15i32,
        log: gf16_log.as_ptr(),
        exp: gf16_exp.as_ptr(),
    }
};

static mut gf256_exp: [uint8_t; 256] = [
    0x1i32 as uint8_t,
    0x2i32 as uint8_t,
    0x4i32 as uint8_t,
    0x8i32 as uint8_t,
    0x10i32 as uint8_t,
    0x20i32 as uint8_t,
    0x40i32 as uint8_t,
    0x80i32 as uint8_t,
    0x1di32 as uint8_t,
    0x3ai32 as uint8_t,
    0x74i32 as uint8_t,
    0xe8i32 as uint8_t,
    0xcdi32 as uint8_t,
    0x87i32 as uint8_t,
    0x13i32 as uint8_t,
    0x26i32 as uint8_t,
    0x4ci32 as uint8_t,
    0x98i32 as uint8_t,
    0x2di32 as uint8_t,
    0x5ai32 as uint8_t,
    0xb4i32 as uint8_t,
    0x75i32 as uint8_t,
    0xeai32 as uint8_t,
    0xc9i32 as uint8_t,
    0x8fi32 as uint8_t,
    0x3i32 as uint8_t,
    0x6i32 as uint8_t,
    0xci32 as uint8_t,
    0x18i32 as uint8_t,
    0x30i32 as uint8_t,
    0x60i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x9di32 as uint8_t,
    0x27i32 as uint8_t,
    0x4ei32 as uint8_t,
    0x9ci32 as uint8_t,
    0x25i32 as uint8_t,
    0x4ai32 as uint8_t,
    0x94i32 as uint8_t,
    0x35i32 as uint8_t,
    0x6ai32 as uint8_t,
    0xd4i32 as uint8_t,
    0xb5i32 as uint8_t,
    0x77i32 as uint8_t,
    0xeei32 as uint8_t,
    0xc1i32 as uint8_t,
    0x9fi32 as uint8_t,
    0x23i32 as uint8_t,
    0x46i32 as uint8_t,
    0x8ci32 as uint8_t,
    0x5i32 as uint8_t,
    0xai32 as uint8_t,
    0x14i32 as uint8_t,
    0x28i32 as uint8_t,
    0x50i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x5di32 as uint8_t,
    0xbai32 as uint8_t,
    0x69i32 as uint8_t,
    0xd2i32 as uint8_t,
    0xb9i32 as uint8_t,
    0x6fi32 as uint8_t,
    0xdei32 as uint8_t,
    0xa1i32 as uint8_t,
    0x5fi32 as uint8_t,
    0xbei32 as uint8_t,
    0x61i32 as uint8_t,
    0xc2i32 as uint8_t,
    0x99i32 as uint8_t,
    0x2fi32 as uint8_t,
    0x5ei32 as uint8_t,
    0xbci32 as uint8_t,
    0x65i32 as uint8_t,
    0xcai32 as uint8_t,
    0x89i32 as uint8_t,
    0xfi32 as uint8_t,
    0x1ei32 as uint8_t,
    0x3ci32 as uint8_t,
    0x78i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xfdi32 as uint8_t,
    0xe7i32 as uint8_t,
    0xd3i32 as uint8_t,
    0xbbi32 as uint8_t,
    0x6bi32 as uint8_t,
    0xd6i32 as uint8_t,
    0xb1i32 as uint8_t,
    0x7fi32 as uint8_t,
    0xfei32 as uint8_t,
    0xe1i32 as uint8_t,
    0xdfi32 as uint8_t,
    0xa3i32 as uint8_t,
    0x5bi32 as uint8_t,
    0xb6i32 as uint8_t,
    0x71i32 as uint8_t,
    0xe2i32 as uint8_t,
    0xd9i32 as uint8_t,
    0xafi32 as uint8_t,
    0x43i32 as uint8_t,
    0x86i32 as uint8_t,
    0x11i32 as uint8_t,
    0x22i32 as uint8_t,
    0x44i32 as uint8_t,
    0x88i32 as uint8_t,
    0xdi32 as uint8_t,
    0x1ai32 as uint8_t,
    0x34i32 as uint8_t,
    0x68i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xbdi32 as uint8_t,
    0x67i32 as uint8_t,
    0xcei32 as uint8_t,
    0x81i32 as uint8_t,
    0x1fi32 as uint8_t,
    0x3ei32 as uint8_t,
    0x7ci32 as uint8_t,
    0xf8i32 as uint8_t,
    0xedi32 as uint8_t,
    0xc7i32 as uint8_t,
    0x93i32 as uint8_t,
    0x3bi32 as uint8_t,
    0x76i32 as uint8_t,
    0xeci32 as uint8_t,
    0xc5i32 as uint8_t,
    0x97i32 as uint8_t,
    0x33i32 as uint8_t,
    0x66i32 as uint8_t,
    0xcci32 as uint8_t,
    0x85i32 as uint8_t,
    0x17i32 as uint8_t,
    0x2ei32 as uint8_t,
    0x5ci32 as uint8_t,
    0xb8i32 as uint8_t,
    0x6di32 as uint8_t,
    0xdai32 as uint8_t,
    0xa9i32 as uint8_t,
    0x4fi32 as uint8_t,
    0x9ei32 as uint8_t,
    0x21i32 as uint8_t,
    0x42i32 as uint8_t,
    0x84i32 as uint8_t,
    0x15i32 as uint8_t,
    0x2ai32 as uint8_t,
    0x54i32 as uint8_t,
    0xa8i32 as uint8_t,
    0x4di32 as uint8_t,
    0x9ai32 as uint8_t,
    0x29i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa4i32 as uint8_t,
    0x55i32 as uint8_t,
    0xaai32 as uint8_t,
    0x49i32 as uint8_t,
    0x92i32 as uint8_t,
    0x39i32 as uint8_t,
    0x72i32 as uint8_t,
    0xe4i32 as uint8_t,
    0xd5i32 as uint8_t,
    0xb7i32 as uint8_t,
    0x73i32 as uint8_t,
    0xe6i32 as uint8_t,
    0xd1i32 as uint8_t,
    0xbfi32 as uint8_t,
    0x63i32 as uint8_t,
    0xc6i32 as uint8_t,
    0x91i32 as uint8_t,
    0x3fi32 as uint8_t,
    0x7ei32 as uint8_t,
    0xfci32 as uint8_t,
    0xe5i32 as uint8_t,
    0xd7i32 as uint8_t,
    0xb3i32 as uint8_t,
    0x7bi32 as uint8_t,
    0xf6i32 as uint8_t,
    0xf1i32 as uint8_t,
    0xffi32 as uint8_t,
    0xe3i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xabi32 as uint8_t,
    0x4bi32 as uint8_t,
    0x96i32 as uint8_t,
    0x31i32 as uint8_t,
    0x62i32 as uint8_t,
    0xc4i32 as uint8_t,
    0x95i32 as uint8_t,
    0x37i32 as uint8_t,
    0x6ei32 as uint8_t,
    0xdci32 as uint8_t,
    0xa5i32 as uint8_t,
    0x57i32 as uint8_t,
    0xaei32 as uint8_t,
    0x41i32 as uint8_t,
    0x82i32 as uint8_t,
    0x19i32 as uint8_t,
    0x32i32 as uint8_t,
    0x64i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x8di32 as uint8_t,
    0x7i32 as uint8_t,
    0xei32 as uint8_t,
    0x1ci32 as uint8_t,
    0x38i32 as uint8_t,
    0x70i32 as uint8_t,
    0xe0i32 as uint8_t,
    0xddi32 as uint8_t,
    0xa7i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa6i32 as uint8_t,
    0x51i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x59i32 as uint8_t,
    0xb2i32 as uint8_t,
    0x79i32 as uint8_t,
    0xf2i32 as uint8_t,
    0xf9i32 as uint8_t,
    0xefi32 as uint8_t,
    0xc3i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x2bi32 as uint8_t,
    0x56i32 as uint8_t,
    0xaci32 as uint8_t,
    0x45i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x9i32 as uint8_t,
    0x12i32 as uint8_t,
    0x24i32 as uint8_t,
    0x48i32 as uint8_t,
    0x90i32 as uint8_t,
    0x3di32 as uint8_t,
    0x7ai32 as uint8_t,
    0xf4i32 as uint8_t,
    0xf5i32 as uint8_t,
    0xf7i32 as uint8_t,
    0xf3i32 as uint8_t,
    0xfbi32 as uint8_t,
    0xebi32 as uint8_t,
    0xcbi32 as uint8_t,
    0x8bi32 as uint8_t,
    0xbi32 as uint8_t,
    0x16i32 as uint8_t,
    0x2ci32 as uint8_t,
    0x58i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x7di32 as uint8_t,
    0xfai32 as uint8_t,
    0xe9i32 as uint8_t,
    0xcfi32 as uint8_t,
    0x83i32 as uint8_t,
    0x1bi32 as uint8_t,
    0x36i32 as uint8_t,
    0x6ci32 as uint8_t,
    0xd8i32 as uint8_t,
    0xadi32 as uint8_t,
    0x47i32 as uint8_t,
    0x8ei32 as uint8_t,
    0x1i32 as uint8_t,
];
static mut gf256_log: [uint8_t; 256] = [
    0i32 as uint8_t,
    0xffi32 as uint8_t,
    0x1i32 as uint8_t,
    0x19i32 as uint8_t,
    0x2i32 as uint8_t,
    0x32i32 as uint8_t,
    0x1ai32 as uint8_t,
    0xc6i32 as uint8_t,
    0x3i32 as uint8_t,
    0xdfi32 as uint8_t,
    0x33i32 as uint8_t,
    0xeei32 as uint8_t,
    0x1bi32 as uint8_t,
    0x68i32 as uint8_t,
    0xc7i32 as uint8_t,
    0x4bi32 as uint8_t,
    0x4i32 as uint8_t,
    0x64i32 as uint8_t,
    0xe0i32 as uint8_t,
    0xei32 as uint8_t,
    0x34i32 as uint8_t,
    0x8di32 as uint8_t,
    0xefi32 as uint8_t,
    0x81i32 as uint8_t,
    0x1ci32 as uint8_t,
    0xc1i32 as uint8_t,
    0x69i32 as uint8_t,
    0xf8i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x8i32 as uint8_t,
    0x4ci32 as uint8_t,
    0x71i32 as uint8_t,
    0x5i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x65i32 as uint8_t,
    0x2fi32 as uint8_t,
    0xe1i32 as uint8_t,
    0x24i32 as uint8_t,
    0xfi32 as uint8_t,
    0x21i32 as uint8_t,
    0x35i32 as uint8_t,
    0x93i32 as uint8_t,
    0x8ei32 as uint8_t,
    0xdai32 as uint8_t,
    0xf0i32 as uint8_t,
    0x12i32 as uint8_t,
    0x82i32 as uint8_t,
    0x45i32 as uint8_t,
    0x1di32 as uint8_t,
    0xb5i32 as uint8_t,
    0xc2i32 as uint8_t,
    0x7di32 as uint8_t,
    0x6ai32 as uint8_t,
    0x27i32 as uint8_t,
    0xf9i32 as uint8_t,
    0xb9i32 as uint8_t,
    0xc9i32 as uint8_t,
    0x9ai32 as uint8_t,
    0x9i32 as uint8_t,
    0x78i32 as uint8_t,
    0x4di32 as uint8_t,
    0xe4i32 as uint8_t,
    0x72i32 as uint8_t,
    0xa6i32 as uint8_t,
    0x6i32 as uint8_t,
    0xbfi32 as uint8_t,
    0x8bi32 as uint8_t,
    0x62i32 as uint8_t,
    0x66i32 as uint8_t,
    0xddi32 as uint8_t,
    0x30i32 as uint8_t,
    0xfdi32 as uint8_t,
    0xe2i32 as uint8_t,
    0x98i32 as uint8_t,
    0x25i32 as uint8_t,
    0xb3i32 as uint8_t,
    0x10i32 as uint8_t,
    0x91i32 as uint8_t,
    0x22i32 as uint8_t,
    0x88i32 as uint8_t,
    0x36i32 as uint8_t,
    0xd0i32 as uint8_t,
    0x94i32 as uint8_t,
    0xcei32 as uint8_t,
    0x8fi32 as uint8_t,
    0x96i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xbdi32 as uint8_t,
    0xf1i32 as uint8_t,
    0xd2i32 as uint8_t,
    0x13i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x83i32 as uint8_t,
    0x38i32 as uint8_t,
    0x46i32 as uint8_t,
    0x40i32 as uint8_t,
    0x1ei32 as uint8_t,
    0x42i32 as uint8_t,
    0xb6i32 as uint8_t,
    0xa3i32 as uint8_t,
    0xc3i32 as uint8_t,
    0x48i32 as uint8_t,
    0x7ei32 as uint8_t,
    0x6ei32 as uint8_t,
    0x6bi32 as uint8_t,
    0x3ai32 as uint8_t,
    0x28i32 as uint8_t,
    0x54i32 as uint8_t,
    0xfai32 as uint8_t,
    0x85i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3di32 as uint8_t,
    0xcai32 as uint8_t,
    0x5ei32 as uint8_t,
    0x9bi32 as uint8_t,
    0x9fi32 as uint8_t,
    0xai32 as uint8_t,
    0x15i32 as uint8_t,
    0x79i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x4ei32 as uint8_t,
    0xd4i32 as uint8_t,
    0xe5i32 as uint8_t,
    0xaci32 as uint8_t,
    0x73i32 as uint8_t,
    0xf3i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x57i32 as uint8_t,
    0x7i32 as uint8_t,
    0x70i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xf7i32 as uint8_t,
    0x8ci32 as uint8_t,
    0x80i32 as uint8_t,
    0x63i32 as uint8_t,
    0xdi32 as uint8_t,
    0x67i32 as uint8_t,
    0x4ai32 as uint8_t,
    0xdei32 as uint8_t,
    0xedi32 as uint8_t,
    0x31i32 as uint8_t,
    0xc5i32 as uint8_t,
    0xfei32 as uint8_t,
    0x18i32 as uint8_t,
    0xe3i32 as uint8_t,
    0xa5i32 as uint8_t,
    0x99i32 as uint8_t,
    0x77i32 as uint8_t,
    0x26i32 as uint8_t,
    0xb8i32 as uint8_t,
    0xb4i32 as uint8_t,
    0x7ci32 as uint8_t,
    0x11i32 as uint8_t,
    0x44i32 as uint8_t,
    0x92i32 as uint8_t,
    0xd9i32 as uint8_t,
    0x23i32 as uint8_t,
    0x20i32 as uint8_t,
    0x89i32 as uint8_t,
    0x2ei32 as uint8_t,
    0x37i32 as uint8_t,
    0x3fi32 as uint8_t,
    0xd1i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x95i32 as uint8_t,
    0xbci32 as uint8_t,
    0xcfi32 as uint8_t,
    0xcdi32 as uint8_t,
    0x90i32 as uint8_t,
    0x87i32 as uint8_t,
    0x97i32 as uint8_t,
    0xb2i32 as uint8_t,
    0xdci32 as uint8_t,
    0xfci32 as uint8_t,
    0xbei32 as uint8_t,
    0x61i32 as uint8_t,
    0xf2i32 as uint8_t,
    0x56i32 as uint8_t,
    0xd3i32 as uint8_t,
    0xabi32 as uint8_t,
    0x14i32 as uint8_t,
    0x2ai32 as uint8_t,
    0x5di32 as uint8_t,
    0x9ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x3ci32 as uint8_t,
    0x39i32 as uint8_t,
    0x53i32 as uint8_t,
    0x47i32 as uint8_t,
    0x6di32 as uint8_t,
    0x41i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1fi32 as uint8_t,
    0x2di32 as uint8_t,
    0x43i32 as uint8_t,
    0xd8i32 as uint8_t,
    0xb7i32 as uint8_t,
    0x7bi32 as uint8_t,
    0xa4i32 as uint8_t,
    0x76i32 as uint8_t,
    0xc4i32 as uint8_t,
    0x17i32 as uint8_t,
    0x49i32 as uint8_t,
    0xeci32 as uint8_t,
    0x7fi32 as uint8_t,
    0xci32 as uint8_t,
    0x6fi32 as uint8_t,
    0xf6i32 as uint8_t,
    0x6ci32 as uint8_t,
    0xa1i32 as uint8_t,
    0x3bi32 as uint8_t,
    0x52i32 as uint8_t,
    0x29i32 as uint8_t,
    0x9di32 as uint8_t,
    0x55i32 as uint8_t,
    0xaai32 as uint8_t,
    0xfbi32 as uint8_t,
    0x60i32 as uint8_t,
    0x86i32 as uint8_t,
    0xb1i32 as uint8_t,
    0xbbi32 as uint8_t,
    0xcci32 as uint8_t,
    0x3ei32 as uint8_t,
    0x5ai32 as uint8_t,
    0xcbi32 as uint8_t,
    0x59i32 as uint8_t,
    0x5fi32 as uint8_t,
    0xb0i32 as uint8_t,
    0x9ci32 as uint8_t,
    0xa9i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x51i32 as uint8_t,
    0xbi32 as uint8_t,
    0xf5i32 as uint8_t,
    0x16i32 as uint8_t,
    0xebi32 as uint8_t,
    0x7ai32 as uint8_t,
    0x75i32 as uint8_t,
    0x2ci32 as uint8_t,
    0xd7i32 as uint8_t,
    0x4fi32 as uint8_t,
    0xaei32 as uint8_t,
    0xd5i32 as uint8_t,
    0xe9i32 as uint8_t,
    0xe6i32 as uint8_t,
    0xe7i32 as uint8_t,
    0xadi32 as uint8_t,
    0xe8i32 as uint8_t,
    0x74i32 as uint8_t,
    0xd6i32 as uint8_t,
    0xf4i32 as uint8_t,
    0xeai32 as uint8_t,
    0xa8i32 as uint8_t,
    0x50i32 as uint8_t,
    0x58i32 as uint8_t,
    0xafi32 as uint8_t,
];
static mut gf256: galois_field = unsafe {
    galois_field {
        p: 255i32,
        log: gf256_log.as_ptr(),
        exp: gf256_exp.as_ptr(),
    }
};
/* ***********************************************************************
 * Polynomial operations
 */
unsafe fn poly_add(
    dst: *mut uint8_t,
    src: *const uint8_t,
    c: uint8_t,
    shift: libc::c_int,
    gf: *const galois_field,
) {
    let log_c: libc::c_int = *(*gf).log.offset(c as isize) as libc::c_int;
    if c == 0 {
        return;
    }
    let mut i = 0i32;
    while i < 64i32 {
        let p: libc::c_int = i + shift;
        let v: uint8_t = *src.offset(i as isize);
        if !(p < 0i32 || p >= 64i32) {
            if !(v == 0) {
                let ref mut fresh0 = *dst.offset(p as isize);
                *fresh0 = (*fresh0 as libc::c_int
                    ^ *(*gf).exp.offset(
                        ((*(*gf).log.offset(v as isize) as libc::c_int + log_c) % (*gf).p) as isize,
                    ) as libc::c_int) as uint8_t
            }
        }
        i += 1
    }
}
unsafe fn poly_eval(s: *const uint8_t, x: uint8_t, gf: *const galois_field) -> uint8_t {
    let mut sum: uint8_t = 0i32 as uint8_t;
    let log_x: uint8_t = *(*gf).log.offset(x as isize);
    if x == 0 {
        return *s.offset(0);
    }
    let mut i = 0i32;
    while i < 64i32 {
        let c: uint8_t = *s.offset(i as isize);
        if !(c == 0) {
            sum = (sum as libc::c_int
                ^ *(*gf).exp.offset(
                    ((*(*gf).log.offset(c as isize) as libc::c_int + log_x as libc::c_int * i)
                        % (*gf).p) as isize,
                ) as libc::c_int) as uint8_t
        }
        i += 1
    }
    return sum;
}
/* ***********************************************************************
 * Berlekamp-Massey algorithm for finding error locator polynomials.
 */
unsafe fn berlekamp_massey(
    s: *const uint8_t,
    N: libc::c_int,
    gf: *const galois_field,
    sigma: *mut uint8_t,
) {
    let mut C: [uint8_t; 64] = [0; 64];
    let mut B: [uint8_t; 64] = [0; 64];
    let mut L: libc::c_int = 0i32;
    let mut m: libc::c_int = 1i32;
    let mut b: uint8_t = 1i32 as uint8_t;

    memset(
        B.as_mut_ptr() as *mut libc::c_void,
        0i32,
        std::mem::size_of::<[uint8_t; 64]>(),
    );
    memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0i32,
        std::mem::size_of::<[uint8_t; 64]>(),
    );
    B[0] = 1i32 as uint8_t;
    C[0] = 1i32 as uint8_t;
    let mut n = 0i32;
    while n < N {
        let mut d: uint8_t = *s.offset(n as isize);
        let mult: uint8_t;
        let mut i = 1i32;
        while i <= L {
            if C[i as usize] as libc::c_int != 0 && *s.offset((n - i) as isize) as libc::c_int != 0
            {
                d = (d as libc::c_int
                    ^ *(*gf).exp.offset(
                        ((*(*gf).log.offset(C[i as usize] as isize) as libc::c_int
                            + *(*gf).log.offset(*s.offset((n - i) as isize) as isize)
                                as libc::c_int)
                            % (*gf).p) as isize,
                    ) as libc::c_int) as uint8_t
            }
            i += 1
        }
        mult = *(*gf).exp.offset(
            (((*gf).p - *(*gf).log.offset(b as isize) as libc::c_int
                + *(*gf).log.offset(d as isize) as libc::c_int)
                % (*gf).p) as isize,
        );
        if d == 0 {
            m += 1
        } else if L * 2i32 <= n {
            let mut T: [uint8_t; 64] = [0; 64];
            memcpy(
                T.as_mut_ptr() as *mut libc::c_void,
                C.as_mut_ptr() as *const libc::c_void,
                std::mem::size_of::<[uint8_t; 64]>(),
            );
            poly_add(C.as_mut_ptr(), B.as_mut_ptr(), mult, m, gf);
            memcpy(
                B.as_mut_ptr() as *mut libc::c_void,
                T.as_mut_ptr() as *const libc::c_void,
                std::mem::size_of::<[uint8_t; 64]>(),
            );
            L = n + 1i32 - L;
            b = d;
            m = 1i32
        } else {
            poly_add(C.as_mut_ptr(), B.as_mut_ptr(), mult, m, gf);
            m += 1
        }
        n += 1
    }
    memcpy(
        sigma as *mut libc::c_void,
        C.as_mut_ptr() as *const libc::c_void,
        64,
    );
}
/* ***********************************************************************
 * Code stream error correction
 *
 * Generator polynomial for GF(2^8) is x^8 + x^4 + x^3 + x^2 + 1
 */
unsafe fn block_syndromes(
    data: *const uint8_t,
    bs: libc::c_int,
    npar: libc::c_int,
    s: *mut uint8_t,
) -> libc::c_int {
    let mut nonzero: libc::c_int = 0i32;
    memset(s as *mut libc::c_void, 0, 64);
    let mut i = 0i32;
    while i < npar {
        let mut j = 0i32;
        while j < bs {
            let c: uint8_t = *data.offset((bs - j - 1i32) as isize);
            if !(c == 0) {
                let ref mut fresh1 = *s.offset(i as isize);
                *fresh1 = (*fresh1 as libc::c_int
                    ^ gf256_exp[((gf256_log[c as usize] as libc::c_int + i * j) % 255i32) as usize]
                        as libc::c_int) as uint8_t
            }
            j += 1
        }
        if *s.offset(i as isize) != 0 {
            nonzero = 1i32
        }
        i += 1
    }
    return nonzero;
}
unsafe fn eloc_poly(
    omega: *mut uint8_t,
    s: *const uint8_t,
    sigma: *const uint8_t,
    npar: libc::c_int,
) {
    memset(omega as *mut libc::c_void, 0i32, 64);
    let mut i = 0i32;
    while i < npar {
        let a: uint8_t = *sigma.offset(i as isize);
        let log_a: uint8_t = gf256_log[a as usize];
        if !(a == 0) {
            let mut j = 0i32;
            while j + 1i32 < 64i32 {
                let b: uint8_t = *s.offset((j + 1i32) as isize);
                if i + j >= npar {
                    break;
                }
                if !(b == 0) {
                    let ref mut fresh2 = *omega.offset((i + j) as isize);
                    *fresh2 = (*fresh2 as libc::c_int
                        ^ gf256_exp[((log_a as libc::c_int + gf256_log[b as usize] as libc::c_int)
                            % 255i32) as usize] as libc::c_int)
                        as uint8_t
                }
                j += 1
            }
        }
        i += 1
    }
}
unsafe fn correct_block(data: *mut uint8_t, ecc: *const quirc_rs_params) -> quirc_decode_error_t {
    let npar: libc::c_int = (*ecc).bs - (*ecc).dw;
    let mut s: [uint8_t; 64] = [0; 64];
    let mut sigma: [uint8_t; 64] = [0; 64];
    let mut sigma_deriv: [uint8_t; 64] = [0; 64];
    let mut omega: [uint8_t; 64] = [0; 64];
    /* Compute syndrome vector */
    if block_syndromes(data, (*ecc).bs, npar, s.as_mut_ptr()) == 0 {
        return QUIRC_SUCCESS;
    }
    berlekamp_massey(s.as_mut_ptr(), npar, &gf256, sigma.as_mut_ptr());
    /* Compute derivative of sigma */
    memset(sigma_deriv.as_mut_ptr() as *mut libc::c_void, 0, 64);
    let mut i = 0i32;
    while i + 1i32 < 64i32 {
        sigma_deriv[i as usize] = sigma[(i + 1i32) as usize];
        i += 2i32
    }
    /* Compute error evaluator polynomial */
    eloc_poly(
        omega.as_mut_ptr(),
        s.as_mut_ptr(),
        sigma.as_mut_ptr(),
        npar - 1i32,
    );
    /* Find error locations and magnitudes */
    i = 0i32;
    while i < (*ecc).bs {
        let xinv: uint8_t = gf256_exp[(255i32 - i) as usize];
        if poly_eval(sigma.as_mut_ptr(), xinv, &gf256) == 0 {
            let sd_x: uint8_t = poly_eval(sigma_deriv.as_mut_ptr(), xinv, &gf256);
            let omega_x: uint8_t = poly_eval(omega.as_mut_ptr(), xinv, &gf256);
            let error: uint8_t = gf256_exp[((255i32 - gf256_log[sd_x as usize] as libc::c_int
                + gf256_log[omega_x as usize] as libc::c_int)
                % 255i32) as usize];
            let ref mut fresh3 = *data.offset(((*ecc).bs - i - 1i32) as isize);
            *fresh3 = (*fresh3 as libc::c_int ^ error as libc::c_int) as uint8_t
        }
        i += 1
    }
    if block_syndromes(data, (*ecc).bs, npar, s.as_mut_ptr()) != 0 {
        return QUIRC_ERROR_DATA_ECC;
    }
    return QUIRC_SUCCESS;
}
unsafe fn format_syndromes(u: uint16_t, s: *mut uint8_t) -> libc::c_int {
    let mut nonzero: libc::c_int = 0i32;
    memset(s as *mut libc::c_void, 0, 64);
    let mut i = 0i32;
    while i < 3i32 * 2i32 {
        *s.offset(i as isize) = 0i32 as uint8_t;
        let mut j = 0i32;
        while j < 15i32 {
            if u as libc::c_int & 1i32 << j != 0 {
                let ref mut fresh4 = *s.offset(i as isize);
                *fresh4 = (*fresh4 as libc::c_int
                    ^ gf16_exp[((i + 1i32) * j % 15i32) as usize] as libc::c_int)
                    as uint8_t
            }
            j += 1
        }
        if *s.offset(i as isize) != 0 {
            nonzero = 1i32
        }
        i += 1
    }
    return nonzero;
}
unsafe fn correct_format(f_ret: *mut uint16_t) -> quirc_decode_error_t {
    let mut u: uint16_t = *f_ret;
    let mut s: [uint8_t; 64] = [0; 64];
    let mut sigma: [uint8_t; 64] = [0; 64];
    /* Evaluate U (received codeword) at each of alpha_1 .. alpha_6
     * to get S_1 .. S_6 (but we index them from 0).
     */
    if format_syndromes(u, s.as_mut_ptr()) == 0 {
        return QUIRC_SUCCESS;
    }
    berlekamp_massey(s.as_mut_ptr(), 3i32 * 2i32, &gf16, sigma.as_mut_ptr());
    /* Now, find the roots of the polynomial */
    let mut i = 0i32;
    while i < 15i32 {
        if poly_eval(sigma.as_mut_ptr(), gf16_exp[(15i32 - i) as usize], &gf16) == 0 {
            u = (u as libc::c_int ^ 1i32 << i) as uint16_t
        }
        i += 1
    }
    if format_syndromes(u, s.as_mut_ptr()) != 0 {
        return QUIRC_ERROR_FORMAT_ECC;
    }
    *f_ret = u;
    return QUIRC_SUCCESS;
}

#[inline]
unsafe fn grid_bit(code: *const quirc_code, x: libc::c_int, y: libc::c_int) -> libc::c_int {
    let p: libc::c_int = y * (*code).size + x;
    (*code).cell_bitmap[(p >> 3i32) as usize] as libc::c_int >> (p & 7i32) & 1i32
}

unsafe fn read_format(
    code: *const quirc_code,
    mut data: *mut quirc_data,
    which: libc::c_int,
) -> quirc_decode_error_t {
    let mut format: uint16_t = 0i32 as uint16_t;
    if which != 0 {
        let mut i = 0i32;
        while i < 7i32 {
            format = ((format as libc::c_int) << 1i32
                | grid_bit(code, 8i32, (*code).size - 1i32 - i)) as uint16_t;
            i += 1
        }
        i = 0i32;
        while i < 8i32 {
            format = ((format as libc::c_int) << 1i32
                | grid_bit(code, (*code).size - 8i32 + i, 8i32)) as uint16_t;
            i += 1
        }
    } else {
        static mut xs: [libc::c_int; 15] = [
            8i32, 8i32, 8i32, 8i32, 8i32, 8i32, 8i32, 8i32, 7i32, 5i32, 4i32, 3i32, 2i32, 1i32,
            0i32,
        ];
        static mut ys: [libc::c_int; 15] = [
            0i32, 1i32, 2i32, 3i32, 4i32, 5i32, 7i32, 8i32, 8i32, 8i32, 8i32, 8i32, 8i32, 8i32,
            8i32,
        ];
        let mut i = 14i32;
        while i >= 0i32 {
            format = ((format as libc::c_int) << 1i32
                | grid_bit(code, xs[i as usize], ys[i as usize])) as uint16_t;
            i -= 1
        }
    }
    format = (format as libc::c_int ^ 0x5412i32) as uint16_t;
    let err = correct_format(&mut format);
    if err as u64 != 0 {
        return err;
    }
    let fdata = (format as libc::c_int >> 10i32) as uint16_t;
    (*data).ecc_level = fdata as libc::c_int >> 3i32;
    (*data).mask = fdata as libc::c_int & 7i32;
    return QUIRC_SUCCESS;
}
unsafe fn mask_bit(mask: libc::c_int, i: libc::c_int, j: libc::c_int) -> libc::c_int {
    match mask {
        0 => return ((i + j) % 2i32 == 0) as libc::c_int,
        1 => return (i % 2i32 == 0) as libc::c_int,
        2 => return (j % 3i32 == 0) as libc::c_int,
        3 => return ((i + j) % 3i32 == 0) as libc::c_int,
        4 => return ((i / 2i32 + j / 3i32) % 2i32 == 0) as libc::c_int,
        5 => return (i * j % 2i32 + i * j % 3i32 == 0) as libc::c_int,
        6 => return ((i * j % 2i32 + i * j % 3i32) % 2i32 == 0) as libc::c_int,
        7 => return ((i * j % 3i32 + (i + j) % 2i32) % 2i32 == 0) as libc::c_int,
        _ => {}
    }
    return 0i32;
}
unsafe fn reserved_cell(version: libc::c_int, i: libc::c_int, j: libc::c_int) -> libc::c_int {
    let ver: *const quirc_version_info =
        &*quirc_version_db.as_ptr().offset(version as isize) as *const quirc_version_info;
    let size: libc::c_int = version * 4i32 + 17i32;
    let mut ai: libc::c_int = -1i32;
    let mut aj: libc::c_int = -1i32;
    /* Finder + format: top left */
    if i < 9i32 && j < 9i32 {
        return 1i32;
    }
    /* Finder + format: bottom left */
    if i + 8i32 >= size && j < 9i32 {
        return 1i32;
    }
    /* Finder + format: top right */
    if i < 9i32 && j + 8i32 >= size {
        return 1i32;
    }
    /* Exclude timing patterns */
    if i == 6i32 || j == 6i32 {
        return 1i32;
    }
    /* Exclude version info, if it exists. Version info sits adjacent to
     * the top-right and bottom-left finders in three rows, bounded by
     * the timing pattern.
     */
    if version >= 7i32 {
        if i < 6i32 && j + 11i32 >= size {
            return 1i32;
        }
        if i + 11i32 >= size && j < 6i32 {
            return 1i32;
        }
    }
    /* Exclude alignment patterns */
    let mut a = 0i32;
    while a < 7i32 && (*ver).apat[a as usize] != 0 {
        let p: libc::c_int = (*ver).apat[a as usize];
        if abs(p - i) < 3i32 {
            ai = a
        }
        if abs(p - j) < 3i32 {
            aj = a
        }
        a += 1
    }
    if ai >= 0i32 && aj >= 0i32 {
        a -= 1;
        if ai > 0i32 && ai < a {
            return 1i32;
        }
        if aj > 0i32 && aj < a {
            return 1i32;
        }
        if aj == a && ai == a {
            return 1i32;
        }
    }
    return 0i32;
}
unsafe fn read_bit(
    code: *const quirc_code,
    data: *mut quirc_data,
    mut ds: *mut datastream,
    i: libc::c_int,
    j: libc::c_int,
) {
    let bitpos: libc::c_int = (*ds).data_bits & 7i32;
    let bytepos: libc::c_int = (*ds).data_bits >> 3i32;
    let mut v: libc::c_int = grid_bit(code, j, i);
    if mask_bit((*data).mask, i, j) != 0 {
        v ^= 1i32
    }
    if v != 0 {
        (*ds).raw[bytepos as usize] =
            ((*ds).raw[bytepos as usize] as libc::c_int | 0x80i32 >> bitpos) as uint8_t
    }
    (*ds).data_bits += 1;
}
unsafe fn read_data(code: *const quirc_code, data: *mut quirc_data, ds: *mut datastream) {
    let mut y: libc::c_int = (*code).size - 1i32;
    let mut x: libc::c_int = (*code).size - 1i32;
    let mut dir: libc::c_int = -1i32;
    while x > 0i32 {
        if x == 6i32 {
            x -= 1
        }
        if reserved_cell((*data).version, y, x) == 0 {
            read_bit(code, data, ds, y, x);
        }
        if reserved_cell((*data).version, y, x - 1i32) == 0 {
            read_bit(code, data, ds, y, x - 1i32);
        }
        y += dir;
        if y < 0i32 || y >= (*code).size {
            dir = -dir;
            x -= 2i32;
            y += dir
        }
    }
}
unsafe fn codestream_ecc(data: *mut quirc_data, mut ds: *mut datastream) -> quirc_decode_error_t {
    let ver: *const quirc_version_info =
        &*quirc_version_db.as_ptr().offset((*data).version as isize) as *const quirc_version_info;
    let sb_ecc: *const quirc_rs_params =
        &*(*ver).ecc.as_ptr().offset((*data).ecc_level as isize) as *const quirc_rs_params;
    let mut lb_ecc: quirc_rs_params = quirc_rs_params {
        bs: 0,
        dw: 0,
        ns: 0,
    };
    let lb_count: libc::c_int =
        ((*ver).data_bytes - (*sb_ecc).bs * (*sb_ecc).ns) / ((*sb_ecc).bs + 1i32);
    let bc: libc::c_int = lb_count + (*sb_ecc).ns;
    let ecc_offset: libc::c_int = (*sb_ecc).dw * bc + lb_count;
    let mut dst_offset: libc::c_int = 0i32;
    memcpy(
        &mut lb_ecc as *mut quirc_rs_params as *mut libc::c_void,
        sb_ecc as *const libc::c_void,
        std::mem::size_of::<quirc_rs_params>(),
    );
    lb_ecc.dw += 1;
    lb_ecc.bs += 1;
    let mut i = 0i32;
    while i < bc {
        let dst: *mut uint8_t = (*ds).data.as_mut_ptr().offset(dst_offset as isize);
        let ecc: *const quirc_rs_params = if i < (*sb_ecc).ns {
            sb_ecc
        } else {
            &mut lb_ecc as *mut quirc_rs_params as *const quirc_rs_params
        };
        let num_ec: libc::c_int = (*ecc).bs - (*ecc).dw;
        let mut j = 0i32;
        while j < (*ecc).dw {
            *dst.offset(j as isize) = (*ds).raw[(j * bc + i) as usize];
            j += 1
        }
        j = 0i32;
        while j < num_ec {
            *dst.offset(((*ecc).dw + j) as isize) = (*ds).raw[(ecc_offset + j * bc + i) as usize];
            j += 1
        }
        let err = correct_block(dst, ecc);
        if err as u64 != 0 {
            return err;
        }
        dst_offset += (*ecc).dw;
        i += 1
    }
    (*ds).data_bits = dst_offset * 8i32;
    return QUIRC_SUCCESS;
}
#[inline]
unsafe fn bits_remaining(ds: *const datastream) -> libc::c_int {
    return (*ds).data_bits - (*ds).ptr;
}
unsafe fn take_bits(mut ds: *mut datastream, mut len: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0i32;
    while len != 0 && (*ds).ptr < (*ds).data_bits {
        let b: uint8_t = (*ds).data[((*ds).ptr >> 3i32) as usize];
        let bitpos: libc::c_int = (*ds).ptr & 7i32;
        ret <<= 1i32;
        if (b as libc::c_int) << bitpos & 0x80i32 != 0 {
            ret |= 1i32
        }
        (*ds).ptr += 1;
        len -= 1
    }
    return ret;
}
unsafe fn numeric_tuple(
    mut data: *mut quirc_data,
    ds: *mut datastream,
    bits: libc::c_int,
    digits: libc::c_int,
) -> libc::c_int {
    if bits_remaining(ds) < bits {
        return -1i32;
    }
    let mut tuple = take_bits(ds, bits);
    let mut i = digits - 1i32;
    while i >= 0i32 {
        (*data).payload[((*data).payload_len + i) as usize] =
            (tuple % 10i32 + '0' as i32) as uint8_t;
        tuple /= 10i32;
        i -= 1
    }
    (*data).payload_len += digits;
    return 0i32;
}
unsafe fn decode_numeric(data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: libc::c_int = 14i32;
    if (*data).version < 10i32 {
        bits = 10i32
    } else if (*data).version < 27i32 {
        bits = 12i32
    }
    let mut count = take_bits(ds, bits);
    if (*data).payload_len + count + 1i32 > 8896i32 {
        return QUIRC_ERROR_DATA_OVERFLOW;
    }
    while count >= 3i32 {
        if numeric_tuple(data, ds, 10i32, 3i32) < 0i32 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 3i32
    }
    if count >= 2i32 {
        if numeric_tuple(data, ds, 7i32, 2i32) < 0i32 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 2i32
    }
    if count != 0 {
        if numeric_tuple(data, ds, 4i32, 1i32) < 0i32 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn alpha_tuple(
    mut data: *mut quirc_data,
    ds: *mut datastream,
    bits: libc::c_int,
    digits: libc::c_int,
) -> libc::c_int {
    if bits_remaining(ds) < bits {
        return -1i32;
    }
    let mut tuple = take_bits(ds, bits);
    let mut i = 0i32;
    while i < digits {
        static mut alpha_map: *const libc::c_char =
            b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:\x00" as *const u8
                as *const libc::c_char;
        (*data).payload[((*data).payload_len + digits - i - 1i32) as usize] =
            *alpha_map.offset((tuple % 45i32) as isize) as uint8_t;
        tuple /= 45i32;
        i += 1
    }
    (*data).payload_len += digits;
    return 0i32;
}
unsafe fn decode_alpha(data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: libc::c_int = 13i32;
    if (*data).version < 10i32 {
        bits = 9i32
    } else if (*data).version < 27i32 {
        bits = 11i32
    }
    let mut count = take_bits(ds, bits);
    if (*data).payload_len + count + 1i32 > 8896i32 {
        return QUIRC_ERROR_DATA_OVERFLOW;
    }
    while count >= 2i32 {
        if alpha_tuple(data, ds, 11i32, 2i32) < 0i32 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 2i32
    }
    if count != 0 {
        if alpha_tuple(data, ds, 6i32, 1i32) < 0i32 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_byte(mut data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: libc::c_int = 16i32;
    if (*data).version < 10i32 {
        bits = 8i32
    }
    let count = take_bits(ds, bits);
    if (*data).payload_len + count + 1i32 > 8896i32 {
        return QUIRC_ERROR_DATA_OVERFLOW;
    }
    if bits_remaining(ds) < count * 8i32 {
        return QUIRC_ERROR_DATA_UNDERFLOW;
    }
    let mut i = 0i32;
    while i < count {
        let fresh5 = (*data).payload_len;
        (*data).payload_len = (*data).payload_len + 1;
        (*data).payload[fresh5 as usize] = take_bits(ds, 8i32) as uint8_t;
        i += 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_kanji(mut data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: libc::c_int = 12i32;
    if (*data).version < 10i32 {
        bits = 8i32
    } else if (*data).version < 27i32 {
        bits = 10i32
    }
    let count = take_bits(ds, bits);
    if (*data).payload_len + count * 2i32 + 1i32 > 8896i32 {
        return QUIRC_ERROR_DATA_OVERFLOW;
    }
    if bits_remaining(ds) < count * 13i32 {
        return QUIRC_ERROR_DATA_UNDERFLOW;
    }
    let mut i = 0i32;
    while i < count {
        let d: libc::c_int = take_bits(ds, 13i32);
        let msB: libc::c_int = d / 0xc0i32;
        let lsB: libc::c_int = d % 0xc0i32;
        let intermediate: libc::c_int = msB << 8i32 | lsB;
        let sjw: uint16_t;
        if intermediate + 0x8140i32 <= 0x9ffci32 {
            /* bytes are in the range 0x8140 to 0x9FFC */
            sjw = (intermediate + 0x8140i32) as uint16_t
        } else {
            /* bytes are in the range 0xE040 to 0xEBBF */
            sjw = (intermediate + 0xc140i32) as uint16_t
        }
        let fresh6 = (*data).payload_len;
        (*data).payload_len = (*data).payload_len + 1;
        (*data).payload[fresh6 as usize] = (sjw as libc::c_int >> 8i32) as uint8_t;
        let fresh7 = (*data).payload_len;
        (*data).payload_len = (*data).payload_len + 1;
        (*data).payload[fresh7 as usize] = (sjw as libc::c_int & 0xffi32) as uint8_t;
        i += 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_eci(mut data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    if bits_remaining(ds) < 8i32 {
        return QUIRC_ERROR_DATA_UNDERFLOW;
    }
    (*data).eci = take_bits(ds, 8i32) as uint32_t;
    if (*data).eci & 0xc0i32 as libc::c_uint == 0x80i32 as libc::c_uint {
        if bits_remaining(ds) < 8i32 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        (*data).eci = (*data).eci << 8i32 | take_bits(ds, 8i32) as libc::c_uint
    } else if (*data).eci & 0xe0i32 as libc::c_uint == 0xc0i32 as libc::c_uint {
        if bits_remaining(ds) < 16i32 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        (*data).eci = (*data).eci << 16i32 | take_bits(ds, 16i32) as libc::c_uint
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_payload(mut data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    while bits_remaining(ds) >= 4i32 {
        let type_0 = take_bits(ds, 4i32);
        let err = match type_0 {
            1 => decode_numeric(data, ds),
            2 => decode_alpha(data, ds),
            4 => decode_byte(data, ds),
            8 => decode_kanji(data, ds),
            7 => decode_eci(data, ds),
            _ => {
                break;
            }
        };
        if err as u64 != 0 {
            return err;
        }
        if type_0 & type_0 - 1i32 == 0 && type_0 > (*data).data_type {
            (*data).data_type = type_0
        }
    }
    /* Add nul terminator to all payloads */
    if (*data).payload_len
        >= ::std::mem::size_of::<[uint8_t; 8896]>() as libc::c_ulong as libc::c_int
    {
        (*data).payload_len -= 1
    }
    (*data).payload[(*data).payload_len as usize] = 0i32 as uint8_t;
    return QUIRC_SUCCESS;
}

/// Decode a QR-code, returning the payload data.
pub unsafe fn quirc_decode(
    code: *const quirc_code,
    mut data: *mut quirc_data,
) -> quirc_decode_error_t {
    let mut ds: datastream = datastream {
        raw: [0; 8896],
        data_bits: 0,
        ptr: 0,
        data: [0; 8896],
    };
    if ((*code).size - 17i32) % 4i32 != 0 {
        return QUIRC_ERROR_INVALID_GRID_SIZE;
    }
    memset(
        data as *mut libc::c_void,
        0,
        std::mem::size_of::<quirc_data>(),
    );
    memset(
        &mut ds as *mut datastream as *mut libc::c_void,
        0,
        std::mem::size_of::<datastream>(),
    );
    (*data).version = ((*code).size - 17i32) / 4i32;
    if (*data).version < 1i32 || (*data).version > 40i32 {
        return QUIRC_ERROR_INVALID_VERSION;
    }
    /* Read format information -- try both locations */
    let mut err = read_format(code, data, 0i32);
    if err as u64 != 0 {
        err = read_format(code, data, 1i32)
    }
    if err as u64 != 0 {
        return err;
    }
    read_data(code, data, &mut ds);
    err = codestream_ecc(data, &mut ds);
    if err as u64 != 0 {
        return err;
    }
    err = decode_payload(data, &mut ds);
    if err as u64 != 0 {
        return err;
    }
    return QUIRC_SUCCESS;
}
