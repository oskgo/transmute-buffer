use std::alloc::Layout;
use std::mem::ManuallyDrop;
use std::{
    mem::align_of,
    mem::{forget, size_of}
};

pub trait Clear {
    type Empty;

    fn clear(self) -> Self::Empty;
}

// Safety Invariant: ptr is an empty dangling or a global_allocd pointer of `ALIGN` alignment to a buffer of `capacity * SIZE <= isize::MAX` bytes
//   `capacity` is bounded by isize::MAX
pub struct EmptyVec {
    capacity: usize,
    ptr: *const (),
    align: usize,
    size: usize
}

impl EmptyVec {
    pub fn new<T>() -> Self {
        Self::with_capacity::<T>(0)
    }

    pub fn with_capacity<T: Sized>(capacity: usize) -> Self {
        if size_of::<T>() == 0 {
            EmptyVec {
                ptr: align_of::<T>() as *mut _, // TODO: Switch to `invalid_mut` on stabilization of `strict_provenance`
                capacity: isize::MAX as usize,
                align: align_of::<T>(),
                size: size_of::<T>()
            }
        } else if capacity == 0 {
            EmptyVec {
                ptr: align_of::<T>() as *mut _, // TODO: Switch to `invalid_mut` on stabilization of `strict_provenance`
                capacity: 0,
                align: align_of::<T>(),
                size: size_of::<T>()
            }
        } else {
            assert!(capacity < (isize::MAX as usize - 1) / size_of::<T>());
            // SAFETY: We check that neither the size nor the capacity are zero, so the layout has non-zero size
            EmptyVec {
                ptr: unsafe {
                    std::alloc::alloc(Layout::from_size_align(size_of::<T>() * capacity, align_of::<T>()).unwrap())
                        as *const ()
                },
                capacity,
                align: align_of::<T>(),
                size: size_of::<T>()
            }
        }
    }
}

impl Drop for EmptyVec {
    fn drop(&mut self) {
        if self.size != 0 && self.capacity != 0 {
            // SAFETY: If the size and capacity are non-zero the buffer must be non-empty so it is not dangling,
            //   and has thus been allocated by the global allocator
            unsafe {
                std::alloc::dealloc(
                    self.ptr as *mut u8,
                    Layout::from_size_align(self.size * self.capacity, self.align).unwrap(),
                )
            }
        }
    }
}

trait EmptyCollection<Collection> {
    fn typed(self) -> Collection;
}

impl<T> EmptyCollection<Vec<T>> for EmptyVec {
    fn typed(self) -> Vec<T> {
        assert!(size_of::<T>() == self.size);
        assert!(align_of::<T>() == self.align);
        let ev = ManuallyDrop::new(self);
        // SAFETY: The pointer is empty and dangling or points to an allocation from the global allocator.
        //   The buffer if allocated was allocated with the alignment of `T`.
        //   The size of the buffer is the product of the size of `T` and the capacity.
        //   The length is less than any capacity.
        //   The first 0 elements are vacuously well formed.
        //   The buffer is no greater than `isize::MAX`.
        unsafe { Vec::from_raw_parts(ev.ptr as *mut T, 0, ev.capacity) }
    }
}

impl<T> Clear for Vec<T> {
    type Empty = EmptyVec;

    fn clear(mut self) -> Self::Empty {
        Vec::clear(&mut self);
        let mut v = ManuallyDrop::new(self);
        let capacity = v.capacity();
        let ptr = v.as_mut_ptr();
        EmptyVec {
            ptr: ptr as *const (),
            capacity,
            align: align_of::<T>(),
            size: size_of::<T>()
        }
    }
}

#[test]
fn test_vec() {
    let mut a = EmptyVec::new::<&i32>();
    for _ in 0..2 {
        let mut b = a.typed();
        let c = &8;
        b.push(c);
        a = b.clear();
    }
}
