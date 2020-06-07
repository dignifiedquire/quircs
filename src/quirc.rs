use num_derive::{FromPrimitive, ToPrimitive};

pub type Pixel = u16;
pub type DecodeError = u32;

#[derive(Debug, Clone, Default)]
pub struct Quirc {
    pub image: Vec<u8>,
    pub pixels: Vec<Pixel>,
    pub w: usize,
    pub h: usize,
    pub regions: Vec<Region>,
    pub capstones: Vec<Capstone>,
    pub grids: Vec<Grid>,
}

impl Quirc {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resize the QR-code recognizer. The size of an image must be
    /// specified before codes can be analyzed.
    ///
    /// This function returns 0 on success, or -1 if sufficient memory could  not be allocated.
    pub fn resize(&mut self, width: usize, height: usize) {
        let newdim = width * height;
        self.image.resize(newdim, 0);
        self.pixels.resize(newdim, 0);
        self.w = width;
        self.h = height;
    }

    pub fn num_regions(&self) -> usize {
        self.regions.len()
    }

    pub fn num_capstones(&self) -> usize {
        self.capstones.len()
    }

    /// Return the number of QR-codes identified in the last processed image.
    pub fn count(&self) -> usize {
        self.grids.len()
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Grid {
    pub caps: [usize; 3],
    pub align_region: i32,
    pub align: Point,
    pub tpep: [Point; 3],
    pub hscan: i32,
    pub vscan: i32,
    pub grid_size: i32,
    pub c: [f64; 8],
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Capstone {
    pub ring: i32,
    pub stone: i32,
    pub corners: [Point; 4],
    pub center: Point,
    pub c: [f64; 8],
    pub qr_grid: i32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Region {
    pub seed: Point,
    pub count: i32,
    pub capstone: i32,
}

pub const QUIRC_ERROR_DATA_UNDERFLOW: DecodeError = 7;
pub const QUIRC_ERROR_DATA_OVERFLOW: DecodeError = 6;
pub const QUIRC_ERROR_UNKNOWN_DATA_TYPE: DecodeError = 5;
pub const QUIRC_ERROR_DATA_ECC: DecodeError = 4;
pub const QUIRC_ERROR_FORMAT_ECC: DecodeError = 3;
pub const QUIRC_ERROR_INVALID_VERSION: DecodeError = 2;
pub const QUIRC_ERROR_INVALID_GRID_SIZE: DecodeError = 1;
pub const QUIRC_SUCCESS: DecodeError = 0;

/// This structure is used to return information about detected QR codes
/// in the input image.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Code {
    /// The four corners of the QR-code, from top left, clockwise
    pub corners: [Point; 4],
    /// The number of cells across in the QR-code. The cell bitmap
    /// is a bitmask giving the actual values of cells. If the cell
    /// at (x, y) is black, then the following bit is set:
    /// ```ignore
    ///     cell_bitmap[i >> 3] & (1 << (i & 7))
    /// ```
    /// where i = (y * size) + x.
    pub size: i32,
    pub cell_bitmap: [u8; 3917],
}

impl Code {
    pub fn clear(&mut self) {
        for val in self.corners.iter_mut() {
            val.clear();
        }
        self.size = 0;
        for val in self.cell_bitmap.iter_mut() {
            *val = 0;
        }
    }
}

/// This structure holds the decoded QR-code data
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Data {
    ///  Various parameters of the QR-code. These can mostly be  ignored
    /// if you only care about the data.
    pub version: i32,
    pub ecc_level: EccLevel,
    pub mask: i32,
    /// This field is the highest-valued data type found in the QR code.
    pub data_type: i32,
    /// Data payload. For the Kanji datatype, payload is encoded as Shift-JIS.
    /// For all other datatypes, payload is ASCII text.
    pub payload: [u8; 8896],
    pub payload_len: i32,
    /// ECI assignment number
    pub eci: Option<Eci>,
}

impl Default for Data {
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
    Box::into_raw(Box::new(Quirc::new()))
}

/// Destroy a QR-code recognizer.
pub unsafe fn quirc_destroy(q: *mut Quirc) {
    let _q = Box::from_raw(q);
}

/// Resize the QR-code recognizer. The size of an image must be
/// specified before codes can be analyzed.
///
/// This function returns 0 on success, or -1 if sufficient memory could  not be allocated.
pub unsafe fn quirc_resize(q: *mut Quirc, w: usize, h: usize) -> i32 {
    (*q).resize(w, h);
    0
}

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
    Iso8859_1 = 1,
    Ibm437 = 2,
    Iso8859_2 = 4,
    Iso8859_3 = 5,
    Iso8859_4 = 6,
    Iso8859_5 = 7,
    Iso8859_6 = 8,
    Iso8859_7 = 9,
    Iso8859_8 = 10,
    Iso8859_9 = 11,
    Windows874 = 13,
    Iso8859_13 = 15,
    Iso8859_15 = 17,
    ShiftJis = 20,
    Utf8 = 26,
}

static ERROR_TABLE: [&str; 8] = [
    "Success",
    "Invalid grid size",
    "Invalid version",
    "Format data ECC failure",
    "ECC failure",
    "Unknown data type",
    "Data overflow",
    "Data underflow",
];

pub fn quirc_strerror(err: DecodeError) -> &'static str {
    if (err as usize) < ERROR_TABLE.len() {
        return ERROR_TABLE[err as usize];
    }

    "Unknown error"
}
