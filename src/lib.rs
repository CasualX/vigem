#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate libloading as lib;
pub mod binds;
pub mod types;

pub use types::vigem::*;
pub use types::target::*;
pub use types::notification;
pub use binds::{XUSB_REPORT, DS4_REPORT};