use core::ffi::{c_char, c_int, c_void};

// https://github.com/rust-lang/rust/issues/88345
#[allow(non_camel_case_types)]
pub type c_size_t = usize;

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *mut c_void {
    let mut i: c_size_t = 0;

    while i < n {
        unsafe {
            *(dest as *mut c_char).wrapping_add(i) = *(src as *const c_char).wrapping_add(i);
        }
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(
    dest: *mut c_void,
    src: *const c_void,
    n: c_size_t,
) -> *mut c_void {
    if (dest as *const c_void) < src {
        unsafe { memcpy(dest, src, n) }
    } else {
        let mut i: c_size_t = n;

        while 0 < i {
            i -= 1;
            unsafe {
                *(dest as *mut c_char).wrapping_add(i) = *(src as *const c_char).wrapping_add(i);
            }
        }
        dest
    }
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: c_size_t) -> *mut c_void {
    let mut i: c_size_t = 0;

    while i < n {
        unsafe {
            *(dest as *mut c_char).wrapping_add(i) = c as c_char;
        }
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: c_size_t) -> c_int {
    let mut i: c_size_t = 0;

    while i < n {
        let diff = unsafe {
            *(s1 as *const c_char).wrapping_add(i) - *(s2 as *const c_char).wrapping_add(i)
        };

        if diff != 0 {
            return diff as c_int;
        }
        i += 1;
    }
    0
}
