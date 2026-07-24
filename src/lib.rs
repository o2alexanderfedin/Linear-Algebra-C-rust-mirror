#![allow(unused_imports, dead_code)]

mod linear_algebra_h;
mod matrix;
mod test;
mod utils;
mod vector;
use crate::linear_algebra_h::Matrix;
use crate::test::__main_inner;

pub(crate) type DarwinSizeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 { return __main_inner(); }

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn malloc(__size: u64)
    -> *mut ();
    fn free(_: *mut ())
    -> ();
    fn printf(_: *const i8, ...)
    -> i32;
    fn pow(_: f64, _: f64)
    -> f64;
    fn is_matrix_orthogonal(m1: *mut Matrix, m2: *mut Matrix)
    -> bool;
    fn rotate2_d(m: *mut Matrix, theta: f64)
    -> *mut Matrix;
    fn cos(_: f64)
    -> f64;
    fn sin(_: f64)
    -> f64;
    fn backtrace(_: *mut *mut (), __size: i32)
    -> i32;
    fn backtrace_symbols(_: *const *mut (), __size: i32)
    -> *mut *mut i8;
    fn exit(_: i32)
    -> ();
    fn round(_: f64)
    -> f64;
    fn sqrt(_: f64)
    -> f64;
}
