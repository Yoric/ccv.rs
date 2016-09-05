use ffi;
use matrix::*;

pub use ffi::Comp;
pub use ffi::SwtParams;

use std::ptr::null_mut;


pub trait TSwt {
    fn swt(&mut self, params: SwtParams) -> Matrix;
    fn detect_words(&mut self, params: SwtParams) -> Vec<Comp>;
}

impl TSwt for Matrix {
    fn swt(&mut self, params: SwtParams) -> Matrix {
        let mut result = null_mut();
        unsafe { ffi::ccv_swt(self.as_c(), &mut result, 0, params) }
        Matrix::from_c(result)
    }
    fn detect_words(&mut self, params: SwtParams) -> Vec<Comp> {
        let array = unsafe { ffi::ccv_swt_detect_words(self.as_c(), params) };
        let start = unsafe { (*array).data as *const Comp };
        let num = unsafe { (*array).rnum };

        // FIXME: Do we really need to copy?
        let mut vec = Vec::new();
        for i in 0..num {
            unsafe {
                let comp = start.clone().offset(i as isize);
                vec.push((*comp).clone())
            }
        }

        unsafe { ffi::ccv_array_free(array) }
        vec
    }
}
