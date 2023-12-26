#![feature(generic_const_exprs)]
#![feature(strict_provenance)]
#![allow(incomplete_features)]
use std::alloc::Layout;
use std::{
    mem::align_of,
    mem::{forget, size_of},
    ptr::invalid_mut,
};

pub trait Clear {
    type Empty;

    fn clear(self) -> Self::Empty;
}

// Safety Invariant: ptr is an empty dangling or a global_allocd pointer of `ALIGN` alignment to a buffer of `capacity * SIZE <= isize::MAX` bytes
//   `capacity` is bounded by isize::MAX
pub struct EmptyVec<const SIZE: usize, const ALIGN: usize> {
    capacity: usize,
    ptr: *const (),
}

impl<const SIZE: usize, const ALIGN: usize> Default for EmptyVec<SIZE, ALIGN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize, const ALIGN: usize> EmptyVec<SIZE, ALIGN> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if SIZE == 0 {
            EmptyVec {
                ptr: invalid_mut(ALIGN),
                capacity: isize::MAX as usize,
            }
        } else if capacity == 0 {
            EmptyVec {
                ptr: invalid_mut(ALIGN),
                capacity: 0,
            }
        } else {
            assert!(capacity < (isize::MAX as usize - 1) / SIZE);
            // SAFETY: We check that neither the size nor the capacity are zero, so the layout has non-zero size
            EmptyVec {
                ptr: unsafe {
                    std::alloc::alloc(Layout::from_size_align(SIZE * capacity, ALIGN).unwrap())
                        as *const ()
                },
                capacity,
            }
        }
    }
}

impl<const SIZE: usize, const ALIGN: usize> Drop for EmptyVec<SIZE, ALIGN> {
    fn drop(&mut self) {
        if SIZE != 0 && self.capacity != 0 {
            // SAFETY: If the size and capacity are non-zero the buffer must be non-empty so it is not dangling,
            //   and has thus been allocated by the global allocator
            unsafe {
                std::alloc::dealloc(
                    self.ptr as *mut u8,
                    Layout::from_size_align(SIZE * self.capacity, ALIGN).unwrap(),
                )
            }
        }
    }
}

trait EmptyCollection<Collection> {
    fn typed(self) -> Collection;
}

impl<T> EmptyCollection<Vec<T>> for EmptyVec<{ size_of::<T>() }, { align_of::<T>() }> {
    fn typed(self) -> Vec<T> {
        // SAFETY: The pointer is empty and dangling or points to an allocation from the global allocator.
        //   The buffer if allocated was allocated with the alignment of `T`.
        //   The size of the buffer is the product of the size of `T` and the capacity.
        //   The length is less than any capacity.
        //   The first 0 elements are vacuously well formed.
        //   The buffer is no greater than `isize::MAX`.
        unsafe { Vec::from_raw_parts(self.ptr as *mut T, 0, self.capacity) }
    }
}

impl<T> Clear for Vec<T>
where
    [(); align_of::<T>()]: Sized,
    [(); size_of::<T>()]: Sized,
{
    type Empty = EmptyVec<{ align_of::<T>() }, { size_of::<T>() }>;

    fn clear(mut self) -> Self::Empty {
        Vec::clear(&mut self);
        let capacity = self.capacity();
        let ptr = self.as_mut_ptr();
        forget(self);
        EmptyVec {
            ptr: ptr as *const (),
            capacity,
        }
    }
}

#[test]
fn test_vec() {
    let mut a = EmptyVec::new();
    for _ in 0..2 {
        let mut b = a.typed();
        let c = &8;
        b.push(c);
        a = b.clear();
    }
}
