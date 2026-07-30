use super::*;
use crate::linear_algebra_h::{Matrix, Vector};
use crate::matrix::{
    add_matrices, copy_matrix, delete_matrix, determinant, flatten_matrix,
    get_anti_diagonal, get_col_vector, get_main_diagonal, get_matrix_element,
    get_row_vector, identity_matrix, inverse_matrix, is_diagonal_matrix,
    is_identity_matrix, is_lo_tri_matrix, is_matrix_equal,
    is_matrix_symmetric, is_square_matrix, is_triangular_matrix,
    is_up_tri_matrix, is_zero_matrix, matrix_size, matrix_size_bytes,
    multiply_matrices, new_matrix, print_matrix, scale_matrix,
    set_anti_diagonal, set_col_vector, set_main_diagonal, set_matrix_element,
    set_row_vector, sub_matrix, trace_matrix, transpose_matrix, zero_matrix,
};
use crate::utils::{assert, roundn};
use crate::vector::{
    copy_vector, cross_product, delete_vector, dot_product,
    get_vector_element, is_unit_vector, is_vector_equal, is_vector_orthogonal,
    new_vector, set_vector_element, vector_distance, vector_magnitude,
    vector_size_bytes, zero_vector,
};

pub(crate) extern "C" fn test_create() -> () {
    unsafe {
        printf(c"\nTest create matrix and vector from array...\n".as_ptr() as
                    *mut i8 as *const i8)
    };
    let mut data: [f64; 4] = [1 as f64, 2 as f64, 3 as f64, 4 as f64];
    let m: *mut Matrix =
        new_matrix(&raw mut data[0 as usize] as *mut f64, 2, 2);
    assert((get_matrix_element(m, 0, 0) == 1 as f64 &&
                                get_matrix_element(m, 0, 1) == 2 as f64 &&
                            get_matrix_element(m, 1, 0) == 3 as f64 &&
                        get_matrix_element(m, 1, 1) == 4 as f64 &&
                    matrix_size(m) == 4 && matrix_size_bytes(m) == 32) as i32);
    delete_matrix(m);
    let mut data2: [f64; 8] =
        [1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64, 6 as f64, 7 as f64,
                8 as f64];
    let v: *mut Vector =
        new_vector(&raw mut data2[0 as usize] as *mut f64, 8);
    assert((vector_size_bytes(v) == 64) as i32);
    delete_vector(v);
}

pub(crate) extern "C" fn test_copy() -> () {
    unsafe {
        printf(c"\nTest matrix and vector copying...\n".as_ptr() as *mut i8 as
                *const i8)
    };
    let m: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(m, 0, 1, 5 as f64);
    let n: *mut Matrix = copy_matrix(m);
    assert((get_matrix_element(m, 0, 0) == get_matrix_element(n, 0, 0) &&
                        get_matrix_element(m, 0, 1) == get_matrix_element(n, 0, 1)
                    &&
                    get_matrix_element(m, 1, 0) == get_matrix_element(n, 1, 0)
                && get_matrix_element(m, 1, 1) == get_matrix_element(n, 1, 1))
            as i32);
    delete_matrix(n);
    delete_matrix(m);
    let v: *mut Vector = zero_vector(4);
    set_vector_element(v, 2, 2 as f64);
    let v2: *mut Vector = copy_vector(v);
    assert((get_vector_element(v2, 2) == 2 as f64) as i32);
    delete_vector(v2);
    delete_vector(v);
}

