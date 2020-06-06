use libc::{abs, memcpy, memset};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::quirc::*;
use crate::version_db::*;

/* ***********************************************************************
 * Decoder algorithm
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct datastream {
    pub raw: [u8; 8896],
    pub data_bits: i32,
    pub ptr: i32,
    pub data: [u8; 8896],
}

/* ***********************************************************************
 * Galois fields
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct galois_field {
    pub p: i32,
    pub log: *const u8,
    pub exp: *const u8,
}
static mut gf16_exp: [u8; 16] = [
    0x1, 0x2, 0x4, 0x8, 0x3, 0x6, 0xc, 0xb, 0x5, 0xa, 0x7, 0xe, 0xf, 0xd, 0x9, 0x1,
];
static mut gf16_log: [u8; 16] = [
    0, 0xf, 0x1, 0x4, 0x2, 0x8, 0x5, 0xa, 0x3, 0xe, 0x9, 0x7, 0x6, 0xd, 0xb, 0xc,
];
static mut gf16: galois_field = unsafe {
    galois_field {
        p: 15,
        log: gf16_log.as_ptr(),
        exp: gf16_exp.as_ptr(),
    }
};

static mut gf256_exp: [u8; 256] = [
    0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80, 0x1d, 0x3a, 0x74, 0xe8, 0xcd, 0x87, 0x13, 0x26,
    0x4c, 0x98, 0x2d, 0x5a, 0xb4, 0x75, 0xea, 0xc9, 0x8f, 0x3, 0x6, 0xc, 0x18, 0x30, 0x60, 0xc0,
    0x9d, 0x27, 0x4e, 0x9c, 0x25, 0x4a, 0x94, 0x35, 0x6a, 0xd4, 0xb5, 0x77, 0xee, 0xc1, 0x9f, 0x23,
    0x46, 0x8c, 0x5, 0xa, 0x14, 0x28, 0x50, 0xa0, 0x5d, 0xba, 0x69, 0xd2, 0xb9, 0x6f, 0xde, 0xa1,
    0x5f, 0xbe, 0x61, 0xc2, 0x99, 0x2f, 0x5e, 0xbc, 0x65, 0xca, 0x89, 0xf, 0x1e, 0x3c, 0x78, 0xf0,
    0xfd, 0xe7, 0xd3, 0xbb, 0x6b, 0xd6, 0xb1, 0x7f, 0xfe, 0xe1, 0xdf, 0xa3, 0x5b, 0xb6, 0x71, 0xe2,
    0xd9, 0xaf, 0x43, 0x86, 0x11, 0x22, 0x44, 0x88, 0xd, 0x1a, 0x34, 0x68, 0xd0, 0xbd, 0x67, 0xce,
    0x81, 0x1f, 0x3e, 0x7c, 0xf8, 0xed, 0xc7, 0x93, 0x3b, 0x76, 0xec, 0xc5, 0x97, 0x33, 0x66, 0xcc,
    0x85, 0x17, 0x2e, 0x5c, 0xb8, 0x6d, 0xda, 0xa9, 0x4f, 0x9e, 0x21, 0x42, 0x84, 0x15, 0x2a, 0x54,
    0xa8, 0x4d, 0x9a, 0x29, 0x52, 0xa4, 0x55, 0xaa, 0x49, 0x92, 0x39, 0x72, 0xe4, 0xd5, 0xb7, 0x73,
    0xe6, 0xd1, 0xbf, 0x63, 0xc6, 0x91, 0x3f, 0x7e, 0xfc, 0xe5, 0xd7, 0xb3, 0x7b, 0xf6, 0xf1, 0xff,
    0xe3, 0xdb, 0xab, 0x4b, 0x96, 0x31, 0x62, 0xc4, 0x95, 0x37, 0x6e, 0xdc, 0xa5, 0x57, 0xae, 0x41,
    0x82, 0x19, 0x32, 0x64, 0xc8, 0x8d, 0x7, 0xe, 0x1c, 0x38, 0x70, 0xe0, 0xdd, 0xa7, 0x53, 0xa6,
    0x51, 0xa2, 0x59, 0xb2, 0x79, 0xf2, 0xf9, 0xef, 0xc3, 0x9b, 0x2b, 0x56, 0xac, 0x45, 0x8a, 0x9,
    0x12, 0x24, 0x48, 0x90, 0x3d, 0x7a, 0xf4, 0xf5, 0xf7, 0xf3, 0xfb, 0xeb, 0xcb, 0x8b, 0xb, 0x16,
    0x2c, 0x58, 0xb0, 0x7d, 0xfa, 0xe9, 0xcf, 0x83, 0x1b, 0x36, 0x6c, 0xd8, 0xad, 0x47, 0x8e, 0x1,
];
static mut gf256_log: [u8; 256] = [
    0, 0xff, 0x1, 0x19, 0x2, 0x32, 0x1a, 0xc6, 0x3, 0xdf, 0x33, 0xee, 0x1b, 0x68, 0xc7, 0x4b, 0x4,
    0x64, 0xe0, 0xe, 0x34, 0x8d, 0xef, 0x81, 0x1c, 0xc1, 0x69, 0xf8, 0xc8, 0x8, 0x4c, 0x71, 0x5,
    0x8a, 0x65, 0x2f, 0xe1, 0x24, 0xf, 0x21, 0x35, 0x93, 0x8e, 0xda, 0xf0, 0x12, 0x82, 0x45, 0x1d,
    0xb5, 0xc2, 0x7d, 0x6a, 0x27, 0xf9, 0xb9, 0xc9, 0x9a, 0x9, 0x78, 0x4d, 0xe4, 0x72, 0xa6, 0x6,
    0xbf, 0x8b, 0x62, 0x66, 0xdd, 0x30, 0xfd, 0xe2, 0x98, 0x25, 0xb3, 0x10, 0x91, 0x22, 0x88, 0x36,
    0xd0, 0x94, 0xce, 0x8f, 0x96, 0xdb, 0xbd, 0xf1, 0xd2, 0x13, 0x5c, 0x83, 0x38, 0x46, 0x40, 0x1e,
    0x42, 0xb6, 0xa3, 0xc3, 0x48, 0x7e, 0x6e, 0x6b, 0x3a, 0x28, 0x54, 0xfa, 0x85, 0xba, 0x3d, 0xca,
    0x5e, 0x9b, 0x9f, 0xa, 0x15, 0x79, 0x2b, 0x4e, 0xd4, 0xe5, 0xac, 0x73, 0xf3, 0xa7, 0x57, 0x7,
    0x70, 0xc0, 0xf7, 0x8c, 0x80, 0x63, 0xd, 0x67, 0x4a, 0xde, 0xed, 0x31, 0xc5, 0xfe, 0x18, 0xe3,
    0xa5, 0x99, 0x77, 0x26, 0xb8, 0xb4, 0x7c, 0x11, 0x44, 0x92, 0xd9, 0x23, 0x20, 0x89, 0x2e, 0x37,
    0x3f, 0xd1, 0x5b, 0x95, 0xbc, 0xcf, 0xcd, 0x90, 0x87, 0x97, 0xb2, 0xdc, 0xfc, 0xbe, 0x61, 0xf2,
    0x56, 0xd3, 0xab, 0x14, 0x2a, 0x5d, 0x9e, 0x84, 0x3c, 0x39, 0x53, 0x47, 0x6d, 0x41, 0xa2, 0x1f,
    0x2d, 0x43, 0xd8, 0xb7, 0x7b, 0xa4, 0x76, 0xc4, 0x17, 0x49, 0xec, 0x7f, 0xc, 0x6f, 0xf6, 0x6c,
    0xa1, 0x3b, 0x52, 0x29, 0x9d, 0x55, 0xaa, 0xfb, 0x60, 0x86, 0xb1, 0xbb, 0xcc, 0x3e, 0x5a, 0xcb,
    0x59, 0x5f, 0xb0, 0x9c, 0xa9, 0xa0, 0x51, 0xb, 0xf5, 0x16, 0xeb, 0x7a, 0x75, 0x2c, 0xd7, 0x4f,
    0xae, 0xd5, 0xe9, 0xe6, 0xe7, 0xad, 0xe8, 0x74, 0xd6, 0xf4, 0xea, 0xa8, 0x50, 0x58, 0xaf,
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
unsafe fn poly_add(dst: *mut u8, src: *const u8, c: u8, shift: i32, gf: *const galois_field) {
    let log_c: i32 = *(*gf).log.offset(c as isize) as i32;
    if c == 0 {
        return;
    }
    let mut i = 0;
    while i < 64 {
        let p: i32 = i + shift;
        let v: u8 = *src.offset(i as isize);
        if !(p < 0 || p >= 64) {
            if !(v == 0) {
                let ref mut fresh0 = *dst.offset(p as isize);
                *fresh0 = (*fresh0 as i32
                    ^ *(*gf)
                        .exp
                        .offset(((*(*gf).log.offset(v as isize) as i32 + log_c) % (*gf).p) as isize)
                        as i32) as u8
            }
        }
        i += 1
    }
}
unsafe fn poly_eval(s: *const u8, x: u8, gf: *const galois_field) -> u8 {
    let mut sum: u8 = 0;
    let log_x: u8 = *(*gf).log.offset(x as isize);
    if x == 0 {
        return *s.offset(0);
    }
    let mut i = 0;
    while i < 64 {
        let c: u8 = *s.offset(i as isize);
        if !(c == 0) {
            sum = (sum as i32
                ^ *(*gf).exp.offset(
                    ((*(*gf).log.offset(c as isize) as i32 + log_x as i32 * i) % (*gf).p) as isize,
                ) as i32) as u8
        }
        i += 1
    }
    return sum;
}
/* ***********************************************************************
 * Berlekamp-Massey algorithm for finding error locator polynomials.
 */
