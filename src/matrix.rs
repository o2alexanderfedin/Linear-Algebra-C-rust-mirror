use super::*;
use crate::linear_algebra_h::{Matrix, Vector};
use crate::utils::{assert, exclusive_or};
use crate::vector::{
    assert_vector, delete_vector, fill_vector, new_vector, null_vector, zero_vector,
};

/// Helper function for asserting matrix and matrix data
pub(crate) extern "C" fn assert_matrix(m: *mut Matrix) -> bool {
    assert(
        (m as *mut () != 0 as *mut () && unsafe { (*m).data } as *mut () != 0 as *mut ()) as i32,
    );
    return 1;
}

/// Return new matrix with null data
pub(crate) extern "C" fn null_matrix(rows: i32, cols: i32) -> *mut Matrix {
    assert((rows > 0 && cols > 0) as i32);
    let m: *mut Matrix = unsafe { malloc(core::mem::size_of::<Matrix>() as u64) } as *mut Matrix;
    unsafe { (*m).rows = rows };
    unsafe { (*m).cols = cols };
    unsafe {
        (*m).data = unsafe {
            malloc(((rows * cols) as u64).wrapping_mul(core::mem::size_of::<f64>() as u64))
        } as *mut f64
    };
    return m;
}

/// Return new matrix from double array d with size rows x cols
pub(crate) extern "C" fn new_matrix(d: *mut f64, rows: i32, cols: i32) -> *mut Matrix {
    assert((d as *mut () != 0 as *mut () && rows > 0 && cols > 0) as i32);
    let m: *mut Matrix = null_matrix(rows, cols);
    let mut idx: i32 = 0;
    {
        let mut i: i32 = 0;
        '__b0: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b0;
            }
            '__c0: loop {
                {
                    let mut j: i32 = 0;
                    '__b1: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b1;
                        }
                        '__c1: loop {
                            unsafe {
                                *unsafe {
                                    (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                } = unsafe {
                                    *d.offset({
                                        let __p = &mut idx;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize)
                                }
                            };
                            break '__c1;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c0;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return m;
}

/// Replace all elements in matrix m with n
pub(crate) extern "C" fn fill_matrix(m: &*mut Matrix, n: f64) -> () {
    assert_matrix(*m);
    {
        let mut i: i32 = 0;
        '__b2: loop {
            if !(i < unsafe { (**m).rows }) {
                break '__b2;
            }
            '__c2: loop {
                {
                    let mut j: i32 = 0;
                    '__b3: loop {
                        if !(j < unsafe { (**m).cols }) {
                            break '__b3;
                        }
                        '__c3: loop {
                            unsafe {
                                *unsafe {
                                    (**m).data.offset((i * unsafe { (**m).cols } + j) as isize)
                                } = n
                            };
                            break '__c3;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c2;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
}

/// Return new matrix as a zero matrix of size rows x cols
pub(crate) extern "C" fn zero_matrix(rows: i32, cols: i32) -> *mut Matrix {
    let m: *mut Matrix = null_matrix(rows, cols);
    fill_matrix(&m, 0 as f64);
    return m;
}

/// Return new matrix as identifiy matrix of size n
pub(crate) extern "C" fn identity_matrix(n: i32) -> *mut Matrix {
    let m: *mut Matrix = zero_matrix(n, n);
    {
        let mut i: i32 = 0;
        '__b4: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b4;
            }
            '__c4: loop {
                {
                    let mut j: i32 = 0;
                    '__b5: loop {
                        if !(j < unsafe { (*m).rows }) {
                            break '__b5;
                        }
                        '__c5: loop {
                            if i == j {
                                unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    } = 1 as f64
                                };
                            }
                            break '__c5;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c4;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return m;
}

/// Release matrix m from memory
pub(crate) extern "C" fn delete_matrix(mut m: *mut Matrix) -> () {
    unsafe { free(unsafe { (*m).data } as *mut ()) };
    unsafe { (*m).data = 0 as *mut () as *mut f64 };
    unsafe { free(m as *mut ()) };
    m = 0 as *mut () as *mut Matrix;
}

/// Return new matrix as a copy of matrix m
pub(crate) extern "C" fn copy_matrix(m: *mut Matrix) -> *mut Matrix {
    assert_matrix(m);
    let c: *mut Matrix = zero_matrix(unsafe { (*m).rows }, unsafe { (*m).cols });
    {
        let mut i: i32 = 0;
        '__b6: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b6;
            }
            '__c6: loop {
                {
                    let mut j: i32 = 0;
                    '__b7: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b7;
                        }
                        '__c7: loop {
                            unsafe {
                                *unsafe {
                                    (*c).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                } = unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    }
                                }
                            };
                            break '__c7;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c6;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return c;
}

/// Return matrix m as flattened vector
pub(crate) extern "C" fn flatten_matrix(m: *mut Matrix) -> *mut Vector {
    assert_matrix(m);
    let flat: *mut Vector = null_vector(unsafe { (*m).rows } * unsafe { (*m).cols });
    let mut idx: i32 = 0;
    {
        let mut i: i32 = 0;
        '__b8: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b8;
            }
            '__c8: loop {
                {
                    let mut j: i32 = 0;
                    '__b9: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b9;
                        }
                        '__c9: loop {
                            unsafe {
                                *unsafe {
                                    (*flat).data.offset({
                                        let __p = &mut idx;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    }
                                        as isize)
                                } = unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    }
                                }
                            };
                            break '__c9;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c8;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return flat;
}

/// Return number of elements of matrix m
pub(crate) extern "C" fn matrix_size(m: *mut Matrix) -> i32 {
    assert_matrix(m);
    return unsafe { (*m).rows } * unsafe { (*m).cols };
}

/// Return size of matrix in bytes
pub(crate) extern "C" fn matrix_size_bytes(m: *mut Matrix) -> i32 {
    return (core::mem::size_of::<f64>() as u64).wrapping_mul(matrix_size(m) as u64) as i32;
}

