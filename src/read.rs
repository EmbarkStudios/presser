use super::*;

/// Gets a shared reference to a `T` within `slab` at `offset`.
/// 
/// - `offset` is the offset, in bytes, after the start of `slab` at which a `T` is placed.
/// 
/// The function will return an error if:
/// - `offset` within `slab` is not properly aligned for `T`
/// - `offset` is out of bounds of the `slab`
/// - `offset + size_of::<T>` is out of bounds of the `slab`
/// 
/// # Safety
/// 
/// You must have previously **fully-initialized** a **valid** `T` at the given offset into `slab`.
/// Validity is a complex topic not to be taken lightly.
/// See [this rust reference page](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) for more details.
#[inline]
pub unsafe fn read_at_offset<'a, T, S: Slab>(slab: &'a S, offset: usize) -> Result<&'a T, Error> {
    let t_layout = Layout::new::<T>();
    let offsets = compute_offsets(slab, offset, t_layout, 1, true)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let ptr = unsafe { slab.base_ptr().add(offsets.start) }.cast::<T>();

    // SAFETY: 
    // - `ptr` is properly aligned, checked by us
    // - `slab` contains enough space for `T` at `ptr`, checked by
    // - if the function-level safety guarantees are met, then:
    //     - `ptr` contains a previously-placed `T`
    //     - we have shared access to all of `slab`, which includes `ptr`.
    Ok(unsafe { &*ptr })
}

/// Gets a mutable reference to a `T` within `slab` at `offset`.
/// 
/// - `offset` is the offset, in bytes, after the start of `slab` at which a `T` is placed.
/// 
/// The function will return an error if:
/// - `offset` within `slab` is not properly aligned for `T`
/// - `offset` is out of bounds of the `slab`
/// - `offset + size_of::<T>` is out of bounds of the `slab`
/// 
/// # Safety
/// 
/// You must have previously **fully-initialized** a **valid**\* `T` at the given offset into `slab`. If you want to fill an uninitialized
/// buffer with data, you should instead use any of the copy helper functions or one of the `maybe_uninit_mut` read functions.
/// 
/// \* Validity is a complex topic not to be taken lightly.
/// See [this rust reference page](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) for more details.
#[inline]
pub unsafe fn read_at_offset_mut<'a, T, S: Slab>(slab: &'a mut S, offset: usize) -> Result<&'a mut T, Error> {
    let t_layout = Layout::new::<T>();
    let offsets = compute_offsets(slab, offset, t_layout, 1, true)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let ptr = unsafe { slab.base_ptr_mut().add(offsets.start) }.cast::<T>();

    // SAFETY: 
    // - `ptr` is properly aligned, checked by us
    // - `slab` contains enough space for `T` at `ptr`, checked by
    // - if the function-level safety guarantees are met, then:
    //     - `ptr` contains a previously-placed `T`
    //     - we have unique access to all of `slab`, which includes `ptr`.
    Ok(unsafe { &mut *ptr })
}

/// Gets a mutable reference to a `MaybeUninit<T>` within `slab` at `offset`.
/// 
/// - `offset` is the offset, in bytes, after the start of `slab` at which a `T` may be placed.
/// 
/// The function will return an error if:
/// - `offset` within `slab` is not properly aligned for `T`
/// - `offset` is out of bounds of the `slab`
/// - `offset + size_of::<T>` is out of bounds of the `slab`
/// 
/// # Safety
/// 
/// This function is safe since in order to read any data you need to call the unsafe [`MaybeUninit::assume_init`] on the returned value.
/// However, you should know that if you do that, you must have ensured that there is indeed a **valid** `T` in its place.
/// Validity is a complex topic not to be taken lightly.
/// See [this rust reference page](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) for more details.
#[inline]
pub fn get_maybe_uninit_at_offset_mut<'a, T, S: Slab>(slab: &'a mut S, offset: usize) -> Result<&'a mut MaybeUninit<T>, Error> {
    let t_layout = Layout::new::<T>();
    let offsets = compute_offsets(slab, offset, t_layout, 1, true)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let ptr = unsafe { slab.base_ptr_mut().add(offsets.start) }.cast::<MaybeUninit<T>>();

    // SAFETY: 
    // - `ptr` is properly aligned, checked by us
    // - `slab` contains enough space for `T` at `ptr`, checked by us
    // - if the function-level safety guarantees are met, then:
    //     - we have unique access to all of `slab`, which includes `ptr`.
    Ok(unsafe { &mut *ptr })
}

