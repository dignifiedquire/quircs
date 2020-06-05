#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]

mod decode;
mod identify;
mod quirc;
mod version_db;

pub use self::decode::*;
pub use self::identify::*;
pub use self::quirc::*;
pub use self::version_db::*;
