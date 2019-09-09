use libc::c_void;
use std::mem;

pub fn make_mut<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    if !ptr.is_null() {
        Some(unsafe { &mut *ptr })
    } else {
        None
    }
}

pub unsafe fn make_mut_unchecked<'a, T>(ptr: *mut T) -> &'a mut T {
    debug_assert!(!ptr.is_null());
    &mut *ptr
}

pub unsafe fn into_fh<T>(val: T) -> u64 {
    debug_assert!(mem::size_of::<*mut c_void>() <= mem::size_of::<u64>());
    Box::into_raw(Box::new(val)) as *mut c_void as u64
}

// Safety: the value of `fh` is a valid pointer of `T` or zero.
pub unsafe fn from_fh<T>(fh: u64) -> Option<T> {
    if fh != 0 {
        let ptr = fh as *mut c_void as *mut T;
        Some(*Box::from_raw(ptr))
    } else {
        None
    }
}