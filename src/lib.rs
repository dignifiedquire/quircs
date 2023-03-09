//! QR Scanner in Rust. Ported from [quirc](https://github.com/dlbeer/quirc).
//!
//! ## Example
//!
//! ```
//! // open the image from disk
//! let img = image::open("tests/data/Hello+World.png").expect("failed to open image");
//!
//! // convert to gray scale
//! let img_gray = img.into_luma8();
//!
//! // create a decoder
//! let mut decoder = quircs::Quirc::default();
//!
//! // identify all qr codes
//! let codes = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray);
//!
//! for code in codes {
//!     let code = code.expect("failed to extract qr code");
//!     let decoded = code.decode().expect("failed to decode qr code");
//!     println!("qrcode: {}", std::str::from_utf8(&decoded.payload).unwrap());
//! }
//! ```

#![deny(clippy::all)]

mod decode;
mod error;
mod identify;
mod quirc;
mod version_db;

pub use self::decode::*;
pub use self::error::*;
pub use self::identify::*;
pub use self::quirc::*;
pub use self::version_db::*;
