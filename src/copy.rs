use super::*;

/// Record of the results of a copy operation
#[derive(Debug, Copy, Clone)]
pub struct CopyRecord {
    /// The offset from the start of the allocation, in bytes, at which the
    /// copy operation began to write data.
    ///
    /// Not necessarily equal to the `start_offset` provided to the copy function, since this offset
    /// includes necessary padding to assure alignment.
    pub start_offset: usize,

    /// The offset from the start of the allocation, in bytes, at which the
    /// copy operation no longer wrote data.
    ///
    /// This does not include any padding at the end necessary to maintain
    /// alignment requirements.
    ///
    /// Unless you have a good reason otherwise, you *likely* want to use
    /// [`end_offset_padded`][CopyRecord::end_offset_padded] instead.
    pub end_offset: usize,

    /// The offset from the start of the allocation, in bytes, at which the
    /// copy operation no longer wrote data, plus any padding necessary to
    /// maintain derived alignment requirements.
    pub end_offset_padded: usize,
}

impl From<ComputedOffsets> for CopyRecord {
    fn from(
        ComputedOffsets {
            start,
            end,
            end_padded,
        }: ComputedOffsets,
    ) -> Self {
        Self {
            start_offset: start,
            end_offset: end,
            end_offset_padded: end_padded,
        }
    }
}

/// Copies `src` into the memory represented by `dst` starting at *exactly*
/// `start_offset` bytes past the start of `dst`
///
/// - `start_offset` is the offset into the allocation represented by `dst`, in bytes,
/// where the first byte of the copied data will be placed. If the requested
/// start offset does not satisfy computed alignment requirements, an error will
/// be returned and no data will be copied.
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline(always)]
pub fn copy_to_offset_exact<T: Copy, S: Slab + ?Sized>(
    src: &T,
    dst: &mut S,
    start_offset: usize,
) -> Result<CopyRecord, Error> {
    copy_to_offset_with_align_exact(src, dst, start_offset, 1)
}

/// Copies `src` into the memory represented by `dst` starting at *exactly*
/// `start_offset` bytes past the start of `dst` and with minimum alignment
/// `min_alignment`. If the requested parameters would be violated by computed alignment requirements,
/// an error will be returned.
///
/// - `start_offset` is the offset into the allocation represented by `dst`, in bytes,
/// where the first byte of the copied data will be placed. If the requested
/// start offset does not satisfy computed alignment requirements, an error will
/// be returned and no data will be copied.
/// - `min_alignment` is the minimum alignment that you are requesting the copy be aligned to. The
/// copy may be aligned greater than `min_alignment` depending on the alignment requirements
/// of `T` (the actual alignment will be the greater of the two between `align_of::<T>()` and
/// `min_align.next_power_of_two()`).
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline(always)]
pub fn copy_to_offset_with_align_exact<T: Copy, S: Slab + ?Sized>(
    src: &T,
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<CopyRecord, Error> {
    let t_layout = Layout::new::<T>();
    let offsets = compute_and_validate_offsets(&*dst, start_offset, t_layout, min_alignment, true)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let dst_ptr = unsafe { dst.base_ptr_mut().add(offsets.start) }.cast::<T>();

    // SAFETY:
    // - src is valid as we have a reference to it
    // - dst is valid so long as requirements for `slab` were met, i.e.
    // we have unique access to the region described and that it is valid for the duration
    // of 'a.
    // - areas not overlapping as long as safety requirements of creation of `self` were met,
    // i.e. that we have exclusive access to the region of memory described.
    // - dst aligned at least to align_of::<T>()
    // - checked that copy stays within bounds of our allocation
    unsafe {
        core::ptr::copy_nonoverlapping(src as *const T, dst_ptr, 1);
    }

    Ok(offsets.into())
}

/// Copies `src` into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `dst`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`].
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline]
pub fn copy_to_offset<T: Copy, S: Slab + ?Sized>(
    src: &T,
    dst: &mut S,
    start_offset: usize,
) -> Result<CopyRecord, Error> {
    copy_to_offset_with_align(src, dst, start_offset, 1)
}

