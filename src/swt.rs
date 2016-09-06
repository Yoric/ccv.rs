use ffi;
use matrix::*;

pub use ffi::Rect;
pub use ffi::SwtParams;

use std::ptr::null_mut;

impl Default for SwtParams {
    fn default() -> Self {
        SwtParams {
            direction: 1,
            interval: 1,
            same_word_thresh: [ 0.1, 0.8 ],
            min_neighbors: 1,
            scale_invariant: 0,
            size: 3,
            low_thresh: 124,
            high_thresh: 204,
            max_height: 300,
            min_height: 8,
            min_area: 38,
            letter_occlude_thresh: 3,
            aspect_ratio: 8.,
            std_ratio: 0.83,
            thickness_ratio: 1.5,
            height_ratio: 1.7,
            intensity_thresh: 31,
            distance_ratio: 2.9,
            intersect_ratio: 1.3,
            letter_thresh: 3,
            elongate_ratio: 1.9,
            breakdown: 1,
            breakdown_ratio: 1.0,
        }
    }
}

pub trait TSwt {
    fn swt(&mut self, params: SwtParams) -> Matrix;
    fn detect_words(&mut self, params: SwtParams) -> Vec<Rect>;
}

impl TSwt for Matrix {
    fn swt(&mut self, params: SwtParams) -> Matrix {
        let mut result = null_mut();
        unsafe { ffi::ccv_swt(self.as_c(), &mut result, 0, params) }
        Matrix::from_c(result)
    }
    fn detect_words(&mut self, params: SwtParams) -> Vec<Rect> { // FIXME Should this rather be an iterator?
        let array = unsafe { ffi::ccv_swt_detect_words(self.as_c(), params) };
        let start = unsafe { (*array).data as *const Rect };
        let num = unsafe { (*array).rnum };

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
