use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Matrix {
    pub(crate) rows: i32,
    pub(crate) cols: i32,
    pub(crate) data: *mut f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Vector {
    pub(crate) _opaque: [u8; 0],
}
