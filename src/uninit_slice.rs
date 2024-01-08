use core::slice;
use std::mem::{ManuallyDrop, MaybeUninit};

/// # Safety
/// Marker trait for types that have the same size and alignment
/// This should only be implemented for types where this is a guarantee, such as with `repr(transparent)`
/// types, types where the only difference is lifetimes, and types that otherwise make such a stable guarantee.
pub unsafe trait EqSizeAlign<T> {}

// TODO: Add derive macro for types that only change lifetimes and add more impls

unsafe impl<'a, 'b, U, V> EqSizeAlign<&'a U> for &'b V {}
unsafe impl<'a, 'b, U, V> EqSizeAlign<&'a mut U> for &'b mut V {}
unsafe impl<U, V> EqSizeAlign<*const U> for *const V {}
unsafe impl<U, V> EqSizeAlign<*mut U> for *mut V {}

pub fn uninit_transmute<T, U>(b: Box<MaybeUninit<T>>) -> Box<MaybeUninit<U>> {
    let mut b = ManuallyDrop::new(b); // No sane person would implement `Drop` for `MaybeUninit<T>` but who knows
    unsafe { Box::from_raw(b.as_mut_ptr() as _) }
}

pub fn uninit_slice_convert<T, U: EqSizeAlign<T>>(
    b: Box<[MaybeUninit<T>]>,
) -> Box<[MaybeUninit<U>]> {
    let mut b = ManuallyDrop::new(b);
    let l = b.len();
    let p = b.as_mut_ptr() as *mut MaybeUninit<U>;
    unsafe { Box::from_raw(slice::from_raw_parts_mut(p, l)) }
}

#[test]
fn test_slice_convert() {
    unsafe impl EqSizeAlign<i32> for i32 {}
    let mut _a: Box<[MaybeUninit<i32>]> = Box::new([MaybeUninit::new(1)]);
    _a = uninit_slice_convert(_a);
}
