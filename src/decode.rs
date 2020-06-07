#![allow(clippy::many_single_char_names)]

use num_traits::{FromPrimitive, ToPrimitive};

use crate::quirc::*;
use crate::version_db::*;
use crate::DecodeError;

#[derive(Copy, Clone)]
struct Datastream {
    raw: [u8; 8896],
    data_bits: i32,
    ptr: i32,
    data: [u8; 8896],
}

/// Galois Field.
#[derive(Copy, Clone)]
struct GaloisField {
    p: i32,
    log: &'static [u8],
    exp: &'static [u8],
}

static GF16_EXP: [u8; 16] = [
    0x1, 0x2, 0x4, 0x8, 0x3, 0x6, 0xc, 0xb, 0x5, 0xa, 0x7, 0xe, 0xf, 0xd, 0x9, 0x1,
];
static GF16_LOG: [u8; 16] = [
    0, 0xf, 0x1, 0x4, 0x2, 0x8, 0x5, 0xa, 0x3, 0xe, 0x9, 0x7, 0x6, 0xd, 0xb, 0xc,
];

static GF16: GaloisField = GaloisField {
    p: 15,
    log: &GF16_LOG,
    exp: &GF16_EXP,
};

static GF256_EXP: [u8; 256] = [
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
static GF256_LOG: [u8; 256] = [
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

static GF256: GaloisField = GaloisField {
    p: 255,
    log: &GF256_LOG,
    exp: &GF256_EXP,
};

// -- Polynomial operations

fn poly_add(dst: &mut [u8], src: &[u8], c: u8, shift: i32, gf: &GaloisField) {
    if c == 0 {
        return;
    }

    let log_c = gf.log[c as usize] as i32;
    for (i, v) in src.iter().enumerate() {
        let p = i + shift as usize;

        if p >= 64 {
            continue;
        }
        if *v == 0 {
            continue;
        }

        dst[p] ^= (gf.exp[((gf.log[*v as usize] as i32 + log_c) % gf.p) as usize] as i32) as u8;
    }
}

fn poly_eval(s: &[u8], x: u8, gf: &GaloisField) -> u8 {
    if x == 0 {
        return s[0];
    }

    let mut sum = 0;
    let log_x = gf.log[x as usize];

    for i in 0..64 {
        let c = s[i as usize];
        if c == 0 {
            continue;
        }
        sum ^= gf.exp[((gf.log[c as usize] as i32 + log_x as i32 * i) % gf.p) as usize] as u8
    }

    sum
}

/// Berlekamp-Massey algorithm for finding error locator polynomials.
#[allow(non_snake_case)]
fn berlekamp_massey(s: &[u8], n: usize, gf: &GaloisField, sigma: &mut [u8]) {
    let mut C: [u8; 64] = [0; 64];
    let mut B: [u8; 64] = [0; 64];
    let mut L = 0;
    let mut m = 1;
    let mut b = 1;

    B[0] = 1;
    C[0] = 1;

    for n in 0..n {
        let mut d = s[n];
        let mult: u8;
        for i in 1..=L {
            if C[i] as i32 != 0 && s[n - i] as i32 != 0 {
                let a = gf.log[C[i] as usize] as usize;
                let b = gf.log[s[n - i] as usize] as usize;
                let index = (a + b) % gf.p as usize;

                d ^= gf.exp[index] as u8;
            }
        }

        mult = gf.exp
            [((gf.p - gf.log[b as usize] as i32 + gf.log[d as usize] as i32) % gf.p) as usize];

        if d == 0 {
            m += 1
        } else if L * 2 <= n {
            let T = C;
            poly_add(&mut C, &B, mult, m, gf);
            B.copy_from_slice(&T);
            L = n + 1 - L;
            b = d;
            m = 1
        } else {
            poly_add(&mut C, &B, mult, m, gf);
            m += 1
        }
    }

    sigma[..64].copy_from_slice(&C);
}

/// Code stream error correction
///
/// Generator polynomial for GF(2^8) is x^8 + x^4 + x^3 + x^2 + 1
fn block_syndromes(data: &[u8], bs: i32, npar: usize, s: &mut [u8]) -> i32 {
    for val in s.iter_mut().take(64) {
        *val = 0;
    }

    let mut nonzero = 0;

    for i in 0..npar {
        for j in 0..bs {
            let c = data[(bs - j - 1) as usize];
            if c == 0 {
                continue;
            }
            s[i] ^= GF256_EXP[((GF256_LOG[c as usize] as i32 + i as i32 * j) % 255) as usize] as u8;
        }

        if s[i] != 0 {
            nonzero = 1;
        }
    }

    nonzero
}

fn eloc_poly(omega: &mut [u8], s: &[u8], sigma: &[u8], npar: usize) {
    for val in omega.iter_mut().take(64) {
        *val = 0;
    }

    for i in 0..npar {
        let a = sigma[i];
        let log_a = GF256_LOG[a as usize];
        if a == 0 {
            continue;
        }
        for j in 0..64 - 1 {
            let b = s[j + 1];
            if i + j >= npar {
                break;
            }
            if b == 0 {
                continue;
            }

            omega[i + j] ^=
                GF256_EXP[((log_a as i32 + GF256_LOG[b as usize] as i32) % 255) as usize] as u8;
        }
    }
}

fn correct_block(data: &mut [u8], ecc: &RsParams) -> Result<(), DecodeError> {
    let npar = ecc.bs as usize - ecc.dw as usize;
    let mut s: [u8; 64] = [0; 64];
    let mut sigma: [u8; 64] = [0; 64];
    let mut sigma_deriv: [u8; 64] = [0; 64];
    let mut omega: [u8; 64] = [0; 64];
    /* Compute syndrome vector */
    if block_syndromes(data, ecc.bs, npar, &mut s) == 0 {
        return Ok(());
    }
    berlekamp_massey(&s, npar, &GF256, &mut sigma);
    /* Compute derivative of sigma */
    let mut i = 0;
    while i + 1 < 64 {
        sigma_deriv[i as usize] = sigma[(i + 1) as usize];
        i += 2
    }
    /* Compute error evaluator polynomial */
    eloc_poly(&mut omega, &s, &sigma, npar - 1);

    /* Find error locations and magnitudes */
    i = 0;
    while i < ecc.bs {
        let xinv = GF256_EXP[(255 - i) as usize];
        if poly_eval(&sigma, xinv, &GF256) == 0 {
            let sd_x = poly_eval(&sigma_deriv, xinv, &GF256);
            let omega_x = poly_eval(&omega, xinv, &GF256);
            let error = GF256_EXP[((255 - GF256_LOG[sd_x as usize] as i32
                + GF256_LOG[omega_x as usize] as i32)
                % 255) as usize];

            let index = (ecc.bs - i - 1) as usize;
            data[index] ^= error as u8;
        }
        i += 1
    }
    if block_syndromes(data, ecc.bs, npar, &mut s) != 0 {
        return Err(DecodeError::DataEcc);
    }
    Ok(())
}

fn format_syndromes(u: u16, s: &mut [u8]) -> i32 {
    let mut nonzero = 0;
    for val in s.iter_mut().take(64) {
        *val = 0;
    }
    let mut i = 0;
    while i < 3 * 2 {
        s[i] = 0;
        for j in 0..15 {
            if u as i32 & 1 << j != 0 {
                s[i] ^= GF16_EXP[((i + 1) * j % 15) as usize] as u8;
            }
        }

        if s[i] != 0 {
            nonzero = 1;
        }
        i += 1
    }

    nonzero
}

fn correct_format(f_ret: &mut u16) -> Result<(), DecodeError> {
    let mut u: u16 = *f_ret;
    let mut s: [u8; 64] = [0; 64];
    let mut sigma: [u8; 64] = [0; 64];

    /* Evaluate U (received codeword) at each of alpha_1 .. alpha_6
     * to get S_1 .. S_6 (but we index them from 0).
     */
    if format_syndromes(u, &mut s) == 0 {
        return Ok(());
    }
    berlekamp_massey(&s, 3 * 2, &GF16, &mut sigma);

    /* Now, find the roots of the polynomial */
    for i in 0..15 {
        if poly_eval(&sigma, GF16_EXP[(15 - i) as usize], &GF16) == 0 {
            u ^= (1 << i) as u16;
        }
    }

    if format_syndromes(u, &mut s) != 0 {
        return Err(DecodeError::FormatEcc);
    }
    *f_ret = u;

    Ok(())
}

#[inline]
fn grid_bit(code: &Code, x: i32, y: i32) -> i32 {
    let p: i32 = y * code.size + x;
    code.cell_bitmap[(p >> 3) as usize] as i32 >> (p & 7) & 1
}

fn read_format(code: &Code, data: &mut Data, which: i32) -> Result<(), DecodeError> {
    let mut format: u16 = 0 as u16;
    if which != 0 {
        for i in 0..7 {
            format = ((format as i32) << 1 | grid_bit(code, 8, code.size - 1 - i)) as u16;
        }
        for i in 0..8 {
            format = ((format as i32) << 1 | grid_bit(code, code.size - 8 + i, 8)) as u16;
        }
    } else {
        static XS: [i32; 15] = [8, 8, 8, 8, 8, 8, 8, 8, 7, 5, 4, 3, 2, 1, 0];
        static YS: [i32; 15] = [0, 1, 2, 3, 4, 5, 7, 8, 8, 8, 8, 8, 8, 8, 8];

        for i in (0..=14).rev() {
            format = ((format as i32) << 1 | grid_bit(code, XS[i as usize], YS[i as usize])) as u16;
        }
    }
    format ^= 0x5412;

    correct_format(&mut format)?;

    let fdata = (format as i32 >> 10) as u16;
    data.ecc_level = EccLevel::from_i32(fdata as i32 >> 3).unwrap();
    data.mask = fdata as i32 & 7;

    Ok(())
}

fn mask_bit(mask: i32, i: i32, j: i32) -> i32 {
    match mask {
        0 => ((i + j) % 2 == 0) as i32,
        1 => (i % 2 == 0) as i32,
        2 => (j % 3 == 0) as i32,
        3 => ((i + j) % 3 == 0) as i32,
        4 => ((i / 2 + j / 3) % 2 == 0) as i32,
        5 => (i * j % 2 + i * j % 3 == 0) as i32,
        6 => ((i * j % 2 + i * j % 3) % 2 == 0) as i32,
        7 => ((i * j % 3 + (i + j) % 2) % 2 == 0) as i32,
        _ => 0,
    }
}

fn reserved_cell(version: i32, i: i32, j: i32) -> i32 {
    let ver = &VERSION_DB[version as usize];
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
    while a < 7 && ver.apat[a as usize] != 0 {
        let p: i32 = ver.apat[a as usize];
        if (p - i).abs() < 3 {
            ai = a
        }
        if (p - j).abs() < 3 {
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

    0
}

fn read_bit(code: &Code, data: &mut Data, mut ds: &mut Datastream, i: i32, j: i32) {
    let bitpos: i32 = ds.data_bits & 7;
    let bytepos: i32 = ds.data_bits >> 3;
    let mut v: i32 = grid_bit(code, j, i);
    if mask_bit(data.mask, i, j) != 0 {
        v ^= 1
    }
    if v != 0 {
        ds.raw[bytepos as usize] = (ds.raw[bytepos as usize] as i32 | 0x80 >> bitpos) as u8;
    }
    ds.data_bits += 1;
}

fn read_data(code: &Code, data: &mut Data, ds: &mut Datastream) {
    let mut y: i32 = code.size - 1;
    let mut x: i32 = code.size - 1;
    let mut dir: i32 = -1;
    while x > 0 {
        if x == 6 {
            x -= 1
        }
        if reserved_cell(data.version, y, x) == 0 {
            read_bit(code, data, ds, y, x);
        }
        if reserved_cell(data.version, y, x - 1) == 0 {
            read_bit(code, data, ds, y, x - 1);
        }
        y += dir;
        if y < 0 || y >= code.size {
            dir = -dir;
            x -= 2;
            y += dir
        }
    }
}

fn codestream_ecc(data: &mut Data, mut ds: &mut Datastream) -> Result<(), DecodeError> {
    let ver = &VERSION_DB[data.version as usize];
    let sb_ecc = &ver.ecc[data.ecc_level as usize];

    let lb_count = (ver.data_bytes - sb_ecc.bs * sb_ecc.ns) / (sb_ecc.bs + 1);
    let bc = lb_count + sb_ecc.ns;
    let ecc_offset = sb_ecc.dw * bc + lb_count;
    let mut dst_offset = 0;

    let mut lb_ecc = sb_ecc.clone();
    lb_ecc.dw += 1;
    lb_ecc.bs += 1;

    for i in 0..bc {
        let dst = &mut ds.data[dst_offset as usize..];
        let ecc = if i < sb_ecc.ns { sb_ecc } else { &lb_ecc };
        let num_ec = ecc.bs - ecc.dw;
        for j in 0..ecc.dw {
            dst[j as usize] = ds.raw[(j * bc + i) as usize];
        }
        for j in 0..num_ec {
            dst[(ecc.dw + j) as usize] = ds.raw[(ecc_offset + j * bc + i) as usize];
        }

        correct_block(dst, ecc)?;
        dst_offset += ecc.dw;
    }

    ds.data_bits = dst_offset * 8;
    Ok(())
}

#[inline]
fn bits_remaining(ds: &Datastream) -> i32 {
    ds.data_bits - ds.ptr
}

fn take_bits(mut ds: &mut Datastream, mut len: i32) -> i32 {
    let mut ret: i32 = 0;
    while len != 0 && ds.ptr < ds.data_bits {
        let b: u8 = ds.data[(ds.ptr >> 3) as usize];
        let bitpos: i32 = ds.ptr & 7;
        ret <<= 1;
        if (b as i32) << bitpos & 0x80 != 0 {
            ret |= 1
        }
        ds.ptr += 1;
        len -= 1
    }

    ret
}

fn numeric_tuple(mut data: &mut Data, ds: &mut Datastream, bits: i32, digits: i32) -> i32 {
    if bits_remaining(ds) < bits {
        return -1;
    }
    let mut tuple = take_bits(ds, bits);
    let mut i = digits - 1;
    while i >= 0 {
        data.payload[(data.payload_len + i) as usize] = (tuple % 10 + '0' as i32) as u8;
        tuple /= 10;
        i -= 1
    }
    data.payload_len += digits;

    0
}

fn decode_numeric(data: &mut Data, ds: &mut Datastream) -> Result<(), DecodeError> {
    let mut bits: i32 = 14;
    if data.version < 10 {
        bits = 10;
    } else if data.version < 27 {
        bits = 12;
    }
    let mut count = take_bits(ds, bits);
    if data.payload_len + count + 1 > 8896 {
        return Err(DecodeError::DataOverflow);
    }
    while count >= 3 {
        if numeric_tuple(data, ds, 10, 3) < 0 {
            return Err(DecodeError::DataUnderflow);
        }
        count -= 3;
    }
    if count >= 2 {
        if numeric_tuple(data, ds, 7, 2) < 0 {
            return Err(DecodeError::DataUnderflow);
        }
        count -= 2;
    }

    if count != 0 && numeric_tuple(data, ds, 4, 1) < 0 {
        return Err(DecodeError::DataUnderflow);
    }

    Ok(())
}

fn alpha_tuple(mut data: &mut Data, ds: &mut Datastream, bits: i32, digits: i32) -> i32 {
    if bits_remaining(ds) < bits {
        return -1;
    }
    let mut tuple = take_bits(ds, bits);
    static ALPHA_MAP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";

    for i in 0..digits {
        data.payload[(data.payload_len + digits - i - 1) as usize] =
            ALPHA_MAP[(tuple % 45) as usize] as u8;
        tuple /= 45;
    }
    data.payload_len += digits;
    0
}

fn decode_alpha(data: &mut Data, ds: &mut Datastream) -> Result<(), DecodeError> {
    let mut bits: i32 = 13;
    if data.version < 10 {
        bits = 9
    } else if data.version < 27 {
        bits = 11
    }
    let mut count = take_bits(ds, bits);
    if data.payload_len + count + 1 > 8896 {
        return Err(DecodeError::DataOverflow);
    }
    while count >= 2 {
        if alpha_tuple(data, ds, 11, 2) < 0 {
            return Err(DecodeError::DataUnderflow);
        }
        count -= 2
    }
    if count != 0 && alpha_tuple(data, ds, 6, 1) < 0 {
        return Err(DecodeError::DataUnderflow);
    }

    Ok(())
}

fn decode_byte(mut data: &mut Data, ds: &mut Datastream) -> Result<(), DecodeError> {
    let bits = if data.version < 10 { 8 } else { 16 };
    let count = take_bits(ds, bits);
    if data.payload_len + count + 1 > 8896 {
        return Err(DecodeError::DataOverflow);
    }
    if bits_remaining(ds) < count * 8 {
        return Err(DecodeError::DataUnderflow);
    }

    for _i in 0..count {
        let len = data.payload_len;
        data.payload_len += 1;
        data.payload[len as usize] = take_bits(ds, 8) as u8;
    }

    Ok(())
}

fn decode_kanji(mut data: &mut Data, ds: &mut Datastream) -> Result<(), DecodeError> {
    let mut bits = 12;
    if data.version < 10 {
        bits = 8;
    } else if data.version < 27 {
        bits = 10;
    }

    let count = take_bits(ds, bits);
    if data.payload_len + count * 2 + 1 > 8896 {
        return Err(DecodeError::DataOverflow);
    }
    if bits_remaining(ds) < count * 13 {
        return Err(DecodeError::DataUnderflow);
    }

    for _i in 0..count {
        let d = take_bits(ds, 13);
        let msb = d / 0xc0;
        let lsb = d % 0xc0;
        let intermediate = msb << 8 | lsb;
        let sjw = if intermediate + 0x8140 <= 0x9ffc {
            /* bytes are in the range 0x8140 to 0x9FFC */
            (intermediate + 0x8140) as u16
        } else {
            /* bytes are in the range 0xE040 to 0xEBBF */
            (intermediate + 0xc140) as u16
        };

        let len = data.payload_len;
        data.payload_len += 1;
        data.payload[len as usize] = (sjw as i32 >> 8) as u8;
        let fresh7 = data.payload_len;
        data.payload_len += 1;
        data.payload[fresh7 as usize] = (sjw as i32 & 0xff) as u8;
    }

    Ok(())
}

fn decode_eci(mut data: &mut Data, ds: &mut Datastream) -> Result<(), DecodeError> {
    if bits_remaining(ds) < 8 {
        return Err(DecodeError::DataUnderflow);
    }
    data.eci = Eci::from_u32(take_bits(ds, 8) as u32);
    if data.eci.and_then(|e| e.to_u32()).unwrap_or_default() & 0xc0 as u32 == 0x80 as u32 {
        if bits_remaining(ds) < 8 {
            return Err(DecodeError::DataUnderflow);
        }
        data.eci = Eci::from_u32(
            data.eci.and_then(|e| e.to_u32()).unwrap_or_default() << 8 | take_bits(ds, 8) as u32,
        );
    } else if data.eci.and_then(|e| e.to_u32()).unwrap_or_default() & 0xe0 as u32 == 0xc0 as u32 {
        if bits_remaining(ds) < 16 {
            return Err(DecodeError::DataUnderflow);
        }
        data.eci = Eci::from_u32(
            data.eci.and_then(|e| e.to_u32()).unwrap_or_default() << 16 | take_bits(ds, 16) as u32,
        );
    }

    Ok(())
}

fn decode_payload(mut data: &mut Data, ds: &mut Datastream) -> Result<(), DecodeError> {
    while bits_remaining(ds) >= 4 {
        let type_0 = take_bits(ds, 4);
        match type_0 {
            1 => decode_numeric(data, ds)?,
            2 => decode_alpha(data, ds)?,
            4 => decode_byte(data, ds)?,
            8 => decode_kanji(data, ds)?,
            7 => decode_eci(data, ds)?,
            _ => {
                break;
            }
        }

        if type_0 & (type_0 - 1) == 0 && type_0 > data.data_type {
            data.data_type = type_0
        }
    }
    /* Add nul terminator to all payloads */
    if data.payload_len >= ::std::mem::size_of::<[u8; 8896]>() as i32 {
        data.payload_len -= 1;
    }
    data.payload[data.payload_len as usize] = 0;

    Ok(())
}

impl Code {
    /// Decode a QR-code, returning the payload data.
    pub fn decode(&self) -> Result<Data, DecodeError> {
        let mut ds: Datastream = Datastream {
            raw: [0; 8896],
            data_bits: 0,
            ptr: 0,
            data: [0; 8896],
        };

        if (self.size - 17) % 4 != 0 {
            return Err(DecodeError::InvalidGridSize);
        }

        let mut data = Data::default();

        data.version = (self.size - 17) / 4;
        if data.version < 1 || data.version > 40 {
            return Err(DecodeError::InvalidVersion);
        }

        /* Read format information -- try both locations */
        let mut res = read_format(self, &mut data, 0);
        if res.is_err() {
            res = read_format(self, &mut data, 1);
        }
        res?;

        read_data(self, &mut data, &mut ds);
        codestream_ecc(&mut data, &mut ds)?;
        decode_payload(&mut data, &mut ds)?;

        Ok(data)
    }
}
