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
