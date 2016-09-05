use libc::*;

#[repr(C)]
pub struct DenseMatrix {
    #[allow(dead_code)]
    placeholder: c_int,
}


#[repr(C)]
pub enum FileType {
    AnyFile = 0x020,
    Gray = 0x100,
    Color = 0x300
}

#[repr(C)]
pub enum FileFormat {
    BMP = 0x021,
    JPEG = 0x022,
    PNG = 0x023,
}

#[repr(C)]
pub enum Depth {
    U8 = 0x01000,
    S32 = 0x02000,
    F32 = 0x04000,
    S64 = 0x08000,
    F64 = 0x10000,
}

#[repr(C)]
pub enum Coordinate {
    C1 = 0x001,
    C2 = 0x002,
    C3 = 0x003,
    C4 = 0x004,
}

#[repr(C)]
pub struct SwtParams {
    interval: c_int,
    min_neighbors: c_int,
    scale_invariant: c_int,
    direction: c_int,
    same_word_thresh: [c_double; 2],

    size: c_int,
    low_thresh: c_int,
    high_thresh: c_int,
    max_height: c_int,
    min_height: c_int,
    min_area: c_int,
    letter_occlude_thresh: c_int,
    aspect_ratio: c_double,
    std_ratio: c_double,
    thickness_ratio: c_double,
    height_ratio: c_double,
    intensity_thresh: c_int,
    distance_ratio: c_double,
    intersect_ratio: c_double,
    elongate_ratio: c_double,
    letter_thresh: c_int,
    breakdown: c_int,
    breakdown_ratio: c_double,
}

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

#[repr(C)]
#[derive(Clone)]
pub struct Array {
    pub type_: c_int,
    pub sig: u64,
    pub refcount: c_int,
    pub rnum: c_int,
    pub size: c_int,
    pub rsize: c_int,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct Classification {
    pub id: c_int,
    pub confidence: c_float,
}

#[repr(C)]
#[derive(Clone)]
pub struct Comp {
    pub rect: Rect,
    pub neighbors: c_int,
    pub classification: Classification
}

#[repr(C)]
#[derive(Clone)]
pub struct Rect {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int
}

extern {
    pub fn ccv_read_impl(path: *const c_char, out: *mut *mut DenseMatrix, type_: c_int, rows: c_int, cols: c_int, scanline: c_int) -> c_int;

    pub fn ccv_write(mat: *mut DenseMatrix, path: *const c_char, len: *mut c_int, format: FileFormat, conf: *mut c_void) -> c_int;

    pub fn ccv_matrix_free(mat: *mut DenseMatrix);
    pub fn ccv_array_free(array: *mut Array);

    pub fn ccv_swt(mat: *mut DenseMatrix, out: *mut *mut DenseMatrix, type_: c_int, params: SwtParams);
    pub fn ccv_swt_detect_words(mat: *mut DenseMatrix, params: SwtParams) -> *mut Array;
}