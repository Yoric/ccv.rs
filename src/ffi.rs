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
    pub interval: c_int,
    pub min_neighbors: c_int,
    pub scale_invariant: c_int,
    pub direction: c_int,
    pub same_word_thresh: [c_double; 2],

    pub size: c_int,
    pub low_thresh: c_int,
    pub high_thresh: c_int,
    pub max_height: c_int,
    pub min_height: c_int,
    pub min_area: c_int,
    pub letter_occlude_thresh: c_int,
    pub aspect_ratio: c_double,
    pub std_ratio: c_double,
    pub thickness_ratio: c_double,
    pub height_ratio: c_double,
    pub intensity_thresh: c_int,
    pub distance_ratio: c_double,
    pub intersect_ratio: c_double,
    pub elongate_ratio: c_double,
    pub letter_thresh: c_int,
    pub breakdown: c_int,
    pub breakdown_ratio: c_double,
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