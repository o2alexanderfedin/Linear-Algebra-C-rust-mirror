use super::*;
use crate::linear_algebra_h::Vector;
use crate::utils::assert;

/// Helper function for asserting vector and vector data
pub(crate) extern "C" fn assert_vector(v: *mut Vector) -> bool {
    assert((v as *mut () != 0 as *mut () &&
                unsafe { (*v).data } as *mut () != 0 as *mut ()) as i32);
    return 1;
}

/// Return new vector with null data
pub(crate) extern "C" fn null_vector(cols: i32) -> *mut Vector {
    assert((cols > 0) as i32);
    let v: *mut Vector =
        unsafe { malloc(core::mem::size_of::<Vector>() as u64) } as
            *mut Vector;
    unsafe { (*v).cols = cols };
    unsafe {
        (*v).data =
            unsafe {
                    malloc((cols as
                                u64).wrapping_mul(core::mem::size_of::<f64>() as u64))
                } as *mut f64
    };
    return v;
}

/// Return new vector from double array d with size cols
pub(crate) extern "C" fn new_vector(d: *mut f64, cols: i32) -> *mut Vector {
    assert((d as *mut () != 0 as *mut () && cols > 0) as i32);
    let v: *mut Vector = null_vector(cols);
    let mut idx: i32 = 0;
    {
        let mut i: i32 = 0;
        '__b70: loop {
            if !(i < unsafe { (*v).cols }) { break '__b70; }
            '__c70: loop {
                unsafe {
                    *unsafe { (*v).data.offset(i as isize) } =
                        unsafe {
                            *d.offset({
                                            let __p = &mut idx;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize)
                        }
                };
                break '__c70;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return v;
}

/// Replace all elements in vector v with n
pub(crate) extern "C" fn fill_vector(v: &*mut Vector, n: f64) -> () {
    assert_vector(*v);
    {
        let mut i: i32 = 0;
        '__b71: loop {
            if !(i < unsafe { (**v).cols }) { break '__b71; }
            '__c71: loop {
                unsafe { *unsafe { (**v).data.offset(i as isize) } = n };
                break '__c71;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

/// Return new vector as a zero vector of size cols
pub(crate) extern "C" fn zero_vector(cols: i32) -> *mut Vector {
    let v: *mut Vector = null_vector(cols);
    fill_vector(&v, 0 as f64);
    return v;
}

/// Release vector v from memory
pub(crate) extern "C" fn delete_vector(mut v: *mut Vector) -> () {
    unsafe { free(unsafe { (*v).data } as *mut ()) };
    unsafe { (*v).data = 0 as *mut () as *mut f64 };
    unsafe { free(v as *mut ()) };
    v = 0 as *mut () as *mut Vector;
}

/// Return new vector as a copy of vector v
pub(crate) extern "C" fn copy_vector(v: *mut Vector) -> *mut Vector {
    assert_vector(v);
    let c: *mut Vector = zero_vector(unsafe { (*v).cols });
    {
        let mut i: i32 = 0;
        '__b72: loop {
            if !(i < unsafe { (*v).cols }) { break '__b72; }
            '__c72: loop {
                unsafe {
                    *unsafe { (*c).data.offset(i as isize) } =
                        unsafe { *unsafe { (*v).data.offset(i as isize) } }
                };
                break '__c72;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return c;
}

/// Return number of elements of vector v
pub(crate) extern "C" fn vector_size(v: *mut Vector) -> i32 {
    assert_vector(v);
    return unsafe { (*v).cols };
}

/// Return size of vector in bytes
pub(crate) extern "C" fn vector_size_bytes(v: *mut Vector) -> i32 {
    return (core::mem::size_of::<f64>() as
                    u64).wrapping_mul(vector_size(v) as u64) as i32;
}

/// Set element of vector v[i] to scalar s
pub(crate) extern "C" fn set_vector_element(v: *mut Vector, i: i32, s: f64)
    -> () {
    assert((assert_vector(v) && i >= 0 && i < unsafe { (*v).cols }) as i32);
    unsafe { *unsafe { (*v).data.offset(i as isize) } = s };
}

/// Return scalar as element v[i]
pub(crate) extern "C" fn get_vector_element(v: *mut Vector, i: i32) -> f64 {
    assert((assert_vector(v) && i >= 0 && i < unsafe { (*v).cols }) as i32);
    return unsafe { *unsafe { (*v).data.offset(i as isize) } };
}

/// "Pretty" print vector v
pub(crate) extern "C" fn print_vector(v: *mut Vector, include_indices: bool)
    -> () {
    assert_vector(v);
    {
        let mut i: i32 = 0;
        '__b73: loop {
            if !(i < unsafe { (*v).cols }) { break '__b73; }
            '__c73: loop {
                if include_indices {
                    unsafe {
                        printf(c"[%d] -> ".as_ptr() as *mut i8 as *const i8, i)
                    };
                }
                unsafe {
                    printf(c"%16.8f ".as_ptr() as *mut i8 as *const i8,
                        unsafe { *unsafe { (*v).data.offset(i as isize) } })
                };
                break '__c73;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

/// Vectors v and w are equal if they contain identical elements
pub(crate) extern "C" fn is_vector_equal(v: *mut Vector, w: *mut Vector)
    -> bool {
    assert((assert_vector(v) && assert_vector(w)) as i32);
    if unsafe { (*v).cols } != unsafe { (*w).cols } { return 0; }
    {
        let mut i: i32 = 0;
        '__b74: loop {
            if !(i < unsafe { (*v).cols }) { break '__b74; }
            '__c74: loop {
                if unsafe { *unsafe { (*v).data.offset(i as isize) } } !=
                        unsafe { *unsafe { (*w).data.offset(i as isize) } } {
                    return 0;
                }
                break '__c74;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}

/// Return new vector as sum of vectors v1 and v2
pub(crate) extern "C" fn add_vectors(v: *mut Vector, w: *mut Vector)
    -> *mut Vector {
    assert((assert_vector(v) && assert_vector(w) &&
                unsafe { (*v).cols } == unsafe { (*w).cols }) as i32);
    let sum: *mut Vector = null_vector(unsafe { (*v).cols });
    {
        let mut i: i32 = 0;
        '__b75: loop {
            if !(i < unsafe { (*v).cols }) { break '__b75; }
            '__c75: loop {
                unsafe {
                    *unsafe { (*sum).data.offset(i as isize) } =
                        unsafe { *unsafe { (*v).data.offset(i as isize) } } +
                            unsafe { *unsafe { (*w).data.offset(i as isize) } }
                };
                break '__c75;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return sum;
}

/// Return new vector as vector v ^ k
pub(crate) extern "C" fn pow_vector(v: *mut Vector, k: f64) -> *mut Vector {
    assert_vector(v);
    let p: *mut Vector = null_vector(unsafe { (*v).cols });
    {
        let mut i: i32 = 0;
        '__b76: loop {
            if !(i < unsafe { (*v).cols }) { break '__b76; }
            '__c76: loop {
                unsafe {
                    *unsafe { (*p).data.offset(i as isize) } =
                        unsafe {
                            pow(unsafe { *unsafe { (*v).data.offset(i as isize) } }, k)
                        }
                };
                break '__c76;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return p;
}

/// Return scalar as dot product of vectors v1 and v2 (Euclidean inner product)
pub(crate) extern "C" fn dot_product(v: *mut Vector, w: *mut Vector) -> f64 {
    assert((assert_vector(v) && assert_vector(w) &&
                unsafe { (*v).cols } == unsafe { (*w).cols }) as i32);
    let mut dp: f64 = 0 as f64;
    {
        let mut i: i32 = 0;
        '__b77: loop {
            if !(i < unsafe { (*v).cols }) { break '__b77; }
            '__c77: loop {
                dp +=
                    unsafe { *unsafe { (*v).data.offset(i as isize) } } *
                        unsafe { *unsafe { (*w).data.offset(i as isize) } };
                break '__c77;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return dp;
}

/// Return new vector as cross product of vectors v1 and v2 (3 dimensions)
pub(crate) extern "C" fn cross_product(v: *mut Vector, w: *mut Vector)
    -> *mut Vector {
    assert((assert_vector(v) && assert_vector(w) && unsafe { (*v).cols } == 3
                && unsafe { (*v).cols } == 3) as i32);
    let c: *mut Vector = null_vector(3);
    unsafe {
        *unsafe { (*c).data.offset(0 as isize) } =
            unsafe { *unsafe { (*v).data.offset(1 as isize) } } *
                    unsafe { *unsafe { (*w).data.offset(2 as isize) } } -
                unsafe { *unsafe { (*v).data.offset(2 as isize) } } *
                    unsafe { *unsafe { (*w).data.offset(1 as isize) } }
    };
    unsafe {
        *unsafe { (*c).data.offset(1 as isize) } =
            unsafe { *unsafe { (*v).data.offset(0 as isize) } } *
                    unsafe { *unsafe { (*w).data.offset(2 as isize) } } -
                unsafe { *unsafe { (*v).data.offset(2 as isize) } } *
                    unsafe { *unsafe { (*w).data.offset(0 as isize) } }
    };
    unsafe {
        *unsafe { (*c).data.offset(2 as isize) } =
            unsafe { *unsafe { (*v).data.offset(0 as isize) } } *
                    unsafe { *unsafe { (*w).data.offset(1 as isize) } } -
                unsafe { *unsafe { (*v).data.offset(1 as isize) } } *
                    unsafe { *unsafe { (*w).data.offset(0 as isize) } }
    };
    return c;
}

/// Return scalar as vector magnitude of vector v (length or magnitude)
pub(crate) extern "C" fn vector_magnitude(v: *mut Vector) -> f64 {
    assert_vector(v);
    let mut sum: f64 = 0 as f64;
    {
        let mut i: i32 = 0;
        '__b78: loop {
            if !(i < unsafe { (*v).cols }) { break '__b78; }
            '__c78: loop {
                sum +=
                    unsafe { *unsafe { (*v).data.offset(i as isize) } } *
                        unsafe { *unsafe { (*v).data.offset(i as isize) } };
                break '__c78;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return unsafe { sqrt(sum) };
}

/// Return scalar as euclidean distance between vectors v1 and v2
pub(crate) extern "C" fn vector_distance(v: *mut Vector, w: *mut Vector)
    -> f64 {
    assert((assert_vector(v) && assert_vector(w) &&
                unsafe { (*v).cols } == unsafe { (*w).cols }) as i32);
    let mut d: f64 = 0 as f64;
    {
        let mut i: i32 = 0;
        '__b79: loop {
            if !(i < unsafe { (*v).cols }) { break '__b79; }
            '__c79: loop {
                d +=
                    (unsafe { *unsafe { (*w).data.offset(i as isize) } } -
                            unsafe { *unsafe { (*v).data.offset(i as isize) } }) *
                        (unsafe { *unsafe { (*w).data.offset(i as isize) } } -
                            unsafe { *unsafe { (*v).data.offset(i as isize) } });
                break '__c79;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return unsafe { sqrt(d) };
}

/// Return new vector as vector v scaled by s
pub(crate) extern "C" fn scale_vector(v: *mut Vector, s: f64) -> *mut Vector {
    assert(assert_vector(v) as i32);
    let scaled: *mut Vector = null_vector(unsafe { (*v).cols });
    {
        let mut i: i32 = 0;
        '__b80: loop {
            if !(i < unsafe { (*v).cols }) { break '__b80; }
            '__c80: loop {
                unsafe {
                    *unsafe { (*scaled).data.offset(i as isize) } =
                        unsafe { *unsafe { (*v).data.offset(i as isize) } } * s
                };
                break '__c80;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return scaled;
}

/// Vector v is a unit vector if the vectorMagnitude(v) = 1
pub(crate) extern "C" fn is_unit_vector(v: *mut Vector) -> bool {
    return vector_magnitude(v) == 1 as f64;
}

/// Vector v1 is orthogonal (perpendicular) to vector v2 if dotProduct(v1, v2) == 0
pub(crate) extern "C" fn is_vector_orthogonal(v1: *mut Vector,
    v2: *mut Vector) -> bool {
    assert((assert_vector(v1) && assert_vector(v2)) as i32);
    return dot_product(v1, v2) == 0 as f64;
}

/// Return scalar of scalar triple product of vectors v1, v2, and v3
pub(crate) extern "C" fn scalar_triple_product(v1: *mut Vector,
    v2: *mut Vector, v3: *mut Vector) -> f64 {
    assert((unsafe { (*v1).cols } == 3 && unsafe { (*v2).cols } == 3 &&
                unsafe { (*v3).cols } == 3) as i32);
    return dot_product(v1, cross_product(v2, v3));
}
