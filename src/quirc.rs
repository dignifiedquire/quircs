use num_derive::{FromPrimitive, ToPrimitive};

pub type Pixel = u16;

#[derive(Debug, Clone)]
pub struct Quirc {
    pub pixels: Vec<Pixel>,
    pub w: usize,
    pub h: usize,
    pub regions: Vec<Region>,
    pub capstones: Vec<Capstone>,
    pub grids: Vec<Grid>,
}

impl Default for Quirc {
    fn default() -> Self {
        Self {
            pixels: Vec::new(),
            w: 0,
            h: 0,
            regions: Vec::with_capacity(254),
            capstones: Vec::with_capacity(32),
            grids: Vec::with_capacity(8),
        }
    }
}

impl Quirc {
    /// Construct a new QR-code recognizer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resize the QR-code recognizer. The size of an image must be
    /// specified before codes can be analyzed.
    ///
    /// This function returns 0 on success, or -1 if sufficient memory could  not be allocated.
    pub fn resize(&mut self, width: usize, height: usize) {
        if self.w == width && self.h == height {
            return;
        }

        let newdim = width * height;
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

    /// Resets all internal state.
    pub fn reset(&mut self) {
        self.regions.clear();
        self.capstones.clear();
        self.grids.clear();
        self.pixels.clear();
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Grid {
    pub caps: [usize; 3],
    pub align_region: Option<Pixel>,
    pub align: Point,
    pub tpep: [Point; 3],
    pub hscan: i32,
    pub vscan: i32,
    pub grid_size: i32,
    pub c: [f64; 8],
}

#[derive(Debug, Copy, Clone, Default)]
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
pub struct Capstone {
    pub ring: i32,
    pub stone: i32,
    pub corners: [Point; 4],
    pub center: Point,
    pub c: [f64; 8],
    pub qr_grid: i32,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Region {
    pub seed: Point,
    pub count: i32,
    pub capstone: i32,
}

/// This structure is used to return information about detected QR codes
/// in the input image.
#[derive(Copy, Clone)]
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

impl Default for Code {
    fn default() -> Self {
        Self {
            corners: [Point::default(); 4],
            size: 0,
            cell_bitmap: [0; 3917],
        }
    }
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
#[derive(Clone)]
pub struct Data {
    ///  Various parameters of the QR-code. These can mostly be  ignored
    /// if you only care about the data.
    pub version: usize,
    pub ecc_level: EccLevel,
    pub mask: i32,
    /// This field is the highest-valued data type found in the QR code.
    pub data_type: Option<DataType>,
    /// Data payload. For the Kanji datatype, payload is encoded as Shift-JIS.
    /// For all other datatypes, payload is ASCII text.
    pub payload: Vec<u8>,
    /// ECI assignment number
    pub eci: Option<Eci>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            version: 0,
            ecc_level: Default::default(),
            mask: 0,
            data_type: None,
            payload: Vec::new(),
            eci: Default::default(),
        }
    }
}

/// Obtain the library version string.
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// QR-code ECC types.
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive, PartialEq, Eq, Hash)]
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
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DataType {
    Numeric = 1,
    Alpha = 2,
    Byte = 4,
    Eci = 7,
    Kanji = 8,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            DataType::Numeric => "numeric",
            DataType::Alpha => "alpha",
            DataType::Byte => "byte",
            DataType::Eci => "eci",
            DataType::Kanji => "kanji",
        };
        f.write_str(x)
    }
}

/// Common character encodings
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive, PartialEq, Eq, Hash)]
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
