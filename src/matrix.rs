use std::ffi::CString;
use std::path::Path;
use std::ptr::null_mut;

use libc::*;

use ffi;

pub struct Matrix(*mut ffi::DenseMatrix);

pub use ffi::FileFormat;

pub enum OpenAs {
    Any,
    ToGray,
    ToColor
}

// Don't expose this to the public.
pub trait TPrivate {
    fn from_c(ptr: *mut ffi::DenseMatrix) -> Matrix;
    fn as_c(&mut self) -> *mut ffi::DenseMatrix;
}

impl TPrivate for Matrix {
    fn from_c(ptr: *mut ffi::DenseMatrix) -> Matrix {
       assert!(!ptr.is_null());
       Matrix(ptr)
   }
   fn as_c(&mut self) -> *mut ffi::DenseMatrix {
       self.0
   }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe {
            ffi::ccv_matrix_free(self.0);
        }
    }
}

impl Matrix {
    pub fn read<P: AsRef<Path>>(path: P, params: OpenAs) -> Option<Matrix> {
        let path : &str = path.as_ref().to_str().unwrap(); // FIXME: Better error reporting.
        let c_path = CString::new(path).unwrap().as_ptr(); // FIXME: Better error reporting.
        let mut matrix = null_mut();
        let params = match params {
            OpenAs::Any => ffi::FileType::AnyFile as c_int,
            OpenAs::ToGray => ffi::FileType::AnyFile as c_int | ffi::FileType::Gray as c_int,
            OpenAs::ToColor => ffi::FileType::AnyFile as c_int | ffi::FileType::Color as c_int,
        };
        if unsafe { ffi::ccv_read_impl(c_path, &mut matrix, params, 0, 0, 0) } == 0 {
            Some(Matrix(matrix))
        } else {
            None
        }
    }
    pub fn write<P: AsRef<Path>>(&self, path: P, format: FileFormat) -> Option<u64> {
        let path : &str = path.as_ref().to_str().unwrap(); // FIXME: Better error reporting.
        let c_path = CString::new(path).unwrap().as_ptr(); // FIXME: Better error reporting.

        let mut len = 0;

        if unsafe { ffi::ccv_write(self.0, c_path, &mut len, format, null_mut()) } == 0 {
            Some(len as u64)
        } else {
            None
        }
    }
}