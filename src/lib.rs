extern crate libc;

mod ffi;
mod matrix;
pub mod swt;

pub use matrix::Matrix;

pub use matrix::OpenAs;
pub use matrix::FileFormat;