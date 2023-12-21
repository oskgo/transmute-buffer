use std::mem::{size_of, align_of};

pub trait Reusable<B> {
    type Buffer;
    fn take(&mut self) -> Self::Buffer;
    fn put(&mut self, t: Self::Buffer);
}

pub struct ReusableVec<A>(Option<Vec<A>>);

impl<A> ReusableVec<A> {
    pub fn new() -> Self {
        Self(None)
    }
}

impl<A, B> Reusable<B> for ReusableVec<A> {
    type Buffer = Vec<B>;

    fn take(&mut self) -> Vec<B> {
        debug_assert!(size_of::<A>() == size_of::<B>());
        debug_assert!(align_of::<A>() == align_of::<B>());

        let mut vec = self.0.take().unwrap_or_else(Vec::new);
        vec.clear();
        let converted =
            vec.into_iter().map(|_| unreachable!()).collect();
        converted
    }

    fn put(&mut self, mut returned: Vec<B>) {
        debug_assert!(size_of::<A>() == size_of::<B>());
        debug_assert!(align_of::<A>() == align_of::<B>());

        returned.clear();
        self.0 = Some(returned.into_iter().map(|_| unreachable!()).collect());
    }
}

#[test]
fn test() {
    use std::path::PathBuf;
    let mut b = ReusableVec::<&str>::new();

    {
        let s = String::from("foo");
        let mut v = b.take();
        v.push(&*s);
        println!("{:p}, {:?}", &*v, v);
        b.put(v);
    }
    {
        let p = PathBuf::from("/bar");
        let mut v = b.take();
        v.push(&*p);
        println!("{:p}, {:?}", &*v, v);
        b.put(v);
    }
}