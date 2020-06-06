use libc::{self, calloc, free, malloc, memcpy, memset};

pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Quirc {
    pub image: *mut uint8_t,
    pub pixels: *mut quirc_pixel_t,
    pub w: usize,
    pub h: usize,
    pub num_regions: libc::c_int,
    pub regions: [quirc_region; 65534],
    pub num_capstones: libc::c_int,
    pub capstones: [quirc_capstone; 32],
    pub num_grids: libc::c_int,
    pub grids: [quirc_grid; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct quirc_grid {
    pub caps: [libc::c_int; 3],
    pub align_region: libc::c_int,
    pub align: quirc_point,
    pub tpep: [quirc_point; 3],
    pub hscan: libc::c_int,
    pub vscan: libc::c_int,
    pub grid_size: libc::c_int,
    pub c: [libc::c_double; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quirc_point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quirc_capstone {
    pub ring: libc::c_int,
    pub stone: libc::c_int,
    pub corners: [quirc_point; 4],
    pub center: quirc_point,
    pub c: [libc::c_double; 8],
    pub qr_grid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quirc_region {
    pub seed: quirc_point,
    pub count: libc::c_int,
    pub capstone: libc::c_int,
}
pub type quirc_pixel_t = uint16_t;
pub type quirc_decode_error_t = libc::c_uint;

pub const QUIRC_ERROR_DATA_UNDERFLOW: quirc_decode_error_t = 7;
pub const QUIRC_ERROR_DATA_OVERFLOW: quirc_decode_error_t = 6;
pub const QUIRC_ERROR_UNKNOWN_DATA_TYPE: quirc_decode_error_t = 5;
pub const QUIRC_ERROR_DATA_ECC: quirc_decode_error_t = 4;
pub const QUIRC_ERROR_FORMAT_ECC: quirc_decode_error_t = 3;
pub const QUIRC_ERROR_INVALID_VERSION: quirc_decode_error_t = 2;
pub const QUIRC_ERROR_INVALID_GRID_SIZE: quirc_decode_error_t = 1;
pub const QUIRC_SUCCESS: quirc_decode_error_t = 0;

/// This structure is used to return information about detected QR codes
/// in the input image.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quirc_code {
    /// The four corners of the QR-code, from top left, clockwise
    pub corners: [quirc_point; 4],
    /// The number of cells across in the QR-code. The cell bitmap
    /// is a bitmask giving the actual values of cells. If the cell
    /// at (x, y) is black, then the following bit is set:
    /// ```ignore
    ///     cell_bitmap[i >> 3] & (1 << (i & 7))
    /// ```
    /// where i = (y * size) + x.
    pub size: libc::c_int,
    pub cell_bitmap: [uint8_t; 3917],
}

/// This structure holds the decoded QR-code data
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quirc_data {
    ///  Various parameters of the QR-code. These can mostly be  ignored
    /// if you only care about the data.
    pub version: libc::c_int,
    pub ecc_level: libc::c_int,
    pub mask: libc::c_int,
    /// This field is the highest-valued data type found in the QR code.
    pub data_type: libc::c_int,
    /// Data payload. For the Kanji datatype, payload is encoded as Shift-JIS.
    /// For all other datatypes, payload is ASCII text.
    pub payload: [uint8_t; 8896],
    pub payload_len: libc::c_int,
    /// ECI assignment number
    pub eci: uint32_t,
}

/// Obtain the library version string.
pub fn quirc_version() -> &'static str {
    "1.0"
}

/// Construct a new QR-code recognizer. This function will return NULL
/// if sufficient memory could not be allocated.
pub unsafe fn quirc_new() -> *mut Quirc {
    let mut q: *mut Quirc = malloc(std::mem::size_of::<Quirc>()) as *mut Quirc;
    if q.is_null() {
        return 0 as *mut Quirc;
    }
    memset(q as *mut libc::c_void, 0i32, ::std::mem::size_of::<Quirc>());
    q
}

/// Destroy a QR-code recognizer.
pub unsafe fn quirc_destroy(mut q: *mut Quirc) {
    free((*q).image as *mut libc::c_void);
    /* q->pixels may alias q->image when their type representation is of the
    same size, so we need to be careful here to avoid a double free */
    if 0i32 == 0 {
        free((*q).pixels as *mut libc::c_void);
    }
    free(q as *mut libc::c_void);
}

/// Resize the QR-code recognizer. The size of an image must be
/// specified before codes can be analyzed.
///
/// This function returns 0 on success, or -1 if sufficient memory could  not be allocated.
pub unsafe fn quirc_resize(mut q: *mut Quirc, mut w: usize, mut h: usize) -> libc::c_int {
    let mut olddim: usize;
    let mut newdim: usize;
    let mut min: usize;
    let mut current_block: u64;
    let mut image: *mut uint8_t;
    let mut pixels: *mut quirc_pixel_t = 0 as *mut quirc_pixel_t;
    /*
     * XXX: w and h should be usize (or at least unsigned) as negatives
     * values would not make much sense. The downside is that it would break
     * both the API and ABI. Thus, at the moment, let's just do a sanity
     * check.
     */

    /*
     * alloc a new buffer for q->image. We avoid realloc(3) because we want
     * on failure to be leave `q` in a consistant, unmodified state.
     */
    image = calloc(w, h) as *mut uint8_t;
    if !image.is_null() {
        /* compute the "old" (i.e. currently allocated) and the "new"
        (i.e. requested) image dimensions */
        olddim = ((*q).w * (*q).h) as usize;
        newdim = (w * h) as usize;
        min = if olddim < newdim { olddim } else { newdim };
        /*
         * copy the data into the new buffer, avoiding (a) to read beyond the
         * old buffer when the new size is greater and (b) to write beyond the
         * new buffer when the new size is smaller, hence the min computation.
         */
        memcpy(
            image as *mut libc::c_void,
            (*q).image as *const libc::c_void,
            min,
        );
        /* alloc a new buffer for q->pixels if needed */
        if 0i32 == 0 {
            pixels = calloc(newdim, std::mem::size_of::<quirc_pixel_t>()) as *mut quirc_pixel_t;
            if pixels.is_null() {
                current_block = 11234461503687749102;
            } else {
                current_block = 13109137661213826276;
            }
        } else {
            current_block = 13109137661213826276;
        }
        match current_block {
            11234461503687749102 => {}
            _ => {
                /* alloc succeeded, update `q` with the new size and buffers */
                (*q).w = w;
                (*q).h = h;
                free((*q).image as *mut libc::c_void);
                (*q).image = image;
                if 0i32 == 0 {
                    free((*q).pixels as *mut libc::c_void);
                    (*q).pixels = pixels
                }
                return 0i32;
            }
        }
    }

    /* NOTREACHED */
    free(image as *mut libc::c_void);
    free(pixels as *mut libc::c_void);
    -1i32
}

// Limits on the maximum size of QR-codes and their content.
const QUIRC_MAX_BITMAP: usize = 3917;
const QUIRC_MAX_PAYLOAD: usize = 8896;

/// QR-code ECC types.
#[derive(Debug)]
pub enum EccLevel {
    M = 0,
    L = 1,
    H = 2,
    Q = 3,
}

/// QR-code data types.
#[derive(Debug)]
pub enum DataType {
    Numeric = 1,
    Alpha = 2,
    Byte = 4,
    Kanji = 8,
}

/// Common character encodings
#[derive(Debug)]
pub enum Eci {
    ISO_8859_1 = 1,
    IBM437 = 2,
    ISO_8859_2 = 4,
    ISO_8859_3 = 5,
    ISO_8859_4 = 6,
    ISO_8859_5 = 7,
    ISO_8859_6 = 8,
    ISO_8859_7 = 9,
    ISO_8859_8 = 10,
    ISO_8859_9 = 11,
    WINDOWS_874 = 13,
    ISO_8859_13 = 15,
    ISO_8859_15 = 17,
    SHIFT_JIS = 20,
    UTF_8 = 26,
}

/// Return the number of QR-codes identified in the last processed image.
pub unsafe fn quirc_count(q: *const Quirc) -> libc::c_int {
    (*q).num_grids
}

static mut error_table: [*const libc::c_char; 8] = [
    b"Success\x00" as *const u8 as *const libc::c_char,
    b"Invalid grid size\x00" as *const u8 as *const libc::c_char,
    b"Invalid version\x00" as *const u8 as *const libc::c_char,
    b"Format data ECC failure\x00" as *const u8 as *const libc::c_char,
    b"ECC failure\x00" as *const u8 as *const libc::c_char,
    b"Unknown data type\x00" as *const u8 as *const libc::c_char,
    b"Data overflow\x00" as *const u8 as *const libc::c_char,
    b"Data underflow\x00" as *const u8 as *const libc::c_char,
];

pub unsafe fn quirc_strerror(mut err: quirc_decode_error_t) -> *const libc::c_char {
    if err as libc::c_uint >= 0i32 as libc::c_uint
        && (err as libc::c_ulong)
            < (::std::mem::size_of::<[*const libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        return error_table[err as usize];
    }
    return b"Unknown error\x00" as *const u8 as *const libc::c_char;
}
