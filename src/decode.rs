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
    0x1 as uint8_t,
    0x2 as uint8_t,
    0x4 as uint8_t,
    0x8 as uint8_t,
    0x3 as uint8_t,
    0x6 as uint8_t,
    0xc as uint8_t,
    0xb as uint8_t,
    0x5 as uint8_t,
    0xa as uint8_t,
    0x7 as uint8_t,
    0xe as uint8_t,
    0xf as uint8_t,
    0xd as uint8_t,
    0x9 as uint8_t,
    0x1 as uint8_t,
];
static mut gf16_log: [uint8_t; 16] = [
    0 as uint8_t,
    0xf as uint8_t,
    0x1 as uint8_t,
    0x4 as uint8_t,
    0x2 as uint8_t,
    0x8 as uint8_t,
    0x5 as uint8_t,
    0xa as uint8_t,
    0x3 as uint8_t,
    0xe as uint8_t,
    0x9 as uint8_t,
    0x7 as uint8_t,
    0x6 as uint8_t,
    0xd as uint8_t,
    0xb as uint8_t,
    0xc as uint8_t,
];
static mut gf16: galois_field = unsafe {
    galois_field {
        p: 15,
        log: gf16_log.as_ptr(),
        exp: gf16_exp.as_ptr(),
    }
};