/// Set element of matrix m[i,j] to scalar s
pub(crate) extern "C" fn set_matrix_element(m: *mut Matrix, i: i32, j: i32, s: f64) -> () {
    assert(
        (assert_matrix(m)
            && i >= 0
            && j >= 0
            && i < unsafe { (*m).rows }
            && j < unsafe { (*m).cols }) as i32,
    );
    unsafe { *unsafe { (*m).data.offset((i * unsafe { (*m).cols } + j) as isize) } = s };
}

/// Return scalar as element m[i,j]
pub(crate) extern "C" fn get_matrix_element(m: *mut Matrix, i: i32, j: i32) -> f64 {
    assert(
        (assert_matrix(m)
            && i >= 0
            && j >= 0
            && i < unsafe { (*m).rows }
            && j < unsafe { (*m).cols }) as i32,
    );
    return unsafe { *unsafe { (*m).data.offset((i * unsafe { (*m).cols } + j) as isize) } };
}

/// Set row vector i of matrix m to vector v
pub(crate) extern "C" fn set_row_vector(m: *mut Matrix, i: i32, v: *mut Vector) -> () {
    assert((assert_matrix(m) && assert_vector(v) && i >= 0 && i < unsafe { (*m).rows }) as i32);
    {
        let mut j: i32 = 0;
        '__b10: loop {
            if !(j < unsafe { (*v).cols }) {
                break '__b10;
            }
            '__c10: loop {
                unsafe {
                    *unsafe { (*m).data.offset((i * unsafe { (*m).cols } + j) as isize) } =
                        unsafe { *unsafe { (*v).data.offset(j as isize) } }
                };
                break '__c10;
            }
            {
                let __p = &mut j;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
}

/// Return new matrix as row vector i of vector v
pub(crate) extern "C" fn get_row_vector(m: *mut Matrix, i: i32) -> *mut Vector {
    assert((assert_matrix(m) && i >= 0 && i < unsafe { (*m).rows }) as i32);
    let row: *mut f64 = unsafe {
        malloc((core::mem::size_of::<f64>() as u64).wrapping_mul(unsafe { (*m).cols } as u64))
    } as *mut f64;
    {
        let mut j: i32 = 0;
        '__b11: loop {
            if !(j < unsafe { (*m).cols }) {
                break '__b11;
            }
            '__c11: loop {
                unsafe {
                    *row.offset(j as isize) = unsafe {
                        *unsafe { (*m).data.offset((i * unsafe { (*m).cols } + j) as isize) }
                    }
                };
                break '__c11;
            }
            {
                let __p = &mut j;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    let v: *mut Vector = new_vector(row, unsafe { (*m).cols });
    unsafe { free(row as *mut ()) };
    return v;
}

/// Set col vector j of matrix m to vector v
pub(crate) extern "C" fn set_col_vector(m: *mut Matrix, j: i32, v: *mut Vector) -> () {
    assert((assert_matrix(m) && assert_vector(v) && j >= 0 && j < unsafe { (*m).cols }) as i32);
    {
        let mut i: i32 = 0;
        '__b12: loop {
            if !(i < unsafe { (*v).cols }) {
                break '__b12;
            }
            '__c12: loop {
                unsafe {
                    *unsafe { (*m).data.offset((i * unsafe { (*m).cols } + j) as isize) } =
                        unsafe { *unsafe { (*v).data.offset(i as isize) } }
                };
                break '__c12;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
}

/// Return new matrix as col vector j of vector v
pub(crate) extern "C" fn get_col_vector(m: *mut Matrix, j: i32) -> *mut Vector {
    assert((assert_matrix(m) && j >= 0 && j < unsafe { (*m).cols }) as i32);
    let col: *mut f64 = unsafe {
        malloc((core::mem::size_of::<f64>() as u64).wrapping_mul(unsafe { (*m).rows } as u64))
    } as *mut f64;
    {
        let mut i: i32 = 0;
        '__b13: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b13;
            }
            '__c13: loop {
                unsafe {
                    *col.offset(i as isize) = unsafe {
                        *unsafe { (*m).data.offset((i * unsafe { (*m).cols } + j) as isize) }
                    }
                };
                break '__c13;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    let v: *mut Vector = new_vector(col, unsafe { (*m).rows });
    unsafe { free(col as *mut ()) };
    return v;
}

/// Matrix m is a square matrix if number of cols = number of rows
pub(crate) extern "C" fn is_square_matrix(m: *mut Matrix) -> bool {
    assert_matrix(m);
    return unsafe { (*m).rows } == unsafe { (*m).cols };
}

/// Return new vector as main diagonal of matrix m (square matrices only)
pub(crate) extern "C" fn get_main_diagonal(m: *mut Matrix) -> *mut Vector {
    assert(is_square_matrix(m) as i32);
    let diag: *mut f64 = unsafe {
        malloc((core::mem::size_of::<f64>() as u64).wrapping_mul(unsafe { (*m).rows } as u64))
    } as *mut f64;
    {
        let mut x: i32 = 0;
        '__b14: loop {
            if !(x < unsafe { (*m).rows }) {
                break '__b14;
            }
            '__c14: loop {
                unsafe {
                    *diag.offset(x as isize) = unsafe {
                        *unsafe { (*m).data.offset((x * unsafe { (*m).cols } + x) as isize) }
                    }
                };
                break '__c14;
            }
            {
                let __p = &mut x;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    let v: *mut Vector = new_vector(diag, unsafe { (*m).rows });
    unsafe { free(diag as *mut ()) };
    return v;
}

/// Set main diagonal of matrix m to vector v
pub(crate) extern "C" fn set_main_diagonal(m: *mut Matrix, v: *mut Vector) -> () {
    assert(
        (is_square_matrix(m)
            && assert_vector(v)
            && unsafe { (*m).rows } == unsafe { (*m).cols }
            && unsafe { (*m).cols } == unsafe { (*v).cols }) as i32,
    );
    {
        let mut x: i32 = 0;
        '__b15: loop {
            if !(x < unsafe { (*v).cols }) {
                break '__b15;
            }
            '__c15: loop {
                unsafe {
                    *unsafe { (*m).data.offset((x * unsafe { (*m).cols } + x) as isize) } =
                        unsafe { *unsafe { (*v).data.offset(x as isize) } }
                };
                break '__c15;
            }
            {
                let __p = &mut x;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
}

/// Return new vector as anti diagonal of matrix m (square matrices only)
pub(crate) extern "C" fn get_anti_diagonal(m: *mut Matrix) -> *mut Vector {
    assert(is_square_matrix(m) as i32);
    let mut x: i32 = 0;
    let diag: *mut f64 = unsafe {
        malloc((core::mem::size_of::<f64>() as u64).wrapping_mul(unsafe { (*m).rows } as u64))
    } as *mut f64;
    {
        let mut i: i32 = unsafe { (*m).rows } - 1;
        '__b16: loop {
            if !(i >= 0) {
                break '__b16;
            }
            '__c16: loop {
                {
                    let mut j: i32 = unsafe { (*m).cols } - 1;
                    '__b17: loop {
                        if !(j >= 0) {
                            break '__b17;
                        }
                        '__c17: loop {
                            if i + j == unsafe { (*m).rows } - 1 {
                                unsafe {
                                    *diag.offset({
                                        let __p = &mut x;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = unsafe {
                                        *unsafe {
                                            (*m).data
                                                .offset((i * unsafe { (*m).cols } + j) as isize)
                                        }
                                    }
                                };
                                if x == unsafe { (*m).rows } {
                                    break '__b17;
                                }
                            }
                            break '__c17;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                    }
                }
                break '__c16;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p -= 1;
                __t
            };
        }
    }
    let v: *mut Vector = new_vector(diag, unsafe { (*m).rows });
    unsafe { free(diag as *mut ()) };
    return v;
}

/// Set anti diagonal of matrix m to vector v
pub(crate) extern "C" fn set_anti_diagonal(m: *mut Matrix, v: *mut Vector) -> () {
    assert(
        (is_square_matrix(m)
            && assert_vector(v)
            && unsafe { (*m).rows } == unsafe { (*m).cols }
            && unsafe { (*m).cols } == unsafe { (*v).cols }) as i32,
    );
    let mut idx: i32 = 0;
    {
        let mut i: i32 = unsafe { (*m).rows } - 1;
        '__b18: loop {
            if !(i >= 0) {
                break '__b18;
            }
            '__c18: loop {
                {
                    let mut j: i32 = unsafe { (*m).cols } - 1;
                    '__b19: loop {
                        if !(j >= 0) {
                            break '__b19;
                        }
                        '__c19: loop {
                            if i + j == unsafe { (*m).rows } - 1 {
                                unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    } = unsafe {
                                        *unsafe {
                                            (*v).data.offset({
                                                let __p = &mut idx;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            }
                                                as isize)
                                        }
                                    }
                                };
                                if idx == unsafe { (*m).rows } {
                                    break '__b19;
                                }
                            }
                            break '__c19;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                    }
                }
                break '__c18;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p -= 1;
                __t
            };
        }
    }
}

/// Return scalar as product of elements of main diagonal of matrix m
pub(crate) extern "C" fn diagonal_product(m: *mut Matrix) -> f64 {
    let diagonal: *mut Vector = get_main_diagonal(m);
    let mut product: f64 = 1.0;
    {
        let mut i: i32 = 0;
        '__b20: loop {
            if !(i < unsafe { (*diagonal).cols }) {
                break '__b20;
            }
            '__c20: loop {
                product *= unsafe { *unsafe { (*diagonal).data.offset(i as isize) } };
                break '__c20;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    delete_vector(diagonal);
    return product;
}

/// "Pretty" print matrix m
pub(crate) extern "C" fn print_matrix(m: *mut Matrix, include_indices: bool) -> () {
    assert_matrix(m);
    {
        let mut i: i32 = 0;
        '__b21: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b21;
            }
            '__c21: loop {
                {
                    let mut j: i32 = 0;
                    '__b22: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b22;
                        }
                        '__c22: loop {
                            if include_indices {
                                unsafe {
                                    printf(c"[%d,%d] -> ".as_ptr() as *mut i8 as *const i8, i, j)
                                };
                            }
                            unsafe {
                                printf(c"%8.2f ".as_ptr() as *mut i8 as *const i8, unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    }
                                })
                            };
                            break '__c22;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                if i < unsafe { (*m).rows } {
                    unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                }
                break '__c21;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
}

