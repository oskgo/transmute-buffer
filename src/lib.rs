#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use std::{mem::align_of, mem::{size_of, forget}};

use layout::Alignment;

pub mod layout {
    use std::convert::Infallible;

    pub struct LayoutTag<const ALIGN: usize, const SIZE: usize>
    where
        USize<ALIGN>: Alignment,
    {_e: Unconstructable<Layout<ALIGN, SIZE>>}

    pub struct Unconstructable<T> {
        _t: T,
        _n: Infallible
    }

    pub struct Layout<const ALIGN: usize, const SIZE: usize>
    where
        USize<ALIGN>: Alignment
    {
        _size: Size<SIZE>,
        _align: Align<ALIGN>
    }

    pub type Size<const N: usize> = [u8; N];

    pub type Align<const N: usize> = <USize<N> as Alignment>::Archetype;

    pub struct USize<const N: usize>
    where
        Self: Alignment;

    pub trait Alignment {
        type Archetype;
    }

    #[repr(C, align(1))] pub struct Alignment1; impl Alignment for USize<1> { type Archetype = Alignment1; }
    #[repr(C, align(2))] pub struct Alignment2; impl Alignment for USize<2> { type Archetype = Alignment2; }
    #[repr(C, align(4))] pub struct Alignment4; impl Alignment for USize<4> { type Archetype = Alignment4; }
    #[repr(C, align(8))] pub struct Alignment8; impl Alignment for USize<8> { type Archetype = Alignment8; }
    #[repr(C, align(16))] pub struct Alignment16; impl Alignment for USize<16> { type Archetype = Alignment16; }
    #[repr(C, align(32))] pub struct Alignment32; impl Alignment for USize<32> { type Archetype = Alignment32; }
    #[repr(C, align(64))] pub struct Alignment64; impl Alignment for USize<64> { type Archetype = Alignment64; }
    #[repr(C, align(128))] pub struct Alignment128; impl Alignment for USize<128> { type Archetype = Alignment128; }
    #[repr(C, align(256))] pub struct Alignment256; impl Alignment for USize<256> { type Archetype = Alignment256; }
    #[repr(C, align(512))] pub struct Alignment512; impl Alignment for USize<512> { type Archetype = Alignment512; }
    #[repr(C, align(1024))] pub struct Alignment1024; impl Alignment for USize<1024> { type Archetype = Alignment1024; }
    #[repr(C, align(2048))] pub struct Alignment2048; impl Alignment for USize<2048> { type Archetype = Alignment2048; }
    #[repr(C, align(4096))] pub struct Alignment4096; impl Alignment for USize<4096> { type Archetype = Alignment4096; }
    #[repr(C, align(8192))] pub struct Alignment8192; impl Alignment for USize<8192> { type Archetype = Alignment8192; }
    #[repr(C, align(16384))] pub struct Alignment16384; impl Alignment for USize<16384> { type Archetype = Alignment16384; }
    #[repr(C, align(32768))] pub struct Alignment32768; impl Alignment for USize<32768> { type Archetype = Alignment32768; }
    #[repr(C, align(65536))] pub struct Alignment65536; impl Alignment for USize<65536> { type Archetype = Alignment65536; }
    #[repr(C, align(131072))] pub struct Alignment131072; impl Alignment for USize<131072> { type Archetype = Alignment131072; }
    #[repr(C, align(262144))] pub struct Alignment262144; impl Alignment for USize<262144> { type Archetype = Alignment262144; }
    #[repr(C, align(524288))] pub struct Alignment524288; impl Alignment for USize<524288> { type Archetype = Alignment524288; }
    #[repr(C, align(1048576))] pub struct Alignment1048576; impl Alignment for USize<1048576> { type Archetype = Alignment1048576; }
    #[repr(C, align(2097152))] pub struct Alignment2097152; impl Alignment for USize<2097152> { type Archetype = Alignment2097152; }
    #[repr(C, align(4194304))] pub struct Alignment4194304; impl Alignment for USize<4194304> { type Archetype = Alignment4194304; }
    #[repr(C, align(8388608))] pub struct Alignment8388608; impl Alignment for USize<8388608> { type Archetype = Alignment8388608; }
    #[repr(C, align(16777216))] pub struct Alignment16777216; impl Alignment for USize<16777216> { type Archetype = Alignment16777216; }
    #[repr(C, align(33554432))] pub struct Alignment33554432; impl Alignment for USize<33554432> { type Archetype = Alignment33554432; }
    #[repr(C, align(67108864))] pub struct Alignment67108864; impl Alignment for USize<67108864> { type Archetype = Alignment67108864; }
    #[repr(C, align(134217728))] pub struct Alignment134217728; impl Alignment for USize<134217728> { type Archetype = Alignment134217728; }
    #[repr(C, align(268435456))] pub struct Alignment268435456; impl Alignment for USize<268435456> { type Archetype = Alignment268435456; }
}

pub trait TransmutableContainer {
    type Empty;

    fn clear(self) -> Self::Empty;
}

pub trait EmptyContainer<Target> {
    fn retype(self) -> Target;
}

impl<T> TransmutableContainer for Vec<T>
where
    layout::USize<{align_of::<T>()}>: Alignment,
    [(); size_of::<T>()]: Sized
{
    type Empty = Vec<layout::LayoutTag<{align_of::<T>()}, {size_of::<T>()}>>;

    fn clear(mut self) -> Self::Empty {
        Vec::clear(&mut self);
        debug_assert!(self.len() == 0);
        let c = self.capacity();
        let p = self.as_mut_ptr();
        forget(self);
        // SAFETY: Empty vectors of types with equal layout are the same
        unsafe {Vec::from_raw_parts(p as _, 0, c)}
    }
}

impl<T> EmptyContainer<Vec<T>> for Vec<layout::LayoutTag<{align_of::<T>()}, {size_of::<T>()}>>
where
    layout::USize<{align_of::<T>()}>: Alignment,
    [(); size_of::<T>()]: Sized
{
    fn retype(mut self) -> Vec<T> {
        debug_assert!(self.len() == 0);
        let c = self.capacity();
        let p = self.as_mut_ptr();
        forget(self);
        // SAFETY: Empty vectors of types with equal layout are the same
        // Since LayoutTag is an empty type we know statically that the vector is empty so clearing is not needed
        unsafe {Vec::from_raw_parts(p as _, 0, c)}
    }
}

#[test]
fn test_vec() {
    let mut a = vec![];
    for _ in 0..2 {
        let mut b = a.retype();
        let c = &8;
        b.push(c);
        a = b.clear();
    }
}