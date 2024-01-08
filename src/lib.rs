use core::slice;
use std::mem::{ManuallyDrop, MaybeUninit};

use uninit_slice::{uninit_slice_convert, EqSizeAlign};

pub mod uninit_slice;

pub trait Clear<Cleared> {
    fn clear(self) -> Cleared;
}

pub fn to_maybeuninit_slice<T>(v: Vec<T>) -> Box<[MaybeUninit<T>]> {
    let mut v = ManuallyDrop::new(v);
    let capacity = v.capacity();
    let ptr = v.as_mut_ptr() as _;
    unsafe { Box::from_raw(slice::from_raw_parts_mut(ptr, capacity)) }
}

pub fn from_maybeuninit_slice<T>(b: Box<[MaybeUninit<T>]>) -> Vec<T> {
    let mut b = ManuallyDrop::new(b);
    let capacity = b.len();
    let ptr = b.as_mut_ptr() as _;
    unsafe { Vec::from_raw_parts(ptr, 0, capacity) }
}

pub struct EmptyVec<T> {
    b: Box<[MaybeUninit<T>]>,
}

impl<T> Default for EmptyVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EmptyVec<T> {
    pub fn new() -> Self {
        EmptyVec {
            b: to_maybeuninit_slice(Vec::new()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        EmptyVec {
            b: to_maybeuninit_slice(Vec::with_capacity(capacity)),
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        from_maybeuninit_slice(self.b)
    }
}

impl<T, U: EqSizeAlign<T>> Clear<EmptyVec<U>> for Vec<T> {
    fn clear(mut self) -> EmptyVec<U> {
        Vec::clear(&mut self);
        EmptyVec {
            b: uninit_slice_convert(to_maybeuninit_slice(self)),
        }
    }
}

#[test]
fn test_vec() {
    let mut a: EmptyVec<&i32> = EmptyVec::new();
    for _ in 0..2 {
        let mut b = a.to_vec();
        let c = &8;
        b.push(c);
        a = b.clear();
    }
}