/// Matrices m and n are equal if same dimension and identical elements
pub(crate) extern "C" fn is_matrix_equal(m: *mut Matrix, n: *mut Matrix) -> bool {
    assert((assert_matrix(m) && assert_matrix(n)) as i32);
    if unsafe { (*m).rows } != unsafe { (*n).rows } || unsafe { (*m).cols } != unsafe { (*n).cols }
    {
        return 0;
    }
    {
        let mut i: i32 = 0;
        '__b23: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b23;
            }
            '__c23: loop {
                {
                    let mut j: i32 = 0;
                    '__b24: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b24;
                        }
                        '__c24: loop {
                            if unsafe {
                                *unsafe {
                                    (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                }
                            } != unsafe {
                                *unsafe {
                                    (*n).data.offset((i * unsafe { (*n).cols } + j) as isize)
                                }
                            } {
                                return 0;
                            }
                            break '__c24;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c23;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 1;
}

/// Matrices m and n have same dimensions if their columns and rows are equal
pub(crate) extern "C" fn has_same_dimensions(m: *mut Matrix, n: *mut Matrix) -> bool {
    assert((assert_matrix(m) && assert_matrix(n)) as i32);
    return unsafe { (*m).rows } == unsafe { (*n).rows }
        && unsafe { (*m).cols } == unsafe { (*n).cols };
}

/// Matrix m is a zero matrix if all elements are 0
pub(crate) extern "C" fn is_zero_matrix(m: *mut Matrix) -> bool {
    assert_matrix(m);
    {
        let mut i: i32 = 0;
        '__b25: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b25;
            }
            '__c25: loop {
                {
                    let mut j: i32 = 0;
                    '__b26: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b26;
                        }
                        '__c26: loop {
                            if unsafe {
                                *unsafe {
                                    (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                }
                            } != 0 as f64
                            {
                                return 0;
                            }
                            break '__c26;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c25;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 1;
}

/// Matrix m is an identity matrix if it is a square matrix with only 1's along main diagonal
pub(crate) extern "C" fn is_identity_matrix(m: *mut Matrix) -> bool {
    if !is_square_matrix(m) as i32 != 0 {
        return 0;
    }
    {
        let mut i: i32 = 0;
        '__b27: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b27;
            }
            '__c27: loop {
                {
                    let mut j: i32 = 0;
                    '__b28: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b28;
                        }
                        '__c28: loop {
                            if i == j
                                && unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    }
                                } != 1.0
                            {
                                return 0;
                            } else if i != j
                                && unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    }
                                } != 0.0
                            {
                                return 0;
                            }
                            break '__c28;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c27;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 1;
}

