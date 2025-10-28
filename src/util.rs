/// Overwrite the data in `dest`, neglecting to execute it's destructor.
#[inline(always)]
pub const fn write<T>(dest: &mut T, src: T) {
    // SAFETY: `dest` is a mutable reference to a valid `T`, as such it is
    //          valid for writes.
    unsafe { (&raw mut *dest).write(src) }
}
