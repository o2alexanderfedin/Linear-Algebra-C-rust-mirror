use super::*;

/// Basic XOR operator
pub(crate) extern "C" fn exclusive_or(a: bool, b: bool) -> bool {
    return (a || b) && !(a && b) as i32 != 0;
}

/// Print the call stack up until assertion fail
pub(crate) extern "C" fn print_call_stack() -> () {
    let mut buffer: [*mut (); 50] = [core::ptr::null_mut(); 50];
    let levels: i32 =
        unsafe { backtrace(&raw mut buffer[0 as usize] as *mut *mut (), 50) };
    let symbols: *const *mut i8 =
        unsafe {
                backtrace_symbols(&raw mut buffer[0 as usize] as *mut *mut ()
                        as *const *mut (), levels)
            } as *const *mut i8;
    unsafe {
        printf(c"----------------------------- STACK TRACE ------------------------------\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        let mut i: i32 = 1;
        '__b69: loop {
            if !(i < levels) { break '__b69; }
            '__c69: loop {
                unsafe {
                    printf(c" %3d  %s\n".as_ptr() as *mut i8 as *const i8,
                        i - 1, unsafe { *symbols.offset(i as isize) })
                };
                break '__c69;
            }
            i += 1;
        }
    }
    unsafe {
        printf(c"------------------------------------------------------------------------\n".as_ptr()
                    as *mut i8 as *const i8)
    };
}

/// Basic custom assert
pub(crate) extern "C" fn assert(condition: i32) -> () {
    if (condition == 0) as i32 != 0 {
        unsafe {
            printf(c"Assertion failed.\n".as_ptr() as *mut i8 as *const i8)
        };
        print_call_stack();
        unsafe { exit(1) };
    }
}

/// Round double to n digits
pub(crate) extern "C" fn roundn(val: f64, n: u32) -> f64 {
    assert((n > 0 as u32) as i32);
    let x: f64 = unsafe { pow(10 as f64, n as f64) };
    return unsafe { round(val * x) } / x;
}
