/// Creates a new Vec<u8> with the specified capacity and memory alignment.
///
/// # Safety
///
/// This function is unsafe because:
/// 1. It directly allocates memory using the system allocator and creates a Vec from raw parts.
/// 2. The returned Vec must not be resized (e.g., with `resize`, `reserve`, `push`, etc.) as this
///    could cause undefined behavior due to the custom allocation.
/// 3. The `align` parameter must be a power of 2, otherwise the function will panic.
/// 4. The caller must ensure that the Vec is properly dropped to avoid memory leaks.
///
/// # Arguments
///
/// * `cap` - The capacity of the Vec in bytes
/// * `align` - The memory alignment in bytes (must be a power of 2)
///
/// # Panics
///
/// This function will panic if:
/// * The alignment is not a power of 2
/// * Memory allocation fails
///
/// # Examples
///
/// ```
/// use hbb_common::mem::aligned_u8_vec;
/// let aligned_vec = unsafe { aligned_u8_vec(1024, 16) };
/// // Use the aligned_vec, but do not resize it
/// ```
pub unsafe fn aligned_u8_vec(cap: usize, align: usize) -> Vec<u8> {
    use std::alloc::*;

    let layout =
        Layout::from_size_align(cap, align).expect("invalid aligned value, must be power of 2");
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            panic!("failed to allocate {} bytes", cap);
        }
        Vec::from_raw_parts(ptr, 0, cap)
    }
}