/// Copies `src` into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `dst` and with minimum alignment
/// `min_alignment`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`].
/// - `min_alignment` is the minimum alignment to which the copy will be aligned. The
/// copy may not actually be aligned to `min_alignment` depending on the alignment requirements
/// of `T` (the actual alignment will be the greater between `align_of::<T>` and `min_align.next_power_of_two()`).
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline]
pub fn copy_to_offset_with_align<T: Copy, S: Slab + ?Sized>(
    src: &T,
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<CopyRecord, Error> {
    let t_layout = Layout::new::<T>();
    let offsets =
        compute_and_validate_offsets(&*dst, start_offset, t_layout, min_alignment, false)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let dst_ptr = unsafe { dst.base_ptr_mut().add(offsets.start) }.cast::<T>();

    // SAFETY:
    // - src is valid as we have a reference to it
    // - dst is valid so long as requirements for `slab` were met, i.e.
    // we have unique access to the region described and that it is valid for the duration
    // of 'a.
    // - areas not overlapping as long as safety requirements of creation of `self` were met,
    // i.e. that we have exclusive access to the region of memory described.
    // - dst aligned at least to align_of::<T>()
    // - checked that copy stays within bounds of our allocation
    unsafe {
        core::ptr::copy_nonoverlapping(src as *const T, dst_ptr, 1);
    }

    Ok(offsets.into())
}

/// Copies from `slice` into the memory represented by `dst` starting at *exactly*
/// `start_offset` bytes past the start of `self`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`, in bytes,
/// where the first byte of the copied data will be placed. If the requested
/// start offset does not satisfy computed alignment requirements, an error will
/// be returned and no data will be copied.
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline]
pub fn copy_from_slice_to_offset_exact<T: Copy, S: Slab + ?Sized>(
    src: &[T],
    dst: &mut S,
    start_offset: usize,
) -> Result<CopyRecord, Error> {
    copy_from_slice_to_offset_with_align(src, dst, start_offset, 1)
}

/// Copies from `slice` into the memory represented by `dst` starting at *exactly*
/// `start_offset` bytes past the start of `dst` and with minimum alignment `min_alignment`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`, in bytes,
/// where the first byte of the copied data will be placed. If the requested
/// start offset does not satisfy computed alignment requirements, an error will
/// be returned and no data will be copied.
/// - `min_alignment` is the minimum alignment that you are requesting the copy be aligned to. The
/// copy may be aligned greater than `min_alignment` depending on the alignment requirements
/// of `T` (the actual alignment will be the greater of the two between `align_of::<T>()` and
/// `min_align.next_power_of_two()`).
///     - The whole data of the slice will be copied directly, so, alignment between elements
///     ignores `min_alignment`.
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline]
pub fn copy_from_slice_to_offset_with_align_exact<T: Copy, S: Slab + ?Sized>(
    src: &[T],
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<CopyRecord, Error> {
    let t_layout = Layout::for_value(src);
    let offsets = compute_and_validate_offsets(&*dst, start_offset, t_layout, min_alignment, true)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let dst_ptr = unsafe { dst.base_ptr_mut().add(offsets.start) }.cast::<T>();

    // SAFETY:
    // - src is valid as we have a reference to it
    // - dst is valid so long as requirements for `slab` were met, i.e.
    // we have unique access to the region described and that it is valid for the duration
    // of 'a.
    // - areas not overlapping as long as safety requirements of creation of `self` were met,
    // i.e. that we have exclusive access to the region of memory described.
    // - dst aligned at least to align_of::<T>()
    // - checked that copy stays within bounds of our allocation
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), dst_ptr, src.len());
    }

    Ok(offsets.into())
}

/// Copies from `slice` into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `self`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`].
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline]
pub fn copy_from_slice_to_offset<T: Copy, S: Slab + ?Sized>(
    src: &[T],
    dst: &mut S,
    start_offset: usize,
) -> Result<CopyRecord, Error> {
    copy_from_slice_to_offset_with_align(src, dst, start_offset, 1)
}