/// Matrix m is an upper triangular matrix if m is a square matrix and all elements below diagonal are zero
pub(crate) extern "C" fn is_up_tri_matrix(m: *mut Matrix) -> bool {
    assert(is_square_matrix(m) as i32);
    {
        let mut i: i32 = 0;
        '__b29: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b29;
            }
            '__c29: loop {
                {
                    let mut j: i32 = 0;
                    '__b30: loop {
                        if !(j < i) {
                            break '__b30;
                        }
                        '__c30: loop {
                            if unsafe {
                                *unsafe {
                                    (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                }
                            } != 0 as f64
                            {
                                return 0;
                            }
                            break '__c30;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c29;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 1;
}

/// Matrix m is a lower triangular matrix if m is a square matrix and all elements above diagonal are zero
pub(crate) extern "C" fn is_lo_tri_matrix(m: *mut Matrix) -> bool {
    assert(is_square_matrix(m) as i32);
    {
        let mut i: i32 = 0;
        '__b31: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b31;
            }
            '__c31: loop {
                {
                    let mut j: i32 = i + 1;
                    '__b32: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b32;
                        }
                        '__c32: loop {
                            if unsafe {
                                *unsafe {
                                    (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                }
                            } != 0 as f64
                            {
                                return 0;
                            }
                            break '__c32;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c31;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 1;
}

/// Matrix m is a triangular matrix if m is an upper triangular or lower triangular matrix
pub(crate) extern "C" fn is_triangular_matrix(m: *mut Matrix) -> bool {
    assert(is_square_matrix(m) as i32);
    return exclusive_or(is_up_tri_matrix(m), is_lo_tri_matrix(m));
}

/// Return new matrix as pivot matrix of matrix m
pub(crate) extern "C" fn pivot_matrix(m: *mut Matrix, mut swaps: *mut i32) -> *mut Matrix {
    assert(is_square_matrix(m) as i32);
    let n: i32 = unsafe { (*m).cols };
    let pivot: *mut Matrix = identity_matrix(n);
    {
        let mut i: i32 = 0;
        '__b33: loop {
            if !(i < n) {
                break '__b33;
            }
            '__c33: loop {
                let mut max: f64 = unsafe { *unsafe { (*m).data.offset((i * n + i) as isize) } };
                let mut row: i32 = i;
                {
                    let mut j: i32 = i;
                    '__b34: loop {
                        if !(j < n) {
                            break '__b34;
                        }
                        '__c34: loop {
                            if unsafe { *unsafe { (*m).data.offset((j * n + i) as isize) } } > max {
                                max = unsafe { *unsafe { (*m).data.offset((j * n + i) as isize) } };
                                row = j;
                            }
                            break '__c34;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                if i != row {
                    let v: *mut Vector = get_row_vector(pivot, i);
                    let w: *mut Vector = get_row_vector(pivot, row);
                    set_row_vector(pivot, i, w);
                    set_row_vector(pivot, row, v);
                    delete_vector(w);
                    delete_vector(v);
                    {
                        let __p = &mut swaps;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                break '__c33;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return pivot;
}

/// Return new matrix as product of matrices m1 and m2
pub(crate) extern "C" fn multiply_matrices(m: *mut Matrix, n: *mut Matrix) -> *mut Matrix {
    assert(
        (assert_matrix(m)
            && assert_matrix(n)
            && unsafe { (*m).cols } == unsafe { (*n).cols }
            && unsafe { (*m).rows } == unsafe { (*n).rows }) as i32,
    );
    let prod: *mut Matrix = null_matrix(unsafe { (*m).rows }, unsafe { (*m).cols });
    {
        let mut j: i32 = 0;
        '__b35: loop {
            if !(j < unsafe { (*m).rows }) {
                break '__b35;
            }
            '__c35: loop {
                {
                    let mut i: i32 = 0;
                    '__b36: loop {
                        if !(i < unsafe { (*m).cols }) {
                            break '__b36;
                        }
                        '__c36: loop {
                            let mut val: f64 = 0.0;
                            {
                                let mut k: i32 = 0;
                                '__b37: loop {
                                    if !(k < unsafe { (*m).cols }) {
                                        break '__b37;
                                    }
                                    '__c37: loop {
                                        val += unsafe {
                                            *unsafe {
                                                (*m).data
                                                    .offset((i * unsafe { (*m).cols } + k) as isize)
                                            }
                                        } * unsafe {
                                            *unsafe {
                                                (*n).data
                                                    .offset((k * unsafe { (*m).cols } + j) as isize)
                                            }
                                        };
                                        break '__c37;
                                    }
                                    {
                                        let __p = &mut k;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                }
                            }
                            unsafe {
                                *unsafe {
                                    (*prod).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                } = val
                            };
                            break '__c36;
                        }
                        {
                            let __p = &mut i;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c35;
            }
            {
                let __p = &mut j;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return prod;
}

/// Decompose matrix m into matrices l (lower triangular), u (upper triangular), p (permutation)
pub(crate) extern "C" fn lu_decomposition(
    m: *mut Matrix,
    l: &mut *mut Matrix,
    u: &mut *mut Matrix,
    p: &mut *mut Matrix,
) -> i32 {
    assert(
        (is_square_matrix(m)
            && *l as *mut () == 0 as *mut ()
            && *u as *mut () == 0 as *mut ()
            && *p as *mut () == 0 as *mut ()) as i32,
    );
    let n: i32 = unsafe { (*m).cols };
    let mut swaps: i32 = 0;
    *l = zero_matrix(n, n);
    *u = zero_matrix(n, n);
    *p = pivot_matrix(m, &mut swaps);
    let m2: *mut Matrix = multiply_matrices(*p, m);
    {
        let mut j: i32 = 0;
        '__b38: loop {
            if !(j < n) {
                break '__b38;
            }
            '__c38: loop {
                unsafe { *unsafe { (**l).data.offset((j * n + j) as isize) } = 1 as f64 };
                {
                    let mut i: i32 = 0;
                    '__b39: loop {
                        if !(i < j + 1) {
                            break '__b39;
                        }
                        '__c39: loop {
                            let mut sum_u: f64 = 0 as f64;
                            {
                                let mut k: i32 = 0;
                                '__b40: loop {
                                    if !(k < i) {
                                        break '__b40;
                                    }
                                    '__c40: loop {
                                        sum_u += unsafe {
                                            *unsafe { (**u).data.offset((k * n + j) as isize) }
                                        } * unsafe {
                                            *unsafe { (**l).data.offset((i * n + k) as isize) }
                                        };
                                        break '__c40;
                                    }
                                    {
                                        let __p = &mut k;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                }
                            }
                            unsafe {
                                *unsafe { (**u).data.offset((i * n + j) as isize) } =
                                    unsafe { *unsafe { (*m2).data.offset((i * n + j) as isize) } }
                                        - sum_u
                            };
                            break '__c39;
                        }
                        {
                            let __p = &mut i;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                {
                    let mut i: i32 = j;
                    '__b41: loop {
                        if !(i < n) {
                            break '__b41;
                        }
                        '__c41: loop {
                            let mut sum_l: f64 = 0 as f64;
                            {
                                let mut k: i32 = 0;
                                '__b42: loop {
                                    if !(k < j) {
                                        break '__b42;
                                    }
                                    '__c42: loop {
                                        sum_l += unsafe {
                                            *unsafe { (**u).data.offset((k * n + j) as isize) }
                                        } * unsafe {
                                            *unsafe { (**l).data.offset((i * n + k) as isize) }
                                        };
                                        break '__c42;
                                    }
                                    {
                                        let __p = &mut k;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                }
                            }
                            unsafe {
                                *unsafe { (**l).data.offset((i * n + j) as isize) } =
                                    (unsafe { *unsafe { (*m2).data.offset((i * n + j) as isize) } }
                                        - sum_l)
                                        / unsafe {
                                            *unsafe { (**u).data.offset((j * n + j) as isize) }
                                        }
                            };
                            break '__c41;
                        }
                        {
                            let __p = &mut i;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c38;
            }
            {
                let __p = &mut j;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    delete_matrix(m2);
    return swaps;
}

/// Return scalar as determinant of matrix m
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn determinant(m: *mut Matrix) -> f64 {
    assert(is_square_matrix(m) as i32);
    '__s43: {
        match unsafe { (*m).rows } {
            1 => {
                return unsafe { *unsafe { (*m).data.offset(0 as isize) } };
            }
            2 => {
                return unsafe { *unsafe { (*m).data.offset(0 as isize) } }
                    * unsafe { *unsafe { (*m).data.offset(3 as isize) } }
                    - unsafe { *unsafe { (*m).data.offset(1 as isize) } }
                        * unsafe { *unsafe { (*m).data.offset(2 as isize) } };
            }
            3 => {
                return unsafe { *unsafe { (*m).data.offset(0 as isize) } }
                    * (unsafe { *unsafe { (*m).data.offset(4 as isize) } }
                        * unsafe { *unsafe { (*m).data.offset(8 as isize) } }
                        - unsafe { *unsafe { (*m).data.offset(5 as isize) } }
                            * unsafe { *unsafe { (*m).data.offset(7 as isize) } })
                    - unsafe { *unsafe { (*m).data.offset(1 as isize) } }
                        * (unsafe { *unsafe { (*m).data.offset(3 as isize) } }
                            * unsafe { *unsafe { (*m).data.offset(8 as isize) } }
                            - unsafe { *unsafe { (*m).data.offset(5 as isize) } }
                                * unsafe { *unsafe { (*m).data.offset(6 as isize) } })
                    + unsafe { *unsafe { (*m).data.offset(2 as isize) } }
                        * (unsafe { *unsafe { (*m).data.offset(3 as isize) } }
                            * unsafe { *unsafe { (*m).data.offset(7 as isize) } }
                            - unsafe { *unsafe { (*m).data.offset(4 as isize) } }
                                * unsafe { *unsafe { (*m).data.offset(6 as isize) } });
            }
            _ => {}
        }
    }
    if is_triangular_matrix(m) {
        return diagonal_product(m);
    }
    let mut l: *mut Matrix = 0 as *mut () as *mut Matrix;
    let mut u: *mut Matrix = 0 as *mut () as *mut Matrix;
    let mut p: *mut Matrix = 0 as *mut () as *mut Matrix;
    /// det(permutation matrix) = (-1)^swaps
    let det: f64 = unsafe {
        pow(
            -1 as f64,
            (lu_decomposition(m, &mut l, &mut u, &mut p) - 1) as f64,
        )
    } * determinant(l)
        * determinant(u);
    delete_matrix(p);
    delete_matrix(u);
    delete_matrix(l);
    return det;
}

/// Matrix m is invertible if it is a square matrix and det(m) != 0
pub(crate) extern "C" fn is_invertible(m: *mut Matrix) -> bool {
    return is_square_matrix(m) && determinant(m) != 0 as f64;
}

/// Matrix m is a diagonal matrix if m is a square matrix and all elements not along diagonal are zero
pub(crate) extern "C" fn is_diagonal_matrix(m: *mut Matrix) -> bool {
    assert(is_square_matrix(m) as i32);
    {
        let mut i: i32 = 0;
        '__b44: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b44;
            }
            '__c44: loop {
                {
                    let mut j: i32 = 0;
                    '__b45: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b45;
                        }
                        '__c45: loop {
                            if i != j
                                && unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    }
                                } != 0 as f64
                            {
                                return 0;
                            }
                            break '__c45;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c44;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 1;
}

/// Return new matrix as transpose of matrix m - flip matrix along diagonal
pub(crate) extern "C" fn transpose_matrix(m: *mut Matrix) -> *mut Matrix {
    assert_matrix(m);
    let t: *mut Matrix = zero_matrix(unsafe { (*m).cols }, unsafe { (*m).rows });
    {
        let mut i: i32 = 0;
        '__b46: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b46;
            }
            '__c46: loop {
                {
                    let mut j: i32 = 0;
                    '__b47: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b47;
                        }
                        '__c47: loop {
                            unsafe {
                                *unsafe {
                                    (*t).data.offset((j * unsafe { (*t).cols } + i) as isize)
                                } = unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    }
                                }
                            };
                            break '__c47;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c46;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return t;
}

/// Matrix m is a symmetric matrix if m = transpose(m)
pub(crate) extern "C" fn is_matrix_symmetric(m: *mut Matrix) -> bool {
    let t: *mut Matrix = transpose_matrix(m);
    let equal: bool = is_matrix_equal(m, t);
    delete_matrix(t);
    return equal;
}

/// Matrix m has a zero row if any row is made entirely of zeroes
pub(crate) extern "C" fn has_zero_row(m: *mut Matrix) -> bool {
    assert_matrix(m);
    let mut all_zeroes: bool = 1;
    {
        let mut i: i32 = 0;
        '__b48: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b48;
            }
            '__c48: loop {
                {
                    let mut j: i32 = 0;
                    '__b49: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b49;
                        }
                        '__c49: loop {
                            if unsafe {
                                *unsafe {
                                    (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                }
                            } != 0 as f64
                            {
                                all_zeroes = 0;
                            }
                            break '__c49;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                if all_zeroes {
                    return 1;
                }
                all_zeroes = 1;
                break '__c48;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 0;
}

/// Matrix m has a zero col if any col is made entirely of zeroes
pub(crate) extern "C" fn has_zero_col(m: *mut Matrix) -> bool {
    assert_matrix(m);
    let mut all_zeroes: bool = 1;
    {
        let mut j: i32 = 0;
        '__b50: loop {
            if !(j < unsafe { (*m).rows }) {
                break '__b50;
            }
            '__c50: loop {
                {
                    let mut i: i32 = 0;
                    '__b51: loop {
                        if !(i < unsafe { (*m).cols }) {
                            break '__b51;
                        }
                        '__c51: loop {
                            if unsafe {
                                *unsafe {
                                    (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                }
                            } != 0 as f64
                            {
                                all_zeroes = 0;
                            }
                            break '__c51;
                        }
                        {
                            let __p = &mut i;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                if all_zeroes {
                    return 1;
                }
                all_zeroes = 1;
                break '__c50;
            }
            {
                let __p = &mut j;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 0;
}

/// Return scalar as trace of matrix m - sum of all diagonals
pub(crate) extern "C" fn trace_matrix(m: *mut Matrix) -> f64 {
    assert(is_square_matrix(m) as i32);
    let mut trace: f64 = 0 as f64;
    {
        let mut i: i32 = 0;
        '__b52: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b52;
            }
            '__c52: loop {
                trace += unsafe {
                    *unsafe { (*m).data.offset((i * unsafe { (*m).cols } + i) as isize) }
                };
                break '__c52;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return trace;
}

/// Return new matrix as sum of matrices m1 and m2
pub(crate) extern "C" fn add_matrices(m: *mut Matrix, n: *mut Matrix) -> *mut Matrix {
    assert(has_same_dimensions(m, n) as i32);
    let sum: *mut Matrix = null_matrix(unsafe { (*m).rows }, unsafe { (*m).cols });
    let mut idx: i32 = 0;
    {
        let mut i: i32 = 0;
        '__b53: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b53;
            }
            '__c53: loop {
                {
                    let mut j: i32 = 0;
                    '__b54: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b54;
                        }
                        '__c54: loop {
                            unsafe {
                                *unsafe { (*sum).data.offset(idx as isize) } =
                                    unsafe { *unsafe { (*m).data.offset(idx as isize) } }
                                        + unsafe { *unsafe { (*n).data.offset(idx as isize) } }
                            };
                            {
                                let __p = &mut idx;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            break '__c54;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c53;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return sum;
}

/// Return new matrix as matrix m ^ k
pub(crate) extern "C" fn pow_matrix(m: *mut Matrix, k: f64) -> *mut Matrix {
    assert_matrix(m);
    let p: *mut Matrix = null_matrix(unsafe { (*m).rows }, unsafe { (*m).cols });
    {
        let mut i: i32 = 0;
        '__b55: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b55;
            }
            '__c55: loop {
                {
                    let mut j: i32 = 0;
                    '__b56: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b56;
                        }
                        '__c56: loop {
                            unsafe {
                                *unsafe {
                                    (*p).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                } = unsafe {
                                    pow(
                                        unsafe {
                                            *unsafe {
                                                (*m).data
                                                    .offset((i * unsafe { (*m).cols } + j) as isize)
                                            }
                                        },
                                        k,
                                    )
                                }
                            };
                            break '__c56;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c55;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return p;
}

/// Return new matrix as matrix m scaled by scalar s
pub(crate) extern "C" fn scale_matrix(m: *mut Matrix, s: f64) -> *mut Matrix {
    assert_matrix(m);
    let scaled: *mut Matrix = null_matrix(unsafe { (*m).rows }, unsafe { (*m).cols });
    {
        let mut i: i32 = 0;
        '__b57: loop {
            if !(i < unsafe { (*m).rows }) {
                break '__b57;
            }
            '__c57: loop {
                {
                    let mut j: i32 = 0;
                    '__b58: loop {
                        if !(j < unsafe { (*m).cols }) {
                            break '__b58;
                        }
                        '__c58: loop {
                            unsafe {
                                *unsafe {
                                    (*scaled)
                                        .data
                                        .offset((i * unsafe { (*m).cols } + j) as isize)
                                } = unsafe {
                                    *unsafe {
                                        (*m).data.offset((i * unsafe { (*m).cols } + j) as isize)
                                    }
                                } * s
                            };
                            break '__c58;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c57;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return scaled;
}

/// Return new 2D matrix as reflection about x or y axis [x:0,y:1]
pub(crate) extern "C" fn reflect_axis2_d(m: *mut Matrix, axis: i32) -> *mut Matrix {
    assert((is_square_matrix(m) && unsafe { (*m).cols } == 2) as i32);
    let n: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(n, 0, 0, if axis != 0 { 1 } else { -1 } as f64);
    set_matrix_element(n, 1, 1, if axis != 0 { -1 } else { 1 } as f64);
    let ref_: *mut Matrix = multiply_matrices(m, n);
    delete_matrix(n);
    return ref_;
}

/// Return new 3D matrix as reflection about xy,xz,yz plane [xy:0,xz:1,yz:2]
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn reflect_axis3_d(m: *mut Matrix, axis: i32) -> *mut Matrix {
    assert((is_square_matrix(m) && unsafe { (*m).cols } == 3 && axis >= 0 && axis <= 2) as i32);
    let n: *mut Matrix = null_matrix(3, 3);
    if axis == 0 {
        let mut data: [f64; 9] = [
            1 as f64, 0 as f64, 0 as f64, 0 as f64, 1 as f64, 0 as f64, 0 as f64, 0 as f64,
            -1 as f64,
        ];

        /// XY
        set_main_diagonal(m, new_vector(&raw mut data[0 as usize] as *mut f64, 3));
    } else if axis == 1 {
        let mut data: [f64; 9] = [
            1 as f64, 0 as f64, 0 as f64, 0 as f64, -1 as f64, 0 as f64, 0 as f64, 0 as f64,
            1 as f64,
        ];

        /// XZ
        set_main_diagonal(m, new_vector(&raw mut data[0 as usize] as *mut f64, 3));
    } else {
        let mut data: [f64; 9] = [
            -1 as f64, 0 as f64, 0 as f64, 0 as f64, 1 as f64, 0 as f64, 0 as f64, 0 as f64,
            1 as f64,
        ];

        /// YZ
        set_main_diagonal(m, new_vector(&raw mut data[0 as usize] as *mut f64, 3));
    }
    let ref_: *mut Matrix = multiply_matrices(m, n);
    delete_matrix(n);
    return ref_;
}

/// Return new 2D matrix as orthogonal projection on x or y axis [x:0,y:1]
pub(crate) extern "C" fn orth_proj2_d(m: *mut Matrix, axis: i32) -> *mut Matrix {
    assert((is_square_matrix(m) && unsafe { (*m).cols } == 2) as i32);
    let n: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(n, axis, axis, 1 as f64);
    let ref_: *mut Matrix = multiply_matrices(m, n);
    delete_matrix(n);
    return ref_;
}

/// Return new 3D matrix as orthogonal projection on xy,xz,yz plane [xy:0,xz:1,yz:2]
pub(crate) extern "C" fn orth_proj3_d(m: *mut Matrix, axis: i32) -> *mut Matrix {
    assert((is_square_matrix(m) && unsafe { (*m).cols } == 3) as i32);
    let n: *mut Matrix = zero_matrix(3, 3);
    '__s59: {
        match axis {
            0 => {
                set_matrix_element(n, 0, 0, 1 as f64);
                set_matrix_element(n, 1, 1, 1 as f64);
            }
            1 => {
                set_matrix_element(n, 0, 0, 1 as f64);
                set_matrix_element(n, 2, 2, 1 as f64);
            }
            2 => {
                set_matrix_element(n, 1, 1, 1 as f64);
                set_matrix_element(n, 2, 2, 1 as f64);
            }
            _ => {}
        }
    }
    let ref_: *mut Matrix = multiply_matrices(m, n);
    delete_matrix(n);
    return ref_;
}

/// Return new nxn matrix as contraction or dilation of factor k on n-space
pub(crate) extern "C" fn scale_n_space(m: *mut Matrix, k: f64) -> *mut Matrix {
    assert(is_square_matrix(m) as i32);
    let n: *mut Matrix = zero_matrix(unsafe { (*m).cols }, unsafe { (*m).cols });
    let v: *mut Vector = zero_vector(unsafe { (*m).cols });
    fill_vector(&v, k);
    set_main_diagonal(n, v);
    let ref_: *mut Matrix = multiply_matrices(m, n);
    delete_vector(v);
    delete_matrix(n);
    return ref_;
}

/// Return new 2D matrix as shear of 2-space in x or y with factor k
pub(crate) extern "C" fn shear2_d(m: *mut Matrix, k: f64, axis: i32) -> *mut Matrix {
    assert((is_square_matrix(m) && unsafe { (*m).cols } == 2) as i32);
    let n: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(n, 0, 0, 1 as f64);
    set_matrix_element(n, 0, 1, if axis != 0 { k } else { 0 as f64 });
    set_matrix_element(n, 1, 0, if axis != 0 { 0 as f64 } else { k });
    set_matrix_element(n, 1, 1, 1 as f64);
    let sheared: *mut Matrix = multiply_matrices(m, n);
    delete_matrix(n);
    return sheared;
}

/// Return new matrix as submatrix of matrix m, excluding row i and col i
pub(crate) extern "C" fn sub_matrix(m: *mut Matrix, i: i32, j: i32) -> *mut Matrix {
    assert(
        (assert_matrix(m)
            && i >= 0
            && i < unsafe { (*m).rows }
            && j >= 0
            && j < unsafe { (*m).cols }) as i32,
    );
    let sm: *mut Matrix = null_matrix(unsafe { (*m).rows } - 1, unsafe { (*m).cols } - 1);
    let mut idx: i32 = 0;
    {
        let mut row: i32 = 0;
        '__b60: loop {
            if !(row < unsafe { (*m).rows }) {
                break '__b60;
            }
            '__c60: loop {
                {
                    let mut col: i32 = 0;
                    '__b61: loop {
                        if !(col < unsafe { (*m).cols }) {
                            break '__b61;
                        }
                        '__c61: loop {
                            if row != i && col != j {
                                unsafe {
                                    *unsafe {
                                        (*sm).data.offset({
                                            let __p = &mut idx;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        }
                                            as isize)
                                    } = unsafe {
                                        *unsafe {
                                            (*m).data
                                                .offset((row * unsafe { (*m).cols } + col) as isize)
                                        }
                                    }
                                };
                            }
                            break '__c61;
                        }
                        {
                            let __p = &mut col;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c60;
            }
            {
                let __p = &mut row;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return sm;
}

/// Return scalar as minor of matrix m at row i and column j
pub(crate) extern "C" fn element_minor(m: *mut Matrix, i: i32, j: i32) -> f64 {
    let sm: *mut Matrix = sub_matrix(m, i, j);
    let minor: f64 = determinant(sm);
    delete_matrix(sm);
    return minor;
}

/// Return new matrix as matrix of minors of matrix m
pub(crate) extern "C" fn matrix_minor(m: *mut Matrix) -> *mut Matrix {
    assert_matrix(m);
    let mm: *mut Matrix = null_matrix(unsafe { (*m).rows }, unsafe { (*m).cols });
    {
        let mut i: i32 = 0;
        '__b62: loop {
            if !(i < unsafe { (*mm).rows }) {
                break '__b62;
            }
            '__c62: loop {
                {
                    let mut j: i32 = 0;
                    '__b63: loop {
                        if !(j < unsafe { (*mm).cols }) {
                            break '__b63;
                        }
                        '__c63: loop {
                            unsafe {
                                *unsafe {
                                    (*mm).data.offset((i * unsafe { (*mm).cols } + j) as isize)
                                } = element_minor(m, i, j)
                            };
                            break '__c63;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c62;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return mm;
}

/// Return scalar as cofactor of matrix m at row i and column j
pub(crate) extern "C" fn element_cofactor(m: *mut Matrix, i: i32, j: i32) -> f64 {
    return unsafe { pow(-1 as f64, (i + 1 + (j + 1)) as f64) } * element_minor(m, i, j);
}

/// Return new matrix as cofactor matrix of matrix m
pub(crate) extern "C" fn matrix_cofactor(m: *mut Matrix) -> *mut Matrix {
    assert_matrix(m);
    let cfm: *mut Matrix = null_matrix(unsafe { (*m).rows }, unsafe { (*m).cols });
    {
        let mut i: i32 = 0;
        '__b64: loop {
            if !(i < unsafe { (*cfm).rows }) {
                break '__b64;
            }
            '__c64: loop {
                {
                    let mut j: i32 = 0;
                    '__b65: loop {
                        if !(j < unsafe { (*cfm).cols }) {
                            break '__b65;
                        }
                        '__c65: loop {
                            unsafe {
                                *unsafe {
                                    (*cfm)
                                        .data
                                        .offset((i * unsafe { (*cfm).cols } + j) as isize)
                                } = element_cofactor(m, i, j)
                            };
                            break '__c65;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c64;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return cfm;
}

/// Return new matrix with elements alternating sign + -
pub(crate) extern "C" fn sign_matrix(rows: i32, cols: i32) -> *mut Matrix {
    assert((rows > 0 && cols > 0) as i32);
    let sm: *mut Matrix = null_matrix(rows, cols);
    fill_matrix(&sm, 1 as f64);
    {
        let mut i: i32 = 0;
        '__b66: loop {
            if !(i < unsafe { (*sm).rows }) {
                break '__b66;
            }
            '__c66: loop {
                {
                    let mut j: i32 = 0;
                    '__b67: loop {
                        if !(j < unsafe { (*sm).cols }) {
                            break '__b67;
                        }
                        '__c67: loop {
                            unsafe {
                                *unsafe {
                                    (*sm).data.offset((i * unsafe { (*sm).cols } + j) as isize)
                                } = if (i * unsafe { (*sm).cols } + j + 1) % 2 != 0 {
                                    1
                                } else {
                                    -1
                                } as f64
                            };
                            break '__c67;
                        }
                        {
                            let __p = &mut j;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                }
                break '__c66;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return sm;
}

/// Return new matrix as adjugate matrix of matrix m
pub(crate) extern "C" fn adjugate_matrix(m: *mut Matrix) -> *mut Matrix {
    assert_matrix(m);
    let mm: *mut Matrix = matrix_minor(m);
    let sign: *mut Matrix = sign_matrix(unsafe { (*m).rows }, unsafe { (*m).cols });
    let adj: *mut Matrix = multiply_matrices(mm, sign);
    delete_matrix(sign);
    delete_matrix(mm);
    return adj;
}

/// Return new matrix as inversion of matrix m
pub(crate) extern "C" fn inverse_matrix(m: *mut Matrix) -> *mut Matrix {
    assert(is_invertible(m) as i32);
    let adj: *mut Matrix = adjugate_matrix(m);
    let inv: *mut Matrix = scale_matrix(adj, 1.0 / determinant(m));
    delete_matrix(adj);
    return inv;
}

pub(crate) extern "C" fn rotate3_d(m: *mut Matrix, theta: f64, axis: i32) -> *mut Matrix {
    assert((is_square_matrix(m) && unsafe { (*m).cols } == 3) as i32);
    let n: *mut Matrix = zero_matrix(3, 3);
    '__s68: {
        match axis {
            0 => {
                set_matrix_element(n, 0, 0, 1 as f64);
                set_matrix_element(n, 1, 0, unsafe { cos(theta) });
                set_matrix_element(n, 1, 1, -unsafe { sin(theta) });
                set_matrix_element(n, 2, 1, unsafe { sin(theta) });
                set_matrix_element(n, 2, 2, unsafe { cos(theta) });
            }
            1 => {
                set_matrix_element(n, 0, 0, unsafe { cos(theta) });
                set_matrix_element(n, 1, 2, unsafe { sin(theta) });
                set_matrix_element(n, 1, 1, 1 as f64);
                set_matrix_element(n, 2, 0, -unsafe { sin(theta) });
                set_matrix_element(n, 2, 2, unsafe { cos(theta) });
            }
            2 => {
                set_matrix_element(n, 0, 0, unsafe { cos(theta) });
                set_matrix_element(n, 0, 1, -unsafe { sin(theta) });
                set_matrix_element(n, 2, 0, unsafe { sin(theta) });
                set_matrix_element(n, 2, 1, unsafe { cos(theta) });
                set_matrix_element(n, 2, 2, 1 as f64);
            }
            _ => {}
        }
    }
    let ref_: *mut Matrix = multiply_matrices(m, n);
    delete_matrix(n);
    return ref_;
}
