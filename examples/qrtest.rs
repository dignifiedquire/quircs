#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]

use libc;
use quircs::*;
use std::ffi::CStr;
use std::path::PathBuf;

extern "C" {
    pub type __sFILEX;
    pub type _telldir;

    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(_: *const libc::c_char);
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn puts(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn __error() -> *mut libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn getopt(_: libc::c_int, _: *const *mut libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn lstat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn closedir(_: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn opendir(_: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn readdir(_: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}

pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_uid_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub _seek:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t, _: libc::c_int) -> fpos_t>,
    pub _write: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const libc::c_char,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type off_t = __darwin_off_t;
pub type uint32_t = libc::c_uint;
pub type uid_t = __darwin_uid_t;
pub type gid_t = __darwin_gid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: libc::c_long,
}
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type nlink_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __uint64_t,
    pub d_seekoff: __uint64_t,
    pub d_reclen: __uint16_t,
    pub d_namlen: __uint16_t,
    pub d_type: __uint8_t,
    pub d_name: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DIR {
    pub __dd_fd: libc::c_int,
    pub __dd_loc: libc::c_long,
    pub __dd_size: libc::c_long,
    pub __dd_buf: *mut libc::c_char,
    pub __dd_len: libc::c_int,
    pub __dd_seek: libc::c_long,
    pub __padding: libc::c_long,
    pub __dd_flags: libc::c_int,
    pub __dd_lock: __darwin_pthread_mutex_t,
    pub __dd_td: *mut _telldir,
}
pub type uint8_t = libc::c_uchar;
pub type clockid_t = libc::c_uint;
pub const _CLOCK_THREAD_CPUTIME_ID: clockid_t = 16;
pub const _CLOCK_PROCESS_CPUTIME_ID: clockid_t = 12;
pub const _CLOCK_UPTIME_RAW_APPROX: clockid_t = 9;
pub const _CLOCK_UPTIME_RAW: clockid_t = 8;
pub const _CLOCK_MONOTONIC_RAW_APPROX: clockid_t = 5;
pub const _CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const _CLOCK_MONOTONIC: clockid_t = 6;
pub const _CLOCK_REALTIME: clockid_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct result_info {
    pub file_count: libc::c_int,
    pub id_count: libc::c_int,
    pub decode_count: libc::c_int,
    pub load_time: libc::c_uint,
    pub identify_time: libc::c_uint,
    pub total_time: libc::c_uint,
}

static mut want_verbose: libc::c_int = 1i32;
static mut want_cell_dump: libc::c_int = 0i32;
static mut decoder: *mut Quirc = 0 as *const Quirc as *mut Quirc;

unsafe fn print_result(mut name: *const libc::c_char, mut info: *mut result_info) {
    puts(
        b"-------------------------------------------------------------------------------\x00"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"%s: %d files, %d codes, %d decoded (%d failures)\x00" as *const u8 as *const libc::c_char,
        name,
        (*info).file_count,
        (*info).id_count,
        (*info).decode_count,
        (*info).id_count - (*info).decode_count,
    );
    if (*info).id_count != 0 {
        printf(
            b", %d%% success rate\x00" as *const u8 as *const libc::c_char,
            ((*info).decode_count * 100i32 + (*info).id_count / 2i32) / (*info).id_count,
        );
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    printf(
        b"Total time [load: %u, identify: %u, total: %u]\n\x00" as *const u8 as *const libc::c_char,
        (*info).load_time,
        (*info).identify_time,
        (*info).total_time,
    );
    if (*info).file_count != 0 {
        printf(
            b"Average time [load: %u, identify: %u, total: %u]\n\x00" as *const u8
                as *const libc::c_char,
            (*info)
                .load_time
                .wrapping_div((*info).file_count as libc::c_uint),
            (*info)
                .identify_time
                .wrapping_div((*info).file_count as libc::c_uint),
            (*info)
                .total_time
                .wrapping_div((*info).file_count as libc::c_uint),
        );
    };
}
unsafe fn add_result(mut sum: *mut result_info, mut inf: *mut result_info) {
    (*sum).file_count += (*inf).file_count;
    (*sum).id_count += (*inf).id_count;
    (*sum).decode_count += (*inf).decode_count;
    (*sum).load_time = (*sum).load_time.wrapping_add((*inf).load_time);
    (*sum).identify_time = (*sum).identify_time.wrapping_add((*inf).identify_time);
    (*sum).total_time = (*sum).total_time.wrapping_add((*inf).total_time);
}

unsafe fn load_jpeg(dec: *mut Quirc, path: &PathBuf) -> libc::c_int {
    todo!()
}

unsafe fn load_png(dec: *mut Quirc, path: &PathBuf) -> libc::c_int {
    use image::GenericImageView;

    println!("opening {}", path.display());
    let img = image::open(&path)
        .expect("failed to open image")
        .into_luma();
    let width = img.width() as i32;
    let height = img.height() as i32;

    assert!(quirc_resize(dec, width, height) > -1);

    let image_ptr = quirc_begin(dec, std::ptr::null_mut(), std::ptr::null_mut());
    // copy image to the ptr
    for (x, y, px) in img.enumerate_pixels() {
        *image_ptr.add(y as usize * width as usize + x as usize) = px[0];
    }

    0
}

unsafe fn scan_file(path: &str, mut info: *mut result_info) -> libc::c_int {
    let path = std::path::PathBuf::from(path);
    let mut tp: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    clock_gettime(_CLOCK_PROCESS_CPUTIME_ID, &mut tp);
    let mut start = (tp.tv_sec * 1000i32 as libc::c_long + tp.tv_nsec / 1000000i32 as libc::c_long)
        as libc::c_uint;
    let mut total_start = start;

    let ret = if path.extension().unwrap() == "jpg" || path.extension().unwrap() == "jpeg" {
        load_jpeg(decoder, &path)
    } else if path.extension().unwrap() == "png" {
        load_png(decoder, &path)
    } else {
        panic!("unsupported extension: {:?}", path.extension());
    };

    clock_gettime(_CLOCK_PROCESS_CPUTIME_ID, &mut tp);
    (*info).load_time = ((tp.tv_sec * 1000i32 as libc::c_long
        + tp.tv_nsec / 1000000i32 as libc::c_long) as libc::c_uint)
        .wrapping_sub(start);
    if ret < 0i32 {
        panic!("{}: load failed", path.display());
    }
    clock_gettime(_CLOCK_PROCESS_CPUTIME_ID, &mut tp);
    start = (tp.tv_sec * 1000i32 as libc::c_long + tp.tv_nsec / 1000000i32 as libc::c_long)
        as libc::c_uint;
    quirc_end(decoder);
    clock_gettime(_CLOCK_PROCESS_CPUTIME_ID, &mut tp);
    (*info).identify_time = ((tp.tv_sec * 1000i32 as libc::c_long
        + tp.tv_nsec / 1000000i32 as libc::c_long) as libc::c_uint)
        .wrapping_sub(start);
    (*info).id_count = quirc_count(decoder);
    let mut i = 0i32;
    while i < (*info).id_count {
        let mut code: quirc_code = quirc_code {
            corners: [quirc_point { x: 0, y: 0 }; 4],
            size: 0,
            cell_bitmap: [0; 3917],
        };
        let mut data: quirc_data = quirc_data {
            version: 0,
            ecc_level: 0,
            mask: 0,
            data_type: 0,
            payload: [0; 8896],
            payload_len: 0,
            eci: 0,
        };
        quirc_extract(decoder, i, &mut code);
        if quirc_decode(&mut code, &mut data) as u64 == 0 {
            (*info).decode_count += 1
        }
        i += 1
    }
    clock_gettime(_CLOCK_PROCESS_CPUTIME_ID, &mut tp);
    (*info).total_time = (*info).total_time.wrapping_add(
        ((tp.tv_sec * 1000i32 as libc::c_long + tp.tv_nsec / 1000000i32 as libc::c_long)
            as libc::c_uint)
            .wrapping_sub(total_start),
    );
    println!(
        "  {:?}: {} {} {} {} {}\n",
        path.file_name().unwrap(),
        (*info).load_time,
        (*info).identify_time,
        (*info).total_time,
        (*info).id_count,
        (*info).decode_count,
    );
    if want_cell_dump != 0 || want_verbose != 0 {
        i = 0i32;
        while i < (*info).id_count {
            let mut code_0: quirc_code = quirc_code {
                corners: [quirc_point { x: 0, y: 0 }; 4],
                size: 0,
                cell_bitmap: [0; 3917],
            };
            quirc_extract(decoder, i, &mut code_0);
            if want_cell_dump != 0 {
                dump_cells(&mut code_0);
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
            if want_verbose != 0 {
                let mut data_0: quirc_data = quirc_data {
                    version: 0,
                    ecc_level: 0,
                    mask: 0,
                    data_type: 0,
                    payload: [0; 8896],
                    payload_len: 0,
                    eci: 0,
                };
                let mut err = quirc_decode(&mut code_0, &mut data_0);
                if err as u64 != 0 {
                    printf(
                        b"  ERROR: %s\n\n\x00" as *const u8 as *const libc::c_char,
                        quirc_strerror(err),
                    );
                } else {
                    printf(b"  Decode successful:\n\x00" as *const u8 as *const libc::c_char);
                    dump_data(&mut data_0);
                    printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
            i += 1
        }
    }
    (*info).file_count = 1i32;
    return 1i32;
}

unsafe fn test_scan(path: &str, mut info: *mut result_info) -> libc::c_int {
    scan_file(path, info)
}

unsafe fn run_tests(paths: &[String]) -> libc::c_int {
    let mut sum: result_info = result_info {
        file_count: 0,
        id_count: 0,
        decode_count: 0,
        load_time: 0,
        identify_time: 0,
        total_time: 0,
    };
    let mut count: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    decoder = quirc_new();
    assert!(!decoder.is_null(), "quirc_new");

    println!("  %-30s  %17s %11s\nTime (ms)Count\x00",);
    printf(
        b"  %-30s  %5s %5s %5s %5s %5s\n\x00" as *const u8 as *const libc::c_char,
        b"Filename\x00" as *const u8 as *const libc::c_char,
        b"Load\x00" as *const u8 as *const libc::c_char,
        b"ID\x00" as *const u8 as *const libc::c_char,
        b"Total\x00" as *const u8 as *const libc::c_char,
        b"ID\x00" as *const u8 as *const libc::c_char,
        b"Dec\x00" as *const u8 as *const libc::c_char,
    );
    puts(
        b"-------------------------------------------------------------------------------\x00"
            as *const u8 as *const libc::c_char,
    );
    memset(
        &mut sum as *mut result_info as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<result_info>() as libc::c_ulong,
    );

    for path in paths {
        let mut info: result_info = result_info {
            file_count: 0,
            id_count: 0,
            decode_count: 0,
            load_time: 0,
            identify_time: 0,
            total_time: 0,
        };
        if test_scan(path, &mut info) > 0i32 {
            add_result(&mut sum, &mut info);
            count += 1
        }
        i += 1
    }
    if count > 1i32 {
        print_result(b"TOTAL\x00" as *const u8 as *const libc::c_char, &mut sum);
    }
    quirc_destroy(decoder);
    return 0i32;
}
unsafe fn main_0(args: Vec<String>) -> libc::c_int {
    println!("quirc test program");
    println!(
        "Library version: {}\n",
        CStr::from_ptr(quirc_version()).to_str().unwrap()
    );

    run_tests(&args)
}

unsafe fn dump_data(data: *const quirc_data) {
    let data = *data;
    let levels = "MLHQ";

    println!("    Version: {}", data.version);
    println!(
        "    ECC level: {}",
        levels.as_bytes()[data.ecc_level as usize] as char
    );
    println!("    Mask: {}", data.mask);
    println!(
        "    Data type: {} ({})",
        data.data_type,
        data_type_str(data.data_type)
    );
    println!("    Length: {}", data.payload_len);
    println!(
        "    Payload: {:?}",
        std::str::from_utf8(&data.payload[..data.payload_len as usize])
    );

    if data.eci != 0 {
        println!("    ECI: {}", data.eci);
    }
}

fn data_type_str(dt: i32) -> &'static str {
    match dt {
        QUIRC_DATA_TYPE_NUMERIC => "NUMERIC",
        QUIRC_DATA_TYPE_ALPHA => "ALPHA",
        QUIRC_DATA_TYPE_BYTE => "BYTE",
        QUIRC_DATA_TYPE_KANJI => "KANJI",
        _ => "unknown",
    }
}

unsafe fn dump_cells(code: *const quirc_code) {
    let code = *code;

    print!("    {} cells, corners:", code.size);
    for u in 0..4 {
        print!(" ({},{})", code.corners[u].x, code.corners[u].y);
    }
    println!();

    for v in 0..code.size {
        print!("    ");
        for u in 0..code.size {
            let p = v * code.size + u;

            if (code.cell_bitmap[(p >> 3) as usize] & (1 << (p & 7))) != 0 {
                print!("[]");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    unsafe { std::process::exit(main_0(args) as i32) }
}
