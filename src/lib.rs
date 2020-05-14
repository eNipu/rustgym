#![deny(clippy::all)]
#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]

#[cfg_attr(test, macro_use)]
pub extern crate util;

pub mod solutions;

pub use solutions::*;