unsafe fn berlekamp_massey(s: *const u8, N: i32, gf: *const galois_field, sigma: *mut u8) {
    let mut C: [u8; 64] = [0; 64];
    let mut B: [u8; 64] = [0; 64];
    let mut L: i32 = 0;
    let mut m: i32 = 1;
    let mut b: u8 = 1;

    memset(
        B.as_mut_ptr() as *mut libc::c_void,
        0,
        std::mem::size_of::<[u8; 64]>(),
    );
    memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0,
        std::mem::size_of::<[u8; 64]>(),
    );
    B[0] = 1;
    C[0] = 1;
    let mut n = 0;
    while n < N {
        let mut d: u8 = *s.offset(n as isize);
        let mult: u8;
        let mut i = 1;
        while i <= L {
            if C[i as usize] as i32 != 0 && *s.offset((n - i) as isize) as i32 != 0 {
                d = (d as i32
                    ^ *(*gf).exp.offset(
                        ((*(*gf).log.offset(C[i as usize] as isize) as i32
                            + *(*gf).log.offset(*s.offset((n - i) as isize) as isize) as i32)
                            % (*gf).p) as isize,
                    ) as i32) as u8
            }
            i += 1
        }
        mult = *(*gf).exp.offset(
            (((*gf).p - *(*gf).log.offset(b as isize) as i32
                + *(*gf).log.offset(d as isize) as i32)
                % (*gf).p) as isize,
        );
        if d == 0 {
            m += 1
        } else if L * 2 <= n {
            let mut T: [u8; 64] = [0; 64];
            memcpy(
                T.as_mut_ptr() as *mut libc::c_void,
                C.as_mut_ptr() as *const libc::c_void,
                std::mem::size_of::<[u8; 64]>(),
            );
            poly_add(C.as_mut_ptr(), B.as_mut_ptr(), mult, m, gf);
            memcpy(
                B.as_mut_ptr() as *mut libc::c_void,
                T.as_mut_ptr() as *const libc::c_void,
                std::mem::size_of::<[u8; 64]>(),
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
unsafe fn block_syndromes(data: *const u8, bs: i32, npar: i32, s: *mut u8) -> i32 {
    let mut nonzero: i32 = 0;
    memset(s as *mut libc::c_void, 0, 64);
    let mut i = 0;
    while i < npar {
        let mut j = 0;
        while j < bs {
            let c: u8 = *data.offset((bs - j - 1) as isize);
            if !(c == 0) {
                let ref mut fresh1 = *s.offset(i as isize);
                *fresh1 = (*fresh1 as i32
                    ^ gf256_exp[((gf256_log[c as usize] as i32 + i * j) % 255) as usize] as i32)
                    as u8
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
unsafe fn eloc_poly(omega: *mut u8, s: *const u8, sigma: *const u8, npar: i32) {
    memset(omega as *mut libc::c_void, 0, 64);
    let mut i = 0;
    while i < npar {
        let a: u8 = *sigma.offset(i as isize);
        let log_a: u8 = gf256_log[a as usize];
        if !(a == 0) {
            let mut j = 0;
            while j + 1 < 64 {
                let b: u8 = *s.offset((j + 1) as isize);
                if i + j >= npar {
                    break;
                }
                if !(b == 0) {
                    let ref mut fresh2 = *omega.offset((i + j) as isize);
                    *fresh2 = (*fresh2 as i32
                        ^ gf256_exp[((log_a as i32 + gf256_log[b as usize] as i32) % 255) as usize]
                            as i32) as u8
                }
                j += 1
            }
        }
        i += 1
    }
}
unsafe fn correct_block(data: *mut u8, ecc: *const quirc_rs_params) -> quirc_decode_error_t {
    let npar: i32 = (*ecc).bs - (*ecc).dw;
    let mut s: [u8; 64] = [0; 64];
    let mut sigma: [u8; 64] = [0; 64];
    let mut sigma_deriv: [u8; 64] = [0; 64];
    let mut omega: [u8; 64] = [0; 64];
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
        let xinv: u8 = gf256_exp[(255 - i) as usize];
        if poly_eval(sigma.as_mut_ptr(), xinv, &gf256) == 0 {
            let sd_x: u8 = poly_eval(sigma_deriv.as_mut_ptr(), xinv, &gf256);
            let omega_x: u8 = poly_eval(omega.as_mut_ptr(), xinv, &gf256);
            let error: u8 = gf256_exp[((255 - gf256_log[sd_x as usize] as i32
                + gf256_log[omega_x as usize] as i32)
                % 255) as usize];
            let ref mut fresh3 = *data.offset(((*ecc).bs - i - 1) as isize);
            *fresh3 = (*fresh3 as i32 ^ error as i32) as u8;
        }
        i += 1
    }
    if block_syndromes(data, (*ecc).bs, npar, s.as_mut_ptr()) != 0 {
        return QUIRC_ERROR_DATA_ECC;
    }
    return QUIRC_SUCCESS;
}
unsafe fn format_syndromes(u: u16, s: *mut u8) -> i32 {
    let mut nonzero: i32 = 0;
    memset(s as *mut libc::c_void, 0, 64);
    let mut i = 0;
    while i < 3 * 2 {
        *s.offset(i as isize) = 0;
        let mut j = 0;
        while j < 15 {
            if u as i32 & 1 << j != 0 {
                let ref mut fresh4 = *s.offset(i as isize);
                *fresh4 = (*fresh4 as i32 ^ gf16_exp[((i + 1) * j % 15) as usize] as i32) as u8;
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
unsafe fn correct_format(f_ret: *mut u16) -> quirc_decode_error_t {
    let mut u: u16 = *f_ret;
    let mut s: [u8; 64] = [0; 64];
    let mut sigma: [u8; 64] = [0; 64];
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
            u = (u as i32 ^ 1 << i) as u16
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
unsafe fn grid_bit(code: *const Code, x: i32, y: i32) -> i32 {
    let p: i32 = y * (*code).size + x;
    (*code).cell_bitmap[(p >> 3) as usize] as i32 >> (p & 7) & 1
}

unsafe fn read_format(code: *const Code, mut data: *mut Data, which: i32) -> quirc_decode_error_t {
    let mut format: u16 = 0 as u16;
    if which != 0 {
        let mut i = 0;
        while i < 7 {
            format = ((format as i32) << 1 | grid_bit(code, 8, (*code).size - 1 - i)) as u16;
            i += 1
        }
        i = 0;
        while i < 8 {
            format = ((format as i32) << 1 | grid_bit(code, (*code).size - 8 + i, 8)) as u16;
            i += 1
        }
    } else {
        static mut xs: [i32; 15] = [8, 8, 8, 8, 8, 8, 8, 8, 7, 5, 4, 3, 2, 1, 0];
        static mut ys: [i32; 15] = [0, 1, 2, 3, 4, 5, 7, 8, 8, 8, 8, 8, 8, 8, 8];
        let mut i = 14;
        while i >= 0 {
            format = ((format as i32) << 1 | grid_bit(code, xs[i as usize], ys[i as usize])) as u16;
            i -= 1
        }
    }
    format = (format as i32 ^ 0x5412) as u16;
    let err = correct_format(&mut format);
    if err as u64 != 0 {
        return err;
    }
    let fdata = (format as i32 >> 10) as u16;
    (*data).ecc_level = EccLevel::from_i32(fdata as i32 >> 3).unwrap();
    (*data).mask = fdata as i32 & 7;
    return QUIRC_SUCCESS;
}
unsafe fn mask_bit(mask: i32, i: i32, j: i32) -> i32 {
    match mask {
        0 => return ((i + j) % 2 == 0) as i32,
        1 => return (i % 2 == 0) as i32,
        2 => return (j % 3 == 0) as i32,
        3 => return ((i + j) % 3 == 0) as i32,
        4 => return ((i / 2 + j / 3) % 2 == 0) as i32,
        5 => return (i * j % 2 + i * j % 3 == 0) as i32,
        6 => return ((i * j % 2 + i * j % 3) % 2 == 0) as i32,
        7 => return ((i * j % 3 + (i + j) % 2) % 2 == 0) as i32,
        _ => {}
    }
    return 0;
}
unsafe fn reserved_cell(version: i32, i: i32, j: i32) -> i32 {
    let ver: *const quirc_version_info =
        &*quirc_version_db.as_ptr().offset(version as isize) as *const quirc_version_info;
    let size: i32 = version * 4 + 17;
    let mut ai: i32 = -1;
    let mut aj: i32 = -1;
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
        let p: i32 = (*ver).apat[a as usize];
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
unsafe fn read_bit(code: *const Code, data: *mut Data, mut ds: *mut datastream, i: i32, j: i32) {
    let bitpos: i32 = (*ds).data_bits & 7;
    let bytepos: i32 = (*ds).data_bits >> 3;
    let mut v: i32 = grid_bit(code, j, i);
    if mask_bit((*data).mask, i, j) != 0 {
        v ^= 1
    }
    if v != 0 {
        (*ds).raw[bytepos as usize] = ((*ds).raw[bytepos as usize] as i32 | 0x80 >> bitpos) as u8;
    }
    (*ds).data_bits += 1;
}
unsafe fn read_data(code: *const Code, data: *mut Data, ds: *mut datastream) {
    let mut y: i32 = (*code).size - 1;
    let mut x: i32 = (*code).size - 1;
    let mut dir: i32 = -1;
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
unsafe fn codestream_ecc(data: *mut Data, mut ds: *mut datastream) -> quirc_decode_error_t {
    let ver: *const quirc_version_info =
        &*quirc_version_db.as_ptr().offset((*data).version as isize) as *const quirc_version_info;
    let sb_ecc: *const quirc_rs_params =
        &*(*ver).ecc.as_ptr().offset((*data).ecc_level as isize) as *const quirc_rs_params;
    let mut lb_ecc: quirc_rs_params = quirc_rs_params {
        bs: 0,
        dw: 0,
        ns: 0,
    };
    let lb_count: i32 = ((*ver).data_bytes - (*sb_ecc).bs * (*sb_ecc).ns) / ((*sb_ecc).bs + 1);
    let bc: i32 = lb_count + (*sb_ecc).ns;
    let ecc_offset: i32 = (*sb_ecc).dw * bc + lb_count;
    let mut dst_offset: i32 = 0;
    memcpy(
        &mut lb_ecc as *mut quirc_rs_params as *mut libc::c_void,
        sb_ecc as *const libc::c_void,
        std::mem::size_of::<quirc_rs_params>(),
    );
    lb_ecc.dw += 1;
    lb_ecc.bs += 1;
    let mut i = 0;
    while i < bc {
        let dst: *mut u8 = (*ds).data.as_mut_ptr().offset(dst_offset as isize);
        let ecc: *const quirc_rs_params = if i < (*sb_ecc).ns {
            sb_ecc
        } else {
            &mut lb_ecc as *mut quirc_rs_params as *const quirc_rs_params
        };
        let num_ec: i32 = (*ecc).bs - (*ecc).dw;
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
unsafe fn bits_remaining(ds: *const datastream) -> i32 {
    return (*ds).data_bits - (*ds).ptr;
}
unsafe fn take_bits(mut ds: *mut datastream, mut len: i32) -> i32 {
    let mut ret: i32 = 0;
    while len != 0 && (*ds).ptr < (*ds).data_bits {
        let b: u8 = (*ds).data[((*ds).ptr >> 3) as usize];
        let bitpos: i32 = (*ds).ptr & 7;
        ret <<= 1;
        if (b as i32) << bitpos & 0x80 != 0 {
            ret |= 1
        }
        (*ds).ptr += 1;
        len -= 1
    }
    return ret;
}
unsafe fn numeric_tuple(mut data: *mut Data, ds: *mut datastream, bits: i32, digits: i32) -> i32 {
    if bits_remaining(ds) < bits {
        return -1;
    }
    let mut tuple = take_bits(ds, bits);
    let mut i = digits - 1;
    while i >= 0 {
        (*data).payload[((*data).payload_len + i) as usize] = (tuple % 10 + '0' as i32) as u8;
        tuple /= 10;
        i -= 1
    }
    (*data).payload_len += digits;
    return 0;
}
unsafe fn decode_numeric(data: *mut Data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: i32 = 14;
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
unsafe fn alpha_tuple(mut data: *mut Data, ds: *mut datastream, bits: i32, digits: i32) -> i32 {
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
            *alpha_map.offset((tuple % 45) as isize) as u8;
        tuple /= 45;
        i += 1
    }
    (*data).payload_len += digits;
    return 0;
}
unsafe fn decode_alpha(data: *mut Data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: i32 = 13;
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
unsafe fn decode_byte(mut data: *mut Data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: i32 = 16;
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
        (*data).payload[fresh5 as usize] = take_bits(ds, 8) as u8;
        i += 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_kanji(mut data: *mut Data, ds: *mut datastream) -> quirc_decode_error_t {
    let mut bits: i32 = 12;
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
        let d: i32 = take_bits(ds, 13);
        let msB: i32 = d / 0xc0;
        let lsB: i32 = d % 0xc0;
        let intermediate: i32 = msB << 8 | lsB;
        let sjw: u16;
        if intermediate + 0x8140 <= 0x9ffc {
            /* bytes are in the range 0x8140 to 0x9FFC */
            sjw = (intermediate + 0x8140) as u16
        } else {
            /* bytes are in the range 0xE040 to 0xEBBF */
            sjw = (intermediate + 0xc140) as u16
        }
        let fresh6 = (*data).payload_len;
        (*data).payload_len = (*data).payload_len + 1;
        (*data).payload[fresh6 as usize] = (sjw as i32 >> 8) as u8;
        let fresh7 = (*data).payload_len;
        (*data).payload_len = (*data).payload_len + 1;
        (*data).payload[fresh7 as usize] = (sjw as i32 & 0xff) as u8;
        i += 1
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_eci(mut data: *mut Data, ds: *mut datastream) -> quirc_decode_error_t {
    if bits_remaining(ds) < 8 {
        return QUIRC_ERROR_DATA_UNDERFLOW;
    }
    (*data).eci = Eci::from_u32(take_bits(ds, 8) as u32);
    if (*data).eci.and_then(|e| e.to_u32()).unwrap_or_default() & 0xc0 as libc::c_uint
        == 0x80 as libc::c_uint
    {
        if bits_remaining(ds) < 8 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        (*data).eci = Eci::from_u32(
            (*data).eci.and_then(|e| e.to_u32()).unwrap_or_default() << 8
                | take_bits(ds, 8) as libc::c_uint,
        );
    } else if (*data).eci.and_then(|e| e.to_u32()).unwrap_or_default() & 0xe0 as libc::c_uint
        == 0xc0 as libc::c_uint
    {
        if bits_remaining(ds) < 16 {
            return QUIRC_ERROR_DATA_UNDERFLOW;
        }
        (*data).eci = Eci::from_u32(
            (*data).eci.and_then(|e| e.to_u32()).unwrap_or_default() << 16
                | take_bits(ds, 16) as libc::c_uint,
        );
    }
    return QUIRC_SUCCESS;
}
unsafe fn decode_payload(mut data: *mut Data, ds: *mut datastream) -> quirc_decode_error_t {
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
    if (*data).payload_len >= ::std::mem::size_of::<[u8; 8896]>() as libc::c_ulong as i32 {
        (*data).payload_len -= 1
    }
    (*data).payload[(*data).payload_len as usize] = 0;
    return QUIRC_SUCCESS;
}

/// Decode a QR-code, returning the payload data.
pub unsafe fn quirc_decode(code: *const Code, mut data: *mut Data) -> quirc_decode_error_t {
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