/// Reads a `&[T]` within `slab` at `offset`.
/// 
/// - `offset` is the offset, in bytes, after the start of `slab` at which a `[T; len]` is placed.
/// - `len` is the length of the returned slice, counted in elements of `T`.
/// 
/// The function will return an error if:
/// - `offset` within `slab` is not properly aligned for `T`
/// - `offset` is out of bounds of the `slab`
/// - `offset + size_of::<T> * len` is out of bounds of the `slab`
/// 
/// # Safety
/// 
/// You must have previously **fully-initialized** a **valid** a `[T; len]` at the given offset into `slab`.
/// Validity is a complex topic not to be taken lightly.
/// See [this rust reference page](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) for more details.
#[inline]
pub unsafe fn read_slice_at_offset<'a, T, S: Slab>(slab: &'a S, offset: usize, len: usize) -> Result<&'a [T], Error> {
    let t_layout = match Layout::array::<T>(len) {
        Ok(layout) => layout,
        Err(_) => return Err(Error::InvalidLayout),
    };
    let offsets = compute_offsets(slab, offset, t_layout, 1, true)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let ptr = unsafe { slab.base_ptr().add(offsets.start) }.cast::<T>();

    // SAFETY: 
    // - `ptr` is properly aligned, checked by us
    // - `slab` contains enough space for the slice's layout, checked by us
    // - if the function-level safety guarantees are met, then:
    //     - `ptr` contains a previously-placed `[T; len]`
    //     - we have shared access to all of `slab`, which includes `ptr`.
    Ok(unsafe { core::slice::from_raw_parts(ptr, len) })
}

/// Reads a `&mut [T]` within `slab` at `offset`.
/// 
/// - `offset` is the offset, in bytes, after the start of `slab` at which a `[T; len]` is placed.
/// - `len` is the length of the returned slice, counted in elements of `T`.
/// 
/// The function will return an error if:
/// - `offset` within `slab` is not properly aligned for `T`
/// - `offset` is out of bounds of the `slab`
/// - `offset + size_of::<T> * len` is out of bounds of the `slab`
/// 
/// # Safety
/// 
/// You must have previously **fully-initialized** a **valid**\* `[T; len]` at the given offset into `slab`. If you want to fill an uninitialized
/// buffer with data, you should instead use any of the copy helper functions or one of the `maybe_uninit_mut` read functions.
/// 
/// \* Validity is a complex topic not to be taken lightly.
/// See [this rust reference page](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) for more details.
#[inline]
pub unsafe fn read_slice_at_offset_mut<'a, T, S: Slab>(slab: &'a mut S, offset: usize, len: usize) -> Result<&'a mut [T], Error> {
    let t_layout = match Layout::array::<T>(len) {
        Ok(layout) => layout,
        Err(_) => return Err(Error::InvalidLayout),
    };
    let offsets = compute_offsets(slab, offset, t_layout, 1, true)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let ptr = unsafe { slab.base_ptr_mut().add(offsets.start) }.cast::<T>();

    // SAFETY: 
    // - `ptr` is properly aligned, checked by us
    // - `slab` contains enough space for the slice's layout, checked by us
    // - if the function-level safety guarantees are met, then:
    //     - `ptr` contains a previously-placed `[T; len]`
    //     - we have mutable access to all of `slab`, which includes `ptr`.
    Ok(unsafe { core::slice::from_raw_parts_mut(ptr, len) })
}

/// Gets a `&mut [MaybeUninit<T>]` within `slab` at `offset`.
/// 
/// - `offset` is the offset, in bytes, after the start of `slab` at which a `[T; len]` may be placed.
/// - `len` is the length of the returned slice, counted in elements of `T`.
/// 
/// The function will return an error if:
/// - `offset` within `slab` is not properly aligned for `T`
/// - `offset` is out of bounds of the `slab`
/// - `offset + size_of::<T> * len` is out of bounds of the `slab`
/// 
/// # Safety
/// 
/// This function is safe since in order to read any data you need to call the unsafe [`MaybeUninit::assume_init`] on the returned value.
/// However, you should know that if you do that, you must have ensured that there is indeed a **valid** `T` in its place.
/// Validity is a complex topic not to be taken lightly.
/// See [this rust reference page](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) for more details.
#[inline]
pub fn get_maybe_uninit_slice_at_offset_mut<'a, T, S: Slab>(slab: &'a mut S, offset: usize, len: usize) -> Result<&'a mut [MaybeUninit<T>], Error> {
    let t_layout = match Layout::array::<T>(len) {
        Ok(layout) => layout,
        Err(_) => return Err(Error::InvalidLayout),
    };
    let offsets = compute_offsets(slab, offset, t_layout, 1, true)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let ptr = unsafe { slab.base_ptr_mut().add(offsets.start) }.cast::<MaybeUninit<T>>();

    // SAFETY: 
    // - `ptr` is properly aligned, checked by us
    // - `slab` contains enough space for the slice's layout, checked by us
    // - if the function-level safety guarantees are met, then:
    //     - we have mutable access to all of `slab`, which includes `ptr`.
    Ok(unsafe { core::slice::from_raw_parts_mut(ptr, len) })
}