static mut gf256_exp: [uint8_t; 256] = [
    0x1 as uint8_t,
    0x2 as uint8_t,
    0x4 as uint8_t,
    0x8 as uint8_t,
    0x10 as uint8_t,
    0x20 as uint8_t,
    0x40 as uint8_t,
    0x80 as uint8_t,
    0x1d as uint8_t,
    0x3a as uint8_t,
    0x74 as uint8_t,
    0xe8 as uint8_t,
    0xcd as uint8_t,
    0x87 as uint8_t,
    0x13 as uint8_t,
    0x26 as uint8_t,
    0x4c as uint8_t,
    0x98 as uint8_t,
    0x2d as uint8_t,
    0x5a as uint8_t,
    0xb4 as uint8_t,
    0x75 as uint8_t,
    0xea as uint8_t,
    0xc9 as uint8_t,
    0x8f as uint8_t,
    0x3 as uint8_t,
    0x6 as uint8_t,
    0xc as uint8_t,
    0x18 as uint8_t,
    0x30 as uint8_t,
    0x60 as uint8_t,
    0xc0 as uint8_t,
    0x9d as uint8_t,
    0x27 as uint8_t,
    0x4e as uint8_t,
    0x9c as uint8_t,
    0x25 as uint8_t,
    0x4a as uint8_t,
    0x94 as uint8_t,
    0x35 as uint8_t,
    0x6a as uint8_t,
    0xd4 as uint8_t,
    0xb5 as uint8_t,
    0x77 as uint8_t,
    0xee as uint8_t,
    0xc1 as uint8_t,
    0x9f as uint8_t,
    0x23 as uint8_t,
    0x46 as uint8_t,
    0x8c as uint8_t,
    0x5 as uint8_t,
    0xa as uint8_t,
    0x14 as uint8_t,
    0x28 as uint8_t,
    0x50 as uint8_t,
    0xa0 as uint8_t,
    0x5d as uint8_t,
    0xba as uint8_t,
    0x69 as uint8_t,
    0xd2 as uint8_t,
    0xb9 as uint8_t,
    0x6f as uint8_t,
    0xde as uint8_t,
    0xa1 as uint8_t,
    0x5f as uint8_t,
    0xbe as uint8_t,
    0x61 as uint8_t,
    0xc2 as uint8_t,
    0x99 as uint8_t,
    0x2f as uint8_t,
    0x5e as uint8_t,
    0xbc as uint8_t,
    0x65 as uint8_t,
    0xca as uint8_t,
    0x89 as uint8_t,
    0xf as uint8_t,
    0x1e as uint8_t,
    0x3c as uint8_t,
    0x78 as uint8_t,
    0xf0 as uint8_t,
    0xfd as uint8_t,
    0xe7 as uint8_t,
    0xd3 as uint8_t,
    0xbb as uint8_t,
    0x6b as uint8_t,
    0xd6 as uint8_t,
    0xb1 as uint8_t,
    0x7f as uint8_t,
    0xfe as uint8_t,
    0xe1 as uint8_t,
    0xdf as uint8_t,
    0xa3 as uint8_t,
    0x5b as uint8_t,
    0xb6 as uint8_t,
    0x71 as uint8_t,
    0xe2 as uint8_t,
    0xd9 as uint8_t,
    0xaf as uint8_t,
    0x43 as uint8_t,
    0x86 as uint8_t,
    0x11 as uint8_t,
    0x22 as uint8_t,
    0x44 as uint8_t,
    0x88 as uint8_t,
    0xd as uint8_t,
    0x1a as uint8_t,
    0x34 as uint8_t,
    0x68 as uint8_t,
    0xd0 as uint8_t,
    0xbd as uint8_t,
    0x67 as uint8_t,
    0xce as uint8_t,
    0x81 as uint8_t,
    0x1f as uint8_t,
    0x3e as uint8_t,
    0x7c as uint8_t,
    0xf8 as uint8_t,
    0xed as uint8_t,
    0xc7 as uint8_t,
    0x93 as uint8_t,
    0x3b as uint8_t,
    0x76 as uint8_t,
    0xec as uint8_t,
    0xc5 as uint8_t,
    0x97 as uint8_t,
    0x33 as uint8_t,
    0x66 as uint8_t,
    0xcc as uint8_t,
    0x85 as uint8_t,
    0x17 as uint8_t,
    0x2e as uint8_t,
    0x5c as uint8_t,
    0xb8 as uint8_t,
    0x6d as uint8_t,
    0xda as uint8_t,
    0xa9 as uint8_t,
    0x4f as uint8_t,
    0x9e as uint8_t,
    0x21 as uint8_t,
    0x42 as uint8_t,
    0x84 as uint8_t,
    0x15 as uint8_t,
    0x2a as uint8_t,
    0x54 as uint8_t,
    0xa8 as uint8_t,
    0x4d as uint8_t,
    0x9a as uint8_t,
    0x29 as uint8_t,
    0x52 as uint8_t,
    0xa4 as uint8_t,
    0x55 as uint8_t,
    0xaa as uint8_t,
    0x49 as uint8_t,
    0x92 as uint8_t,
    0x39 as uint8_t,
    0x72 as uint8_t,
    0xe4 as uint8_t,
    0xd5 as uint8_t,
    0xb7 as uint8_t,
    0x73 as uint8_t,
    0xe6 as uint8_t,
    0xd1 as uint8_t,
    0xbf as uint8_t,
    0x63 as uint8_t,
    0xc6 as uint8_t,
    0x91 as uint8_t,
    0x3f as uint8_t,
    0x7e as uint8_t,
    0xfc as uint8_t,
    0xe5 as uint8_t,
    0xd7 as uint8_t,
    0xb3 as uint8_t,
    0x7b as uint8_t,
    0xf6 as uint8_t,
    0xf1 as uint8_t,
    0xff as uint8_t,
    0xe3 as uint8_t,
    0xdb as uint8_t,
    0xab as uint8_t,
    0x4b as uint8_t,
    0x96 as uint8_t,
    0x31 as uint8_t,
    0x62 as uint8_t,
    0xc4 as uint8_t,
    0x95 as uint8_t,
    0x37 as uint8_t,
    0x6e as uint8_t,
    0xdc as uint8_t,
    0xa5 as uint8_t,
    0x57 as uint8_t,
    0xae as uint8_t,
    0x41 as uint8_t,
    0x82 as uint8_t,
    0x19 as uint8_t,
    0x32 as uint8_t,
    0x64 as uint8_t,
    0xc8 as uint8_t,
    0x8d as uint8_t,
    0x7 as uint8_t,
    0xe as uint8_t,
    0x1c as uint8_t,
    0x38 as uint8_t,
    0x70 as uint8_t,
    0xe0 as uint8_t,
    0xdd as uint8_t,
    0xa7 as uint8_t,
    0x53 as uint8_t,
    0xa6 as uint8_t,
    0x51 as uint8_t,
    0xa2 as uint8_t,
    0x59 as uint8_t,
    0xb2 as uint8_t,
    0x79 as uint8_t,
    0xf2 as uint8_t,
    0xf9 as uint8_t,
    0xef as uint8_t,
    0xc3 as uint8_t,
    0x9b as uint8_t,
    0x2b as uint8_t,
    0x56 as uint8_t,
    0xac as uint8_t,
    0x45 as uint8_t,
    0x8a as uint8_t,
    0x9 as uint8_t,
    0x12 as uint8_t,
    0x24 as uint8_t,
    0x48 as uint8_t,
    0x90 as uint8_t,
    0x3d as uint8_t,
    0x7a as uint8_t,
    0xf4 as uint8_t,
    0xf5 as uint8_t,
    0xf7 as uint8_t,
    0xf3 as uint8_t,
    0xfb as uint8_t,
    0xeb as uint8_t,
    0xcb as uint8_t,
    0x8b as uint8_t,
    0xb as uint8_t,
    0x16 as uint8_t,
    0x2c as uint8_t,
    0x58 as uint8_t,
    0xb0 as uint8_t,
    0x7d as uint8_t,
    0xfa as uint8_t,
    0xe9 as uint8_t,
    0xcf as uint8_t,
    0x83 as uint8_t,
    0x1b as uint8_t,
    0x36 as uint8_t,
    0x6c as uint8_t,
    0xd8 as uint8_t,
    0xad as uint8_t,
    0x47 as uint8_t,
    0x8e as uint8_t,
    0x1 as uint8_t,
];
static mut gf256_log: [uint8_t; 256] = [
    0 as uint8_t,
    0xff as uint8_t,
    0x1 as uint8_t,
    0x19 as uint8_t,
    0x2 as uint8_t,
    0x32 as uint8_t,
    0x1a as uint8_t,
    0xc6 as uint8_t,
    0x3 as uint8_t,
    0xdf as uint8_t,
    0x33 as uint8_t,
    0xee as uint8_t,
    0x1b as uint8_t,
    0x68 as uint8_t,
    0xc7 as uint8_t,
    0x4b as uint8_t,
    0x4 as uint8_t,
    0x64 as uint8_t,
    0xe0 as uint8_t,
    0xe as uint8_t,
    0x34 as uint8_t,
    0x8d as uint8_t,
    0xef as uint8_t,
    0x81 as uint8_t,
    0x1c as uint8_t,
    0xc1 as uint8_t,
    0x69 as uint8_t,
    0xf8 as uint8_t,
    0xc8 as uint8_t,
    0x8 as uint8_t,
    0x4c as uint8_t,
    0x71 as uint8_t,
    0x5 as uint8_t,
    0x8a as uint8_t,
    0x65 as uint8_t,
    0x2f as uint8_t,
    0xe1 as uint8_t,
    0x24 as uint8_t,
    0xf as uint8_t,
    0x21 as uint8_t,
    0x35 as uint8_t,
    0x93 as uint8_t,
    0x8e as uint8_t,
    0xda as uint8_t,
    0xf0 as uint8_t,
    0x12 as uint8_t,
    0x82 as uint8_t,
    0x45 as uint8_t,
    0x1d as uint8_t,
    0xb5 as uint8_t,
    0xc2 as uint8_t,
    0x7d as uint8_t,
    0x6a as uint8_t,
    0x27 as uint8_t,
    0xf9 as uint8_t,
    0xb9 as uint8_t,
    0xc9 as uint8_t,
    0x9a as uint8_t,
    0x9 as uint8_t,
    0x78 as uint8_t,
    0x4d as uint8_t,
    0xe4 as uint8_t,
    0x72 as uint8_t,
    0xa6 as uint8_t,
    0x6 as uint8_t,
    0xbf as uint8_t,
    0x8b as uint8_t,
    0x62 as uint8_t,
    0x66 as uint8_t,
    0xdd as uint8_t,
    0x30 as uint8_t,
    0xfd as uint8_t,
    0xe2 as uint8_t,
    0x98 as uint8_t,
    0x25 as uint8_t,
    0xb3 as uint8_t,
    0x10 as uint8_t,
    0x91 as uint8_t,
    0x22 as uint8_t,
    0x88 as uint8_t,
    0x36 as uint8_t,
    0xd0 as uint8_t,
    0x94 as uint8_t,
    0xce as uint8_t,
    0x8f as uint8_t,
    0x96 as uint8_t,
    0xdb as uint8_t,
    0xbd as uint8_t,
    0xf1 as uint8_t,
    0xd2 as uint8_t,
    0x13 as uint8_t,
    0x5c as uint8_t,
    0x83 as uint8_t,
    0x38 as uint8_t,
    0x46 as uint8_t,
    0x40 as uint8_t,
    0x1e as uint8_t,
    0x42 as uint8_t,
    0xb6 as uint8_t,
    0xa3 as uint8_t,
    0xc3 as uint8_t,
    0x48 as uint8_t,
    0x7e as uint8_t,
    0x6e as uint8_t,
    0x6b as uint8_t,
    0x3a as uint8_t,
    0x28 as uint8_t,
    0x54 as uint8_t,
    0xfa as uint8_t,
    0x85 as uint8_t,
    0xba as uint8_t,
    0x3d as uint8_t,
    0xca as uint8_t,
    0x5e as uint8_t,
    0x9b as uint8_t,
    0x9f as uint8_t,
    0xa as uint8_t,
    0x15 as uint8_t,
    0x79 as uint8_t,
    0x2b as uint8_t,
    0x4e as uint8_t,
    0xd4 as uint8_t,
    0xe5 as uint8_t,
    0xac as uint8_t,
    0x73 as uint8_t,
    0xf3 as uint8_t,
    0xa7 as uint8_t,
    0x57 as uint8_t,
    0x7 as uint8_t,
    0x70 as uint8_t,
    0xc0 as uint8_t,
    0xf7 as uint8_t,
    0x8c as uint8_t,
    0x80 as uint8_t,
    0x63 as uint8_t,
    0xd as uint8_t,
    0x67 as uint8_t,
    0x4a as uint8_t,
    0xde as uint8_t,
    0xed as uint8_t,
    0x31 as uint8_t,
    0xc5 as uint8_t,
    0xfe as uint8_t,
    0x18 as uint8_t,
    0xe3 as uint8_t,
    0xa5 as uint8_t,
    0x99 as uint8_t,
    0x77 as uint8_t,
    0x26 as uint8_t,
    0xb8 as uint8_t,
    0xb4 as uint8_t,
    0x7c as uint8_t,
    0x11 as uint8_t,
    0x44 as uint8_t,
    0x92 as uint8_t,
    0xd9 as uint8_t,
    0x23 as uint8_t,
    0x20 as uint8_t,
    0x89 as uint8_t,
    0x2e as uint8_t,
    0x37 as uint8_t,
    0x3f as uint8_t,
    0xd1 as uint8_t,
    0x5b as uint8_t,
    0x95 as uint8_t,
    0xbc as uint8_t,
    0xcf as uint8_t,
    0xcd as uint8_t,
    0x90 as uint8_t,
    0x87 as uint8_t,
    0x97 as uint8_t,
    0xb2 as uint8_t,
    0xdc as uint8_t,
    0xfc as uint8_t,
    0xbe as uint8_t,
    0x61 as uint8_t,
    0xf2 as uint8_t,
    0x56 as uint8_t,
    0xd3 as uint8_t,
    0xab as uint8_t,
    0x14 as uint8_t,
    0x2a as uint8_t,
    0x5d as uint8_t,
    0x9e as uint8_t,
    0x84 as uint8_t,
    0x3c as uint8_t,
    0x39 as uint8_t,
    0x53 as uint8_t,
    0x47 as uint8_t,
    0x6d as uint8_t,
    0x41 as uint8_t,
    0xa2 as uint8_t,
    0x1f as uint8_t,
    0x2d as uint8_t,
    0x43 as uint8_t,
    0xd8 as uint8_t,
    0xb7 as uint8_t,
    0x7b as uint8_t,
    0xa4 as uint8_t,
    0x76 as uint8_t,
    0xc4 as uint8_t,
    0x17 as uint8_t,
    0x49 as uint8_t,
    0xec as uint8_t,
    0x7f as uint8_t,
    0xc as uint8_t,
    0x6f as uint8_t,
    0xf6 as uint8_t,
    0x6c as uint8_t,
    0xa1 as uint8_t,
    0x3b as uint8_t,
    0x52 as uint8_t,
    0x29 as uint8_t,
    0x9d as uint8_t,
    0x55 as uint8_t,
    0xaa as uint8_t,
    0xfb as uint8_t,
    0x60 as uint8_t,
    0x86 as uint8_t,
    0xb1 as uint8_t,
    0xbb as uint8_t,
    0xcc as uint8_t,
    0x3e as uint8_t,
    0x5a as uint8_t,
    0xcb as uint8_t,
    0x59 as uint8_t,
    0x5f as uint8_t,
    0xb0 as uint8_t,
    0x9c as uint8_t,
    0xa9 as uint8_t,
    0xa0 as uint8_t,
    0x51 as uint8_t,
    0xb as uint8_t,
    0xf5 as uint8_t,
    0x16 as uint8_t,
    0xeb as uint8_t,
    0x7a as uint8_t,
    0x75 as uint8_t,
    0x2c as uint8_t,
    0xd7 as uint8_t,
    0x4f as uint8_t,
    0xae as uint8_t,
    0xd5 as uint8_t,
    0xe9 as uint8_t,
    0xe6 as uint8_t,
    0xe7 as uint8_t,
    0xad as uint8_t,
    0xe8 as uint8_t,
    0x74 as uint8_t,
    0xd6 as uint8_t,
    0xf4 as uint8_t,
    0xea as uint8_t,
    0xa8 as uint8_t,
    0x50 as uint8_t,
    0x58 as uint8_t,
    0xaf as uint8_t,
];
static mut gf256: galois_field = unsafe {
    galois_field {
        p: 255,
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
    let mut i = 0;
    while i < 64 {
        let p: libc::c_int = i + shift;
        let v: uint8_t = *src.offset(i as isize);
        if !(p < 0 || p >= 64) {
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
    let mut sum: uint8_t = 0 as uint8_t;
    let log_x: uint8_t = *(*gf).log.offset(x as isize);
    if x == 0 {
        return *s.offset(0);
    }
    let mut i = 0;
    while i < 64 {
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
    let mut L: libc::c_int = 0;
    let mut m: libc::c_int = 1;
    let mut b: uint8_t = 1 as uint8_t;

    memset(
        B.as_mut_ptr() as *mut libc::c_void,
        0,
        std::mem::size_of::<[uint8_t; 64]>(),
    );
    memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0,
        std::mem::size_of::<[uint8_t; 64]>(),
    );
    B[0] = 1 as uint8_t;
    C[0] = 1 as uint8_t;
    let mut n = 0;
    while n < N {
        let mut d: uint8_t = *s.offset(n as isize);
        let mult: uint8_t;
        let mut i = 1;
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
        } else if L * 2 <= n {
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
            L = n + 1 - L;
            b = d;
            m = 1
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
    let mut nonzero: libc::c_int = 0;
    memset(s as *mut libc::c_void, 0, 64);
    let mut i = 0;
    while i < npar {
        let mut j = 0;
        while j < bs {
            let c: uint8_t = *data.offset((bs - j - 1) as isize);
            if !(c == 0) {
                let ref mut fresh1 = *s.offset(i as isize);
                *fresh1 = (*fresh1 as libc::c_int
                    ^ gf256_exp[((gf256_log[c as usize] as libc::c_int + i * j) % 255) as usize]
                        as libc::c_int) as uint8_t
            }
            j += 1
        }
        if *s.offset(i as isize) != 0 {
            nonzero = 1
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
    memset(omega as *mut libc::c_void, 0, 64);
    let mut i = 0;
    while i < npar {
        let a: uint8_t = *sigma.offset(i as isize);
        let log_a: uint8_t = gf256_log[a as usize];
        if !(a == 0) {
            let mut j = 0;
            while j + 1 < 64 {
                let b: uint8_t = *s.offset((j + 1) as isize);
                if i + j >= npar {
                    break;
                }
                if !(b == 0) {
                    let ref mut fresh2 = *omega.offset((i + j) as isize);
                    *fresh2 = (*fresh2 as libc::c_int
                        ^ gf256_exp[((log_a as libc::c_int + gf256_log[b as usize] as libc::c_int)
                            % 255) as usize] as libc::c_int)
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
    let mut i = 0;
    while i + 1 < 64 {
        sigma_deriv[i as usize] = sigma[(i + 1) as usize];
        i += 2
    }
    /* Compute error evaluator polynomial */
    eloc_poly(
        omega.as_mut_ptr(),
        s.as_mut_ptr(),
        sigma.as_mut_ptr(),
        npar - 1,
    );
    /* Find error locations and magnitudes */
    i = 0;
    while i < (*ecc).bs {
        let xinv: uint8_t = gf256_exp[(255 - i) as usize];
        if poly_eval(sigma.as_mut_ptr(), xinv, &gf256) == 0 {
            let sd_x: uint8_t = poly_eval(sigma_deriv.as_mut_ptr(), xinv, &gf256);
            let omega_x: uint8_t = poly_eval(omega.as_mut_ptr(), xinv, &gf256);
            let error: uint8_t = gf256_exp[((255 - gf256_log[sd_x as usize] as libc::c_int
                + gf256_log[omega_x as usize] as libc::c_int)
                % 255) as usize];
            let ref mut fresh3 = *data.offset(((*ecc).bs - i - 1) as isize);
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
    let mut nonzero: libc::c_int = 0;
    memset(s as *mut libc::c_void, 0, 64);
    let mut i = 0;
    while i < 3 * 2 {
        *s.offset(i as isize) = 0 as uint8_t;
        let mut j = 0;
        while j < 15 {
            if u as libc::c_int & 1 << j != 0 {
                let ref mut fresh4 = *s.offset(i as isize);
                *fresh4 = (*fresh4 as libc::c_int
                    ^ gf16_exp[((i + 1) * j % 15) as usize] as libc::c_int)
                    as uint8_t
            }
            j += 1
        }
        if *s.offset(i as isize) != 0 {
            nonzero = 1
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
    berlekamp_massey(s.as_mut_ptr(), 3 * 2, &gf16, sigma.as_mut_ptr());
    /* Now, find the roots of the polynomial */
    let mut i = 0;
    while i < 15 {
        if poly_eval(sigma.as_mut_ptr(), gf16_exp[(15 - i) as usize], &gf16) == 0 {
            u = (u as libc::c_int ^ 1 << i) as uint16_t
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
    (*code).cell_bitmap[(p >> 3) as usize] as libc::c_int >> (p & 7) & 1
}

unsafe fn read_format(
    code: *const quirc_code,
    mut data: *mut quirc_data,
    which: libc::c_int,
) -> quirc_decode_error_t {
    let mut format: uint16_t = 0 as uint16_t;
    if which != 0 {
        let mut i = 0;
        while i < 7 {
            format = ((format as libc::c_int) << 1 | grid_bit(code, 8, (*code).size - 1 - i))
                as uint16_t;
            i += 1
        }
        i = 0;
        while i < 8 {
            format = ((format as libc::c_int) << 1 | grid_bit(code, (*code).size - 8 + i, 8))
                as uint16_t;
            i += 1
        }
    } else {
        static mut xs: [libc::c_int; 15] = [8, 8, 8, 8, 8, 8, 8, 8, 7, 5, 4, 3, 2, 1, 0];
        static mut ys: [libc::c_int; 15] = [0, 1, 2, 3, 4, 5, 7, 8, 8, 8, 8, 8, 8, 8, 8];
        let mut i = 14;
        while i >= 0 {
            format = ((format as libc::c_int) << 1 | grid_bit(code, xs[i as usize], ys[i as usize]))
                as uint16_t;
            i -= 1
        }
    }
    format = (format as libc::c_int ^ 0x5412) as uint16_t;
    let err = correct_format(&mut format);
    if err as u64 != 0 {
        return err;
    }
    let fdata = (format as libc::c_int >> 10) as uint16_t;
    (*data).ecc_level = fdata as libc::c_int >> 3;
    (*data).mask = fdata as libc::c_int & 7;
    return QUIRC_SUCCESS;
}
unsafe fn mask_bit(mask: libc::c_int, i: libc::c_int, j: libc::c_int) -> libc::c_int {
    match mask {
        0 => return ((i + j) % 2 == 0) as libc::c_int,
        1 => return (i % 2 == 0) as libc::c_int,
        2 => return (j % 3 == 0) as libc::c_int,
        3 => return ((i + j) % 3 == 0) as libc::c_int,
        4 => return ((i / 2 + j / 3) % 2 == 0) as libc::c_int,
        5 => return (i * j % 2 + i * j % 3 == 0) as libc::c_int,
        6 => return ((i * j % 2 + i * j % 3) % 2 == 0) as libc::c_int,
        7 => return ((i * j % 3 + (i + j) % 2) % 2 == 0) as libc::c_int,
        _ => {}
    }
    return 0;
}
unsafe fn reserved_cell(version: libc::c_int, i: libc::c_int, j: libc::c_int) -> libc::c_int {
    let ver: *const quirc_version_info =
        &*quirc_version_db.as_ptr().offset(version as isize) as *const quirc_version_info;
    let size: libc::c_int = version * 4 + 17;
    let mut ai: libc::c_int = -1;
    let mut aj: libc::c_int = -1;
    /* Finder + format: top left */
    if i < 9 && j < 9 {
        return 1;
    }
    /* Finder + format: bottom left */
    if i + 8 >= size && j < 9 {
        return 1;
    }
    /* Finder + format: top right */
    if i < 9 && j + 8 >= size {
        return 1;
    }
    /* Exclude timing patterns */
    if i == 6 || j == 6 {
        return 1;
    }
    /* Exclude version info, if it exists. Version info sits adjacent to
     * the top-right and bottom-left finders in three rows, bounded by
     * the timing pattern.
     */
    if version >= 7 {
        if i < 6 && j + 11 >= size {
            return 1;
        }
        if i + 11 >= size && j < 6 {
            return 1;
        }
    }
    /* Exclude alignment patterns */
    let mut a = 0;
    while a < 7 && (*ver).apat[a as usize] != 0 {
        let p: libc::c_int = (*ver).apat[a as usize];
        if abs(p - i) < 3 {
            ai = a
        }
        if abs(p - j) < 3 {
            aj = a
        }
        a += 1
    }
    if ai >= 0 && aj >= 0 {
        a -= 1;
        if ai > 0 && ai < a {
            return 1;
        }
        if aj > 0 && aj < a {
            return 1;
        }
        if aj == a && ai == a {
            return 1;
        }
    }
    return 0;
}
unsafe fn read_bit(
    code: *const quirc_code,
    data: *mut quirc_data,
    mut ds: *mut datastream,
    i: libc::c_int,
    j: libc::c_int,
) {
    let bitpos: libc::c_int = (*ds).data_bits & 7;
    let bytepos: libc::c_int = (*ds).data_bits >> 3;
    let mut v: libc::c_int = grid_bit(code, j, i);
    if mask_bit((*data).mask, i, j) != 0 {
        v ^= 1
    }
    if v != 0 {
        (*ds).raw[bytepos as usize] =
            ((*ds).raw[bytepos as usize] as libc::c_int | 0x80 >> bitpos) as uint8_t
    }
    (*ds).data_bits += 1;
}
unsafe fn read_data(code: *const quirc_code, data: *mut quirc_data, ds: *mut datastream) {
    let mut y: libc::c_int = (*code).size - 1;
    let mut x: libc::c_int = (*code).size - 1;
    let mut dir: libc::c_int = -1;
    while x > 0 {
        if x == 6 {
            x -= 1
        }
        if reserved_cell((*data).version, y, x) == 0 {
            read_bit(code, data, ds, y, x);
        }
        if reserved_cell((*data).version, y, x - 1) == 0 {
            read_bit(code, data, ds, y, x - 1);
        }
        y += dir;
        if y < 0 || y >= (*code).size {
            dir = -dir;
            x -= 2;
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
        ((*ver).data_bytes - (*sb_ecc).bs * (*sb_ecc).ns) / ((*sb_ecc).bs + 1);
    let bc: libc::c_int = lb_count + (*sb_ecc).ns;
    let ecc_offset: libc::c_int = (*sb_ecc).dw * bc + lb_count;
    let mut dst_offset: libc::c_int = 0;
    memcpy(
        &mut lb_ecc as *mut quirc_rs_params as *mut libc::c_void,
        sb_ecc as *const libc::c_void,
        std::mem::size_of::<quirc_rs_params>(),
    );
    lb_ecc.dw += 1;
    lb_ecc.bs += 1;
    let mut i = 0;
    while i < bc {
        let dst: *mut uint8_t = (*ds).data.as_mut_ptr().offset(dst_offset as isize);
        let ecc: *const quirc_rs_params = if i < (*sb_ecc).ns {
            sb_ecc
        } else {
            &mut lb_ecc as *mut quirc_rs_params as *const quirc_rs_params
        };
        let num_ec: libc::c_int = (*ecc).bs - (*ecc).dw;
        let mut j = 0;
        while j < (*ecc).dw {
            *dst.offset(j as isize) = (*ds).raw[(j * bc + i) as usize];
            j += 1
        }
        j = 0;
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
    (*ds).data_bits = dst_offset * 8;
    return QUIRC_SUCCESS;
}
#[inline]
unsafe fn bits_remaining(ds: *const datastream) -> libc::c_int {
    return (*ds).data_bits - (*ds).ptr;
}
unsafe fn take_bits(mut ds: *mut datastream, mut len: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    while len != 0 && (*ds).ptr < (*ds).data_bits {
        let b: uint8_t = (*ds).data[((*ds).ptr >> 3) as usize];
        let bitpos: libc::c_int = (*ds).ptr & 7;
        ret <<= 1;
        if (b as libc::c_int) << bitpos & 0x80 != 0 {
            ret |= 1
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
        return -1;
    }
    let mut tuple = take_bits(ds, bits);
    let mut i = digits - 1;
    while i >= 0 {
        (*data).payload[((*data).payload_len + i) as usize] = (tuple % 10 + '0' as i32) as uint8_t;
        tuple /= 10;
        i -= 1
    }
    (*data).payload_len += digits;
    return 0;
}
unsafe fn decode_numeric(data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: libc::c_int = 14;
    if (*data).version < 10 {
        bits = 10
    } else if (*data).version < 27 {
        bits = 12
    }
    let mut count = take_bits(ds, bits);
    if (*data).payload_len + count + 1 > 8896 {
        return QUIRC_ERROR_DATA_OVERFLOW;
    }
    while count >= 3 {
        if numeric_tuple(data, ds, 10, 3) < 0 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 3
    }
    if count >= 2 {
        if numeric_tuple(data, ds, 7, 2) < 0 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 2
    }
    if count != 0 {
        if numeric_tuple(data, ds, 4, 1) < 0 {
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
        return -1;
    }
    let mut tuple = take_bits(ds, bits);
    let mut i = 0;
    while i < digits {
        static mut alpha_map: *const libc::c_char =
            b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:\x00" as *const u8
                as *const libc::c_char;
        (*data).payload[((*data).payload_len + digits - i - 1) as usize] =
            *alpha_map.offset((tuple % 45) as isize) as uint8_t;
        tuple /= 45;
        i += 1
    }
    (*data).payload_len += digits;
    return 0;
}
unsafe fn decode_alpha(data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: libc::c_int = 13;
    if (*data).version < 10 {
        bits = 9
    } else if (*data).version < 27 {
        bits = 11
    }
    let mut count = take_bits(ds, bits);
    if (*data).payload_len + count + 1 > 8896 {
        return QUIRC_ERROR_DATA_OVERFLOW;
    }
    while count >= 2 {
        if alpha_tuple(data, ds, 11, 2) < 0 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 2
    }
    if count != 0 {
        if alpha_tuple(data, ds, 6, 1) < 0 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        count -= 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_byte(mut data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: libc::c_int = 16;
    if (*data).version < 10 {
        bits = 8
    }
    let count = take_bits(ds, bits);
    if (*data).payload_len + count + 1 > 8896 {
        return QUIRC_ERROR_DATA_OVERFLOW;
    }
    if bits_remaining(ds) < count * 8 {
        return QUIRC_ERROR_DATA_UNDERFLOW;
    }
    let mut i = 0;
    while i < count {
        let fresh5 = (*data).payload_len;
        (*data).payload_len = (*data).payload_len + 1;
        (*data).payload[fresh5 as usize] = take_bits(ds, 8) as uint8_t;
        i += 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_kanji(mut data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: libc::c_int = 12;
    if (*data).version < 10 {
        bits = 8
    } else if (*data).version < 27 {
        bits = 10
    }
    let count = take_bits(ds, bits);
    if (*data).payload_len + count * 2 + 1 > 8896 {
        return QUIRC_ERROR_DATA_OVERFLOW;
    }
    if bits_remaining(ds) < count * 13 {
        return QUIRC_ERROR_DATA_UNDERFLOW;
    }
    let mut i = 0;
    while i < count {
        let d: libc::c_int = take_bits(ds, 13);
        let msB: libc::c_int = d / 0xc0;
        let lsB: libc::c_int = d % 0xc0;
        let intermediate: libc::c_int = msB << 8 | lsB;
        let sjw: uint16_t;
        if intermediate + 0x8140 <= 0x9ffc {
            /* bytes are in the range 0x8140 to 0x9FFC */
            sjw = (intermediate + 0x8140) as uint16_t
        } else {
            /* bytes are in the range 0xE040 to 0xEBBF */
            sjw = (intermediate + 0xc140) as uint16_t
        }
        let fresh6 = (*data).payload_len;
        (*data).payload_len = (*data).payload_len + 1;
        (*data).payload[fresh6 as usize] = (sjw as libc::c_int >> 8) as uint8_t;
        let fresh7 = (*data).payload_len;
        (*data).payload_len = (*data).payload_len + 1;
        (*data).payload[fresh7 as usize] = (sjw as libc::c_int & 0xff) as uint8_t;
        i += 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_eci(mut data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    if bits_remaining(ds) < 8 {
        return QUIRC_ERROR_DATA_UNDERFLOW;
    }
    (*data).eci = take_bits(ds, 8) as uint32_t;
    if (*data).eci & 0xc0 as libc::c_uint == 0x80 as libc::c_uint {
        if bits_remaining(ds) < 8 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        (*data).eci = (*data).eci << 8 | take_bits(ds, 8) as libc::c_uint
    } else if (*data).eci & 0xe0 as libc::c_uint == 0xc0 as libc::c_uint {
        if bits_remaining(ds) < 16 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        (*data).eci = (*data).eci << 16 | take_bits(ds, 16) as libc::c_uint
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_payload(mut data: *mut quirc_data, ds: *mut datastream) -> quirc_decode_error_t {
    while bits_remaining(ds) >= 4 {
        let type_0 = take_bits(ds, 4);
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
        if type_0 & type_0 - 1 == 0 && type_0 > (*data).data_type {
            (*data).data_type = type_0
        }
    }
    /* Add nul terminator to all payloads */
    if (*data).payload_len
        >= ::std::mem::size_of::<[uint8_t; 8896]>() as libc::c_ulong as libc::c_int
    {
        (*data).payload_len -= 1
    }
    (*data).payload[(*data).payload_len as usize] = 0 as uint8_t;
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
    if ((*code).size - 17) % 4 != 0 {
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
    (*data).version = ((*code).size - 17) / 4;
    if (*data).version < 1 || (*data).version > 40 {
        return QUIRC_ERROR_INVALID_VERSION;
    }
    /* Read format information -- try both locations */
    let mut err = read_format(code, data, 0);
    if err as u64 != 0 {
        err = read_format(code, data, 1)
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