/// Copies from `slice` into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `dst`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`].
/// - `min_alignment` is the minimum alignment that you are requesting the copy be aligned to. The
/// copy may be aligned greater than `min_alignment` depending on the alignment requirements
/// of `T` (the actual alignment will be the greater of the two between `align_of::<T>()` and
/// `min_align.next_power_of_two()`).
///     - The whole data of the slice will be copied directly, so alignment between elements
///     ignores `min_alignment`.
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline]
pub fn copy_from_slice_to_offset_with_align<T: Copy, S: Slab + ?Sized>(
    src: &[T],
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<CopyRecord, Error> {
    let t_layout = Layout::for_value(src);
    let offsets =
        compute_and_validate_offsets(&*dst, start_offset, t_layout, min_alignment, false)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let dst_ptr = unsafe { dst.base_ptr_mut().add(offsets.start) }.cast::<T>();

    // SAFETY:
    // - src is valid as we have a reference to it
    // - dst is valid so long as requirements for `slab` were met, i.e.
    // we have unique access to the region described and that it is valid for the duration
    // of 'a.
    // - areas not overlapping as long as safety requirements of creation of `self` were met,
    // i.e. that we have exclusive access to the region of memory described.
    // - dst aligned at least to align_of::<T>()
    // - checked that copy stays within bounds of our allocation
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), dst_ptr, src.len());
    }

    Ok(offsets.into())
}

/// Copies from `src` iterator into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `dst`.
///
/// Returns a vector of [`CopyRecord`]s, one for each item in the `src` iterator.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`]s.
/// - `min_alignment` is the minimum alignment that you are requesting the copy be aligned to. The
/// copy may be aligned greater than `min_alignment` depending on the alignment requirements
/// of `T` (the actual alignment will be the greater of the two between `align_of::<T>()` and
/// `min_align.next_power_of_two()`).
/// - For this variation, `min_alignment` will also be respected *between* elements yielded by
/// the iterator. To copy inner elements aligned only to `align_of::<T>()` (i.e. with the layout of
/// an `[T]`), see [`copy_from_iter_to_offset_with_align_packed`].
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[cfg(feature = "std")]
#[inline]
pub fn copy_from_iter_to_offset_with_align<T: Copy, Iter: Iterator<Item = T>, S: Slab + ?Sized>(
    src: Iter,
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<Vec<CopyRecord>, Error> {
    let mut offset = start_offset;

    src.map(|item| {
        let copy_record = copy_to_offset_with_align(&item, dst, offset, min_alignment)?;
        offset = copy_record.end_offset;
        Ok(copy_record)
    })
    .collect::<Result<Vec<_>, _>>()
}

/// Like [`copy_from_iter_to_offset_with_align`] except that
/// alignment between elements yielded by the iterator will ignore `min_alignment`
/// and rather only be aligned to the alignment of `T`.
///
/// Because of this, only one [`CopyRecord`] is returned specifying the record of the
/// entire block of copied data. If the `src` iterator is empty, returns `None`.
#[inline]
pub fn copy_from_iter_to_offset_with_align_packed<
    T: Copy,
    Iter: Iterator<Item = T>,
    S: Slab + ?Sized,
>(
    mut src: Iter,
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<Option<CopyRecord>, Error> {
    let first_record = if let Some(first_item) = src.next() {
        copy_to_offset_with_align(&first_item, dst, start_offset, min_alignment)?
    } else {
        return Ok(None);
    };

    let mut prev_record = first_record;

    for item in src {
        let copy_record = copy_to_offset_with_align(&item, dst, prev_record.end_offset, 1)?;
        prev_record = copy_record;
    }

    Ok(Some(CopyRecord {
        start_offset: first_record.start_offset,
        end_offset: prev_record.end_offset,
        end_offset_padded: prev_record.end_offset_padded,
    }))
}

/// Like [`copy_from_iter_to_offset_with_align_packed`] except that it will return an error
/// and no data will be copied if the supplied `start_offset` doesn't meet the computed alignment
/// requirements.
#[inline]
pub fn copy_from_iter_to_offset_with_align_exact_packed<
    T: Copy,
    Iter: Iterator<Item = T>,
    S: Slab + ?Sized,
>(
    mut src: Iter,
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<Option<CopyRecord>, Error> {
    let first_record = if let Some(first_item) = src.next() {
        copy_to_offset_with_align_exact(&first_item, dst, start_offset, min_alignment)?
    } else {
        return Ok(None);
    };

    let mut prev_record = first_record;

    for item in src {
        let copy_record = copy_to_offset_with_align_exact(&item, dst, prev_record.end_offset, 1)?;
        prev_record = copy_record;
    }

    Ok(Some(CopyRecord {
        start_offset: first_record.start_offset,
        end_offset: prev_record.end_offset,
        end_offset_padded: prev_record.end_offset_padded,
    }))
}
