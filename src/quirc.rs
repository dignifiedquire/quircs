use libc;
use num_derive::{FromPrimitive, ToPrimitive};

pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;

#[derive(Clone)]
#[repr(C)]
pub struct Quirc {
    pub image: Vec<uint8_t>,
    pub pixels: Vec<quirc_pixel_t>,
    pub w: usize,
    pub h: usize,
    pub regions: Vec<quirc_region>,
    pub num_capstones: libc::c_int,
    pub capstones: [quirc_capstone; 32],
    pub num_grids: libc::c_int,
    pub grids: [quirc_grid; 8],
}

impl Quirc {
    pub fn num_regions(&self) -> usize {
        self.regions.len()
    }
}

impl Default for Quirc {
    fn default() -> Self {
        Quirc {
            image: Default::default(),
            pixels: Default::default(),
            w: 0,
            h: 0,
            regions: Default::default(),
            num_capstones: 0,
            capstones: [Default::default(); 32],
            num_grids: 0,
            grids: [Default::default(); 8],
        }
    }
}

#[derive(Copy, Clone, Default)]
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

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct quirc_point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct quirc_capstone {
    pub ring: libc::c_int,
    pub stone: libc::c_int,
    pub corners: [quirc_point; 4],
    pub center: quirc_point,
    pub c: [libc::c_double; 8],
    pub qr_grid: libc::c_int,
}

#[derive(Copy, Clone, Default)]
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
    pub ecc_level: EccLevel,
    pub mask: libc::c_int,
    /// This field is the highest-valued data type found in the QR code.
    pub data_type: libc::c_int,
    /// Data payload. For the Kanji datatype, payload is encoded as Shift-JIS.
    /// For all other datatypes, payload is ASCII text.
    pub payload: [uint8_t; 8896],
    pub payload_len: libc::c_int,
    /// ECI assignment number
    pub eci: Option<Eci>,
}

impl Default for quirc_data {
    fn default() -> Self {
        Self {
            version: 0,
            ecc_level: Default::default(),
            mask: 0,
            data_type: 0,
            payload: [0; 8896],
            payload_len: 0,
            eci: Default::default(),
        }
    }
}

/// Obtain the library version string.
pub fn quirc_version() -> &'static str {
    "1.0"
}

/// Construct a new QR-code recognizer. This function will return NULL
/// if sufficient memory could not be allocated.
pub unsafe fn quirc_new() -> *mut Quirc {
    let quirc = Quirc::default();
    Box::into_raw(Box::new(quirc))
}

/// Destroy a QR-code recognizer.
pub unsafe fn quirc_destroy(q: *mut Quirc) {
    let _q = Box::from_raw(q);
}

/// Resize the QR-code recognizer. The size of an image must be
/// specified before codes can be analyzed.
///
/// This function returns 0 on success, or -1 if sufficient memory could  not be allocated.
pub unsafe fn quirc_resize(q: *mut Quirc, w: usize, h: usize) -> libc::c_int {
    let q = &mut *q;
    let newdim = w * h;
    q.image.resize(newdim, 0);
    q.pixels.resize(newdim, 0);
    q.w = w;
    q.h = h;

    0
}

// Limits on the maximum size of QR-codes and their content.
const QUIRC_MAX_BITMAP: usize = 3917;
const QUIRC_MAX_PAYLOAD: usize = 8896;

/// QR-code ECC types.
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum EccLevel {
    M = 0,
    L = 1,
    H = 2,
    Q = 3,
}

impl Default for EccLevel {
    fn default() -> Self {
        EccLevel::M
    }
}

/// QR-code data types.
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum DataType {
    Numeric = 1,
    Alpha = 2,
    Byte = 4,
    Kanji = 8,
}

/// Common character encodings
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
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

pub unsafe fn quirc_strerror(err: quirc_decode_error_t) -> *const libc::c_char {
    if err as libc::c_uint >= 0 as libc::c_uint
        && (err as libc::c_ulong)
            < (::std::mem::size_of::<[*const libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        return error_table[err as usize];
    }
    return b"Unknown error\x00" as *const u8 as *const libc::c_char;
}