pub(crate) extern "C" fn test_flatten() -> () {
    unsafe {
        printf(c"\nTest flatten...\n".as_ptr() as *mut i8 as *const i8)
    };
    let mut data: [f64; 6] =
        [1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64, 6 as f64];
    let m: *mut Matrix =
        new_matrix(&raw mut data[0 as usize] as *mut f64, 2, 3);
    let flat: *mut Vector = flatten_matrix(m);
    assert((get_vector_element(flat, 0) == 1 as f64 &&
                get_vector_element(flat, 5) == 6 as f64) as i32);
    delete_vector(flat);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_elem() -> () {
    unsafe {
        printf(c"\nTest element accessor and mutator...\n".as_ptr() as *mut i8
                as *const i8)
    };
    let m: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(m, 0, 0, 4 as f64);
    assert((get_matrix_element(m, 0, 0) == 4 as f64 &&
                        get_matrix_element(m, 0, 1) == 0 as f64 &&
                    get_matrix_element(m, 1, 0) == 0 as f64 &&
                get_matrix_element(m, 1, 1) == 0 as f64) as i32);
    delete_matrix(m);
    let v: *mut Vector = zero_vector(2);
    set_vector_element(v, 1, 10 as f64);
    assert((get_vector_element(v, 1) == 10 as f64) as i32);
    delete_vector(v);
}

pub(crate) extern "C" fn test_row_vector() -> () {
    unsafe {
        printf(c"\nTest accessor/mutator for row vectors...\n".as_ptr() as
                    *mut i8 as *const i8)
    };
    let m: *mut Matrix = zero_matrix(3, 2);
    let row: *mut Vector = get_row_vector(m, 1);
    set_vector_element(row, 0, 123 as f64);
    assert((get_vector_element(row, 0) == 123 as f64 &&
                get_vector_element(row, 1) == 0 as f64) as i32);
    let mut data: [f64; 2] = [1 as f64, 4 as f64];
    let v: *mut Vector = new_vector(&raw mut data[0 as usize] as *mut f64, 2);
    set_row_vector(m, 1, v);
    assert((get_matrix_element(m, 0, 0) == 0 as f64 &&
                        get_matrix_element(m, 0, 1) == 0 as f64 &&
                    get_matrix_element(m, 1, 0) == 1 as f64 &&
                get_matrix_element(m, 1, 1) == 4 as f64) as i32);
    delete_vector(v);
    delete_vector(row);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_col_vector() -> () {
    unsafe {
        printf(c"\nTest accessor/mutator for column vectors...\n".as_ptr() as
                    *mut i8 as *const i8)
    };
    let m: *mut Matrix = zero_matrix(3, 2);
    let col: *mut Vector = get_col_vector(m, 0);
    set_vector_element(col, 0, 5 as f64);
    set_vector_element(col, 1, 10 as f64);
    set_vector_element(col, 2, 15 as f64);
    assert((get_vector_element(col, 0) == 5 as f64 &&
                get_vector_element(col, 1) == 10 as f64) as i32);
    let mut data: [f64; 2] = [10 as f64, 3 as f64];
    let v: *mut Vector = new_vector(&raw mut data[0 as usize] as *mut f64, 2);
    set_col_vector(m, 0, v);
    delete_vector(v);
    delete_vector(col);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_main_diagonal() -> () {
    unsafe {
        printf(c"\nTest accessor/mutator for main diagonal...\n".as_ptr() as
                    *mut i8 as *const i8)
    };
    let m: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(m, 0, 0, 34 as f64);
    set_matrix_element(m, 1, 1, 56 as f64);
    let v: *mut Vector = get_main_diagonal(m);
    assert((get_vector_element(v, 0) == 34 as f64 &&
                get_vector_element(v, 1) == 56 as f64) as i32);
    delete_vector(v);
    delete_matrix(m);
    let n: *mut Matrix = zero_matrix(2, 2);
    let w: *mut Vector = zero_vector(2);
    set_vector_element(w, 0, 100 as f64);
    set_vector_element(w, 1, 4 as f64);
    set_main_diagonal(n, w);
    assert((get_matrix_element(n, 0, 0) == 100 as f64 &&
                get_matrix_element(n, 1, 1) == 4 as f64) as i32);
    delete_matrix(n);
    delete_vector(w);
}

pub(crate) extern "C" fn test_anti_diagonal() -> () {
    unsafe {
        printf(c"\nTest accessor/mutator for anti diagonal...\n".as_ptr() as
                    *mut i8 as *const i8)
    };
    let m: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(m, 0, 1, 100 as f64);
    set_matrix_element(m, 1, 0, 250 as f64);
    let d: *mut Vector = get_anti_diagonal(m);
    assert((get_vector_element(d, 0) == 250 as f64 &&
                get_vector_element(d, 1) == 100 as f64) as i32);
    delete_vector(d);
    delete_matrix(m);
    let n: *mut Matrix = zero_matrix(2, 2);
    let e: *mut Vector = zero_vector(2);
    set_vector_element(e, 0, 9 as f64);
    set_vector_element(e, 1, 8 as f64);
    set_anti_diagonal(n, e);
    assert((get_matrix_element(n, 1, 0) == 9 as f64 &&
                get_matrix_element(n, 0, 1) == 8 as f64) as i32);
    delete_matrix(n);
    delete_vector(e);
}

pub(crate) extern "C" fn test_is_equal() -> () {
    unsafe {
        printf(c"\nTest vector and matrix equal...\n".as_ptr() as *mut i8 as
                *const i8)
    };
    let m: *mut Matrix = zero_matrix(2, 2);
    let n: *mut Matrix = zero_matrix(2, 2);
    let o: *mut Matrix = zero_matrix(3, 3);
    assert(!is_matrix_equal(m, o) as i32 as i32);
    assert(is_matrix_equal(m, n) as i32);
    set_matrix_element(m, 0, 0, 1 as f64);
    assert(!is_matrix_equal(m, n) as i32 as i32);
    delete_matrix(o);
    delete_matrix(n);
    delete_matrix(m);
    let v: *mut Vector = zero_vector(3);
    let w: *mut Vector = zero_vector(3);
    assert(is_vector_equal(v, w) as i32);
    delete_vector(w);
    delete_vector(v);
}

pub(crate) extern "C" fn test_is_zero_matrix() -> () {
    unsafe {
        printf(c"\nTest isZeroMatrix...\n".as_ptr() as *mut i8 as *const i8)
    };
    let m: *mut Matrix = zero_matrix(1, 3);
    let n: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(n, 0, 0, 1 as f64);
    assert(is_zero_matrix(m) as i32);
    assert(!is_zero_matrix(n) as i32 as i32);
    delete_matrix(n);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_is_identity_matrix() -> () {
    unsafe {
        printf(c"\nTest isIdentityMatrix...\n".as_ptr() as *mut i8 as
                *const i8)
    };
    let m: *mut Matrix = zero_matrix(2, 2);
    set_matrix_element(m, 0, 0, 1 as f64);
    set_matrix_element(m, 1, 1, 1 as f64);
    assert(is_identity_matrix(m) as i32);
    delete_matrix(m);
    let n: *mut Matrix = zero_matrix(1, 3);
    assert(!is_identity_matrix(n) as i32 as i32);
    delete_matrix(n);
    let o: *mut Matrix = zero_matrix(3, 3);
    set_matrix_element(o, 0, 0, 1 as f64);
    set_matrix_element(o, 1, 1, 1 as f64);
    set_matrix_element(o, 2, 2, 1 as f64);
    set_matrix_element(o, 1, 2, -4 as f64);
    assert(!is_identity_matrix(o) as i32 as i32);
    delete_matrix(o);
    let p: *mut Matrix = identity_matrix(2);
    assert(is_identity_matrix(p) as i32);
    delete_matrix(p);
}

pub(crate) extern "C" fn test_is_square_matrix() -> () {
    unsafe {
        printf(c"\nTest isSquareMatrix...\n".as_ptr() as *mut i8 as *const i8)
    };
    let m: *mut Matrix = zero_matrix(2, 2);
    assert(is_square_matrix(m) as i32);
    delete_matrix(m);
    let n: *mut Matrix = zero_matrix(1, 3);
    assert(!is_square_matrix(n) as i32 as i32);
    delete_matrix(n);
}

pub(crate) extern "C" fn test_is_diagonal() -> () {
    unsafe {
        printf(c"\nTest isDiagonal...\n".as_ptr() as *mut i8 as *const i8)
    };
    let mut data1: [f64; 9] =
        [1 as f64, 2 as f64, 3 as f64, 0 as f64, 5 as f64, 6 as f64, 0 as f64,
                0 as f64, 9 as f64];
    let m1: *mut Matrix =
        new_matrix(&raw mut data1[0 as usize] as *mut f64, 3, 3);
    assert(!is_diagonal_matrix(m1) as i32 as i32);
    delete_matrix(m1);
    let mut data2: [f64; 9] =
        [1 as f64, 0 as f64, 0 as f64, 0 as f64, 2 as f64, 0 as f64, 0 as f64,
                0 as f64, 3 as f64];
    let m2: *mut Matrix =
        new_matrix(&raw mut data2[0 as usize] as *mut f64, 3, 3);
    assert(is_diagonal_matrix(m2) as i32);
    delete_matrix(m2);
}

pub(crate) extern "C" fn test_is_triangular() -> () {
    unsafe {
        printf(c"\nTest isTriangular, isLoTriMatrix, isUpTriMatrix...\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    let mut data1: [f64; 9] =
        [1 as f64, 2 as f64, 3 as f64, 0 as f64, 5 as f64, 6 as f64, 0 as f64,
                0 as f64, 9 as f64];
    let m1: *mut Matrix =
        new_matrix(&raw mut data1[0 as usize] as *mut f64, 3, 3);
    assert(is_up_tri_matrix(m1) as i32);
    delete_matrix(m1);
    let mut data2: [f64; 9] =
        [1 as f64, 0 as f64, 0 as f64, 4 as f64, 5 as f64, 0 as f64, 7 as f64,
                8 as f64, 9 as f64];
    let m2: *mut Matrix =
        new_matrix(&raw mut data2[0 as usize] as *mut f64, 3, 3);
    assert(is_lo_tri_matrix(m2) as i32);
    delete_matrix(m2);
    let mut data3: [f64; 9] =
        [1 as f64, 0 as f64, 0 as f64, 4 as f64, 5 as f64, 0 as f64, 7 as f64,
                8 as f64, 9 as f64];
    let m3: *mut Matrix =
        new_matrix(&raw mut data3[0 as usize] as *mut f64, 3, 3);
    assert(is_triangular_matrix(m3) as i32);
    delete_matrix(m3);
}

pub(crate) extern "C" fn test_is_symmetric() -> () {
    unsafe {
        printf(c"\nTest isSymmetric...\n".as_ptr() as *mut i8 as *const i8)
    };
    let mut data: [f64; 6] =
        [1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64, 6 as f64];
    let m: *mut Matrix =
        new_matrix(&raw mut data[0 as usize] as *mut f64, 2, 3);
    assert(!is_matrix_symmetric(m) as i32 as i32);
    delete_matrix(m);
    let n: *mut Matrix = zero_matrix(2, 2);
    assert(is_matrix_symmetric(n) as i32);
    delete_matrix(n);
}

pub(crate) extern "C" fn test_transpose() -> () {
    unsafe {
        printf(c"\nTest transposeMatrix...\n".as_ptr() as *mut i8 as
                *const i8)
    };
    let mut data: [f64; 6] =
        [1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64, 6 as f64];
    let m: *mut Matrix =
        new_matrix(&raw mut data[0 as usize] as *mut f64, 2, 3);
    let t: *mut Matrix = transpose_matrix(m);
    assert((get_matrix_element(t, 0, 0) == 1 as f64 &&
                                get_matrix_element(t, 1, 0) == 2 as f64 &&
                            get_matrix_element(t, 2, 0) == 3 as f64 &&
                        get_matrix_element(t, 0, 1) == 4 as f64 &&
                    get_matrix_element(t, 1, 1) == 5 as f64 &&
                get_matrix_element(t, 2, 1) == 6 as f64) as i32);
    delete_matrix(t);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_trace() -> () {
    unsafe { printf(c"\nTest trace...\n".as_ptr() as *mut i8 as *const i8) };
    let mut data: [f64; 9] =
        [1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64, 6 as f64, 7 as f64,
                8 as f64, 9 as f64];
    let m: *mut Matrix =
        new_matrix(&raw mut data[0 as usize] as *mut f64, 3, 3);
    assert((trace_matrix(m) == 15 as f64) as i32);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_add_matrices() -> () {
    unsafe {
        printf(c"\nTest adding matrices...\n".as_ptr() as *mut i8 as
                *const i8)
    };
    let mut data1: [f64; 4] = [4 as f64, 6 as f64, 3 as f64, 8 as f64];
    let mut data2: [f64; 4] = [6 as f64, 4 as f64, 7 as f64, 2 as f64];
    let m1: *mut Matrix =
        new_matrix(&raw mut data1[0 as usize] as *mut f64, 2, 2);
    let m2: *mut Matrix =
        new_matrix(&raw mut data2[0 as usize] as *mut f64, 2, 2);
    let sum: *mut Matrix = add_matrices(m1, m2);
    assert((get_matrix_element(sum, 0, 0) == 10 as f64 &&
                        get_matrix_element(sum, 0, 1) == 10 as f64 &&
                    get_matrix_element(sum, 1, 0) == 10 as f64 &&
                get_matrix_element(sum, 1, 1) == 10 as f64) as i32);
    delete_matrix(sum);
    delete_matrix(m2);
    delete_matrix(m1);
}

pub(crate) extern "C" fn test_multiply_matrices() -> () {
    unsafe {
        printf(c"\nTest multiplying matrices...\n".as_ptr() as *mut i8 as
                *const i8)
    };
    let mut data1: [f64; 4] = [1 as f64, 2 as f64, 3 as f64, 4 as f64];
    let mut data2: [f64; 4] = [1 as f64, 1 as f64, 1 as f64, 1 as f64];
    let m1: *mut Matrix =
        new_matrix(&raw mut data1[0 as usize] as *mut f64, 2, 2);
    let m2: *mut Matrix =
        new_matrix(&raw mut data2[0 as usize] as *mut f64, 2, 2);
    let prod: *mut Matrix = multiply_matrices(m1, m2);
    assert((get_matrix_element(prod, 0, 0) == 3 as f64 &&
                        get_matrix_element(prod, 0, 1) == 3 as f64 &&
                    get_matrix_element(prod, 1, 0) == 7 as f64 &&
                get_matrix_element(prod, 1, 1) == 7 as f64) as i32);
    delete_matrix(prod);
    delete_matrix(m2);
    delete_matrix(m1);
}

pub(crate) extern "C" fn test_scale_matrix() -> () {
    unsafe {
        printf(c"\nTest scaling matrices...\n".as_ptr() as *mut i8 as
                *const i8)
    };
    let mut data: [f64; 6] =
        [1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64, 6 as f64];
    let m: *mut Matrix =
        new_matrix(&raw mut data[0 as usize] as *mut f64, 3, 2);
    let scaled: *mut Matrix = scale_matrix(m, 10 as f64);
    assert((get_matrix_element(scaled, 0, 0) == 10 as f64 &&
                                get_matrix_element(scaled, 0, 1) == 20 as f64 &&
                            get_matrix_element(scaled, 1, 0) == 30 as f64 &&
                        get_matrix_element(scaled, 1, 1) == 40 as f64 &&
                    get_matrix_element(scaled, 2, 0) == 50 as f64 &&
                get_matrix_element(scaled, 2, 1) == 60 as f64) as i32);
    delete_matrix(scaled);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_sub_matrix() -> () {
    unsafe {
        printf(c"\nTest sub matrix...\n".as_ptr() as *mut i8 as *const i8)
    };
    let mut data: [f64; 9] =
        [1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64, 6 as f64, 7 as f64,
                8 as f64, 9 as f64];
    let m: *mut Matrix =
        new_matrix(&raw mut data[0 as usize] as *mut f64, 3, 3);
    let sub: *mut Matrix = sub_matrix(m, 2, 2);
    assert((get_matrix_element(sub, 0, 0) == 1 as f64 &&
                        get_matrix_element(sub, 0, 1) == 2 as f64 &&
                    get_matrix_element(sub, 1, 0) == 4 as f64 &&
                get_matrix_element(sub, 1, 1) == 5 as f64) as i32);
    delete_matrix(sub);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_determinant() -> () {
    unsafe {
        printf(c"\nTest determinant...\n".as_ptr() as *mut i8 as *const i8)
    };
    let mut data1: [f64; 1] = [10 as f64];
    let m1: *mut Matrix =
        new_matrix(&raw mut data1[0 as usize] as *mut f64, 1, 1);
    assert((determinant(m1) == 10 as f64) as i32);
    delete_matrix(m1);
    let mut data2: [f64; 4] = [4 as f64, 6 as f64, 3 as f64, 8 as f64];
    let m2: *mut Matrix =
        new_matrix(&raw mut data2[0 as usize] as *mut f64, 2, 2);
    assert((determinant(m2) == 14 as f64) as i32);
    delete_matrix(m2);
    let mut data3: [f64; 9] =
        [6 as f64, 1 as f64, 1 as f64, 4 as f64, -2 as f64, 5 as f64,
                2 as f64, 8 as f64, 7 as f64];
    let m3: *mut Matrix =
        new_matrix(&raw mut data3[0 as usize] as *mut f64, 3, 3);
    assert((determinant(m3) == -306 as f64) as i32);
    delete_matrix(m3);
    let mut data4: [f64; 16] =
        [11 as f64, 9 as f64, 24 as f64, 2 as f64, 1 as f64, 5 as f64,
                2 as f64, 6 as f64, 3 as f64, 17 as f64, 18 as f64, 1 as f64,
                2 as f64, 5 as f64, 7 as f64, 1 as f64];
    let m4: *mut Matrix =
        new_matrix(&raw mut data4[0 as usize] as *mut f64, 4, 4);
    let det: f64 = determinant(m4);
    assert((284 as f64 == roundn(det, 1 as u32)) as i32);
    delete_matrix(m4);
}

pub(crate) extern "C" fn test_inverse() -> () {
    unsafe {
        printf(c"\nTest inverse matrix...\n".as_ptr() as *mut i8 as *const i8)
    };
    let mut data: [f64; 9] =
        [3 as f64, 0 as f64, 2 as f64, 2 as f64, 0 as f64, -2 as f64,
                0 as f64, 1 as f64, 1 as f64];
    let m: *mut Matrix =
        new_matrix(&raw mut data[0 as usize] as *mut f64, 3, 3);
    let inv: *mut Matrix = inverse_matrix(m);
    unsafe { printf(c"\nInv(m)\n".as_ptr() as *mut i8 as *const i8) };
    print_matrix(inv, 0);
    delete_matrix(inv);
    delete_matrix(m);
}

pub(crate) extern "C" fn test_vector_operations() -> () {
    unsafe {
        printf(c"\nTest vector operations...\n".as_ptr() as *mut i8 as
                *const i8)
    };
    let mut data1: [f64; 3] = [2 as f64, 4 as f64, 3 as f64];
    let mut data2: [f64; 3] = [5 as f64, 10 as f64, 15 as f64];
    let mut data3: [f64; 3] = [1 as f64, 0 as f64, 0 as f64];
    let mut data4: [f64; 3] = [0 as f64, 1 as f64, 0 as f64];
    let v: *mut Vector =
        new_vector(&raw mut data1[0 as usize] as *mut f64, 3);
    let w: *mut Vector =
        new_vector(&raw mut data2[0 as usize] as *mut f64, 3);
    let u1: *mut Vector =
        new_vector(&raw mut data3[0 as usize] as *mut f64, 3);
    let u2: *mut Vector =
        new_vector(&raw mut data4[0 as usize] as *mut f64, 3);
    assert((roundn(vector_magnitude(v), 3 as u32) == 5.385) as i32);
    assert((!is_unit_vector(v) as i32 != 0 && is_unit_vector(u1)) as i32);
    assert((dot_product(v, w) == 95 as f64) as i32);
    assert(is_vector_orthogonal(u1, u2) as i32);
    assert((roundn(vector_distance(v, w), 3 as u32) == 13.748) as i32);
    let cross: *mut Vector = cross_product(v, w);
    assert((get_vector_element(cross, 0) == 30 as f64 &&
                    get_vector_element(cross, 1) == 15 as f64 &&
                get_vector_element(cross, 2) == 0 as f64) as i32);
    delete_vector(cross);
    delete_vector(u2);
    delete_vector(u1);
    delete_vector(w);
    delete_vector(v);
}

pub(crate) extern "C" fn __main_inner() -> i32 {
    test_create();
    test_copy();
    test_flatten();
    test_elem();
    test_row_vector();
    test_col_vector();
    test_main_diagonal();
    test_anti_diagonal();
    test_is_equal();
    test_is_zero_matrix();
    test_is_identity_matrix();
    test_is_square_matrix();
    test_is_diagonal();
    test_is_triangular();
    test_is_symmetric();
    test_transpose();
    test_trace();
    test_add_matrices();
    test_multiply_matrices();
    test_scale_matrix();
    test_sub_matrix();
    test_determinant();
    test_inverse();
    test_vector_operations();
    return 0;
}
