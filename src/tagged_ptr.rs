#[cfg(not(target_pointer_width = "64"))]
const ERROR: () = "Pointer size should be 64 bits";

use std::marker::PhantomData;
use std::mem;
use std::boxed::Box;
use std::ops::{Deref, DerefMut};
use std;

const BITSIZE: usize = 3;
const BITMASK: usize = (1 << BITSIZE) - 1;

pub trait Packable3 {
    unsafe fn pack(&self) -> usize;
    unsafe fn unpack(usize) -> Self;
}
impl<T> Packable3 for *const T {
    unsafe fn pack(&self) -> usize {(*self as usize) >> BITSIZE}
    unsafe fn unpack(data: usize) -> Self {(data << BITSIZE) as Self}
}
impl<T> Packable3 for *mut T {
    unsafe fn pack(&self) -> usize {(*self as usize) >> BITSIZE}
    unsafe fn unpack(data: usize) -> Self {(data << BITSIZE) as Self}
}
impl<'a, T> Packable3 for &'a T {
    unsafe fn pack(&self) -> usize {(*self as *const T).pack()}
    unsafe fn unpack(data: usize) -> Self {
        let ptr: *const T = Packable3::unpack(data);
        &*ptr as &'a T
    }
}
impl<'a, T> Packable3 for &'a mut T {
    unsafe fn pack(&self) -> usize {(*self as *const T).pack()}
    unsafe fn unpack(data: usize) -> Self {
        let ptr: *mut T = Packable3::unpack(data);
        &mut *ptr as &'a mut T
    }
}
impl<T> Packable3 for Box<T> {
    unsafe fn pack(&self) -> usize {(&**self).pack()}
    unsafe fn unpack(data:usize) -> Self {Box::from_raw(Packable3::unpack(data))}
}
impl Packable3 for bool {
    unsafe fn pack(&self) -> usize {*self as usize}
    unsafe fn unpack(data: usize) -> Self {assert!(data < 2); data == 0}
}
impl Packable3 for u16 {
    unsafe fn pack(&self) -> usize {*self as usize}
    unsafe fn unpack(data: usize) -> Self {assert!(data <= (u16::max_value() as usize)); data as u16}
}
impl Packable3 for u32 {
    unsafe fn pack(&self) -> usize {*self as usize}
    unsafe fn unpack(data: usize) -> Self {assert!(data <= (u32::max_value() as usize)); data as u32}
}
impl Packable3 for f32 {
    unsafe fn pack(&self) -> usize {std::mem::transmute::<f32, u32>(*self) as usize
}
    unsafe fn unpack(data: usize) -> Self {std::mem::transmute::<u32, f32>(data as u32)
}
}
impl Packable3 for () {
    unsafe fn pack(&self) -> usize {0}
    unsafe fn unpack(d: usize) -> Self {assert!(d == 0); ()}
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Unpacked<A=(),B=(),C=(),D=(),E=(),F=(),G=(),H=()> {
    A(A), B(B), C(C), D(D),
    E(E), F(F), G(G), H(H)
}
pub struct Packed<A=(),B=(),C=(),D=(),E=(),F=(),G=(),H=()> where
    A: Packable3, B: Packable3, C: Packable3, D: Packable3,
    E: Packable3, F: Packable3, G: Packable3, H: Packable3 {
    tagged_ptr: usize,
    phantom: PhantomData<(A,B,C,D,E,F,G,H)>
}

impl<A,B,C,D,E,F,G,H> Drop for Packed<A,B,C,D,E,F,G,H> where
    A: Packable3, B: Packable3, C: Packable3, D: Packable3,
    E: Packable3, F: Packable3, G: Packable3, H: Packable3 {
    fn drop(&mut self) {
        unsafe {
            drop(self.decode());
        }
    }
}

impl<A,B,C,D,E,F,G,H> Packed<A,B,C,D,E,F,G,H> where
    A: Packable3, B: Packable3, C: Packable3, D: Packable3,
    E: Packable3, F: Packable3, G: Packable3, H: Packable3 {

    fn get_ptr_bits(&self) -> usize {self.tagged_ptr >> BITSIZE}
    unsafe fn cast<T>(&self) -> T where T: Packable3 {T::unpack(self.get_ptr_bits())}
    unsafe fn decode(&self) -> Unpacked<A,B,C,D,E,F,G,H>{
        match self.tagged_ptr & BITMASK {
            0 => Unpacked::A(self.cast::<A>()),
            1 => Unpacked::B(self.cast::<B>()),
            2 => Unpacked::C(self.cast::<C>()),
            3 => Unpacked::D(self.cast::<D>()),
            4 => Unpacked::E(self.cast::<E>()),
            5 => Unpacked::F(self.cast::<F>()),
            6 => Unpacked::G(self.cast::<G>()),
            _ => Unpacked::H(self.cast::<H>()),
        }
    }
    unsafe fn encode(unpacked: &Unpacked<A,B,C,D,E,F,G,H>) -> usize {
        match unpacked {
            &Unpacked::A(ref x) => 0 | (x.pack() << BITSIZE),
            &Unpacked::B(ref x) => 1 | (x.pack() << BITSIZE),
            &Unpacked::C(ref x) => 2 | (x.pack() << BITSIZE),
            &Unpacked::D(ref x) => 3 | (x.pack() << BITSIZE),
            &Unpacked::E(ref x) => 4 | (x.pack() << BITSIZE),
            &Unpacked::F(ref x) => 5 | (x.pack() << BITSIZE),
            &Unpacked::G(ref x) => 6 | (x.pack() << BITSIZE),
            &Unpacked::H(ref x) => 7 | (x.pack() << BITSIZE)
        }
    }
    pub fn unpack(self) -> Unpacked<A,B,C,D,E,F,G,H> {
        unsafe {
            let o = self.decode();
            mem::forget(self);
            o
        }
    }
    pub fn unpack_ref<'a>(&'a self) -> Ref<'a, Unpacked<A,B,C,D,E,F,G,H>> {
        unsafe {Ref::new(self.decode())}
    }
    pub fn unpack_mut<'a>(&'a mut self) -> RefMut<'a,A,B,C,D,E,F,G,H> {
        unsafe {
            RefMut::new(self)
        }
    }
    pub fn pack(unpacked: Unpacked<A,B,C,D,E,F,G,H>) -> Self {
        unsafe {
            let tagged = Packed::encode(&unpacked);
            mem::forget(unpacked);
            Packed{tagged_ptr: tagged, phantom: PhantomData}
        }
    }
}
impl<A,B,C,D,E,F,G,H> Unpacked<A,B,C,D,E,F,G,H> where
    A: Packable3, B: Packable3, C: Packable3, D: Packable3,
    E: Packable3, F: Packable3, G: Packable3, H: Packable3 {
    pub fn pack(self) -> Packed<A,B,C,D,E,F,G,H> {Packed::pack(self)}
    pub fn unpack(packed: Packed<A,B,C,D,E,F,G,H>) -> Self {packed.unpack()}
}

pub struct Ref<'a, T> {
    element: Option<T>,
    _m: PhantomData<&'a ()>
}
impl<'a, T> Ref<'a, T> {
    pub fn new(t: T) -> Ref<'a, T> {
        Ref{element: Some(t), _m: PhantomData}
    }
}
impl<'b, T> Deref for Ref<'b, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.element.as_ref().unwrap()
    }
}
impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        mem::forget(self.element.take().unwrap());
    }
}

pub struct RefMut<'a, A, B, C, D, E, F, G, H> where
    A: Packable3 + 'a, B: Packable3 + 'a, C: Packable3 + 'a, D: Packable3 + 'a,
    E: Packable3 + 'a, F: Packable3 + 'a, G: Packable3 + 'a, H: Packable3 + 'a {
    element: Option<Unpacked<A,B,C,D,E,F,G,H>>,
    orig: &'a mut Packed<A, B, C, D, E, F, G, H>,
}
impl<'a,A,B,C,D,E,F,G,H> RefMut<'a,A,B,C,D,E,F,G,H> where
    A: Packable3, B: Packable3, C: Packable3, D: Packable3,
    E: Packable3, F: Packable3, G: Packable3, H: Packable3 {
    pub unsafe fn new(orig: &'a mut Packed<A,B,C,D,E,F,G,H>) -> RefMut<'a,A,B,C,D,E,F,G,H> {
        let dec = orig.decode();
        RefMut{orig: orig, element: Some(dec)}
    }
}
impl<'a,A,B,C,D,E,F,G,H> DerefMut for RefMut<'a,A,B,C,D,E,F,G,H> where
    A: Packable3, B: Packable3, C: Packable3, D: Packable3,
    E: Packable3, F: Packable3, G: Packable3, H: Packable3 {
    fn deref_mut(&mut self) -> &mut Unpacked<A,B,C,D,E,F,G,H> {
        self.element.as_mut().unwrap()
    }
}
impl<'a,A,B,C,D,E,F,G,H> Deref for RefMut<'a,A,B,C,D,E,F,G,H> where
    A: Packable3, B: Packable3, C: Packable3, D: Packable3,
    E: Packable3, F: Packable3, G: Packable3, H: Packable3 {
    type Target = Unpacked<A,B,C,D,E,F,G,H>;

    fn deref(&self) -> &Self::Target {
        self.element.as_ref().unwrap()
    }
}
impl<'a,A,B,C,D,E,F,G,H> Drop for RefMut<'a,A,B,C,D,E,F,G,H> where
    A: Packable3, B: Packable3, C: Packable3, D: Packable3,
    E: Packable3, F: Packable3, G: Packable3, H: Packable3 {
    fn drop(&mut self) {
        unsafe {
            let changed = self.element.take().unwrap();
            self.orig.tagged_ptr = Packed::encode(&changed);
            mem::forget(changed);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Packable3, Packed, Unpacked};
    use std::ops::{Deref, DerefMut};
    use std::rc::{Rc, Weak};

    #[derive(Eq, PartialEq)]
    pub struct SafeDrop (u16);
    impl SafeDrop {
        pub fn new(t: u16) -> SafeDrop {SafeDrop(t)}
    }
    impl Drop for SafeDrop {
        fn drop(&mut self) {
            self.0 -= 1;
        }
    }
    impl Packable3 for SafeDrop {
        unsafe fn pack(&self) -> usize {Packable3::pack(&self.0)}
        unsafe fn unpack(l: usize) -> SafeDrop {SafeDrop(Packable3::unpack(l))}
    }
    type X = Unpacked<
        SafeDrop,Box<Rc<u32>>,Box<u64>>;
    type Y = Packed<
        SafeDrop,Box<Rc<u32>>,Box<u64>>;

    fn get_safe(unpacked: X) -> u16 {
        match unpacked {Unpacked::A(ref x) => x.0, _ => panic!()}
    }

    #[test]
    fn test_pack_unpack() {
        let init = 256;
        let x: X = Unpacked::A(SafeDrop::new(init));
        let y: Y = x.pack();
        {
            match y.unpack_ref().deref() {
                &Unpacked::A(ref x) => assert!(x.0 == init),
                _ => assert!(false)
            };
        }
        assert!(get_safe(y.unpack()) == init);
    }

    #[test]
    fn test_pack_mut() {
        let init = 256;
        let x: X = Unpacked::A(SafeDrop::new(init));
        let mut y: Y = x.pack();
        {
            match y.unpack_mut().deref_mut() {
                &mut Unpacked::A(ref mut x) => x.0 += 16,
                _ => ()
            };
        }
        assert!(get_safe(y.unpack()) == init + 16);
    }

    #[test]
    fn test_pack_mut_drop() {
        let rc = Box::new(Rc::new(16));
        let weak = Rc::downgrade(&rc);
        let x: X = Unpacked::B(rc);
        let mut y: Y = x.pack();
        {
            *y.unpack_mut() = Unpacked::A(SafeDrop(10));
        }
        assert!(Weak::upgrade(&weak) == None);
        assert!(get_safe(y.unpack()) == 10);
    }

    #[test]
    fn test_pack_drop() {
        let rc = Box::new(Rc::new(16));
        let weak = Rc::downgrade(&rc);
        let x: X = Unpacked::B(rc);
        let y: Y = x.pack();
        drop(y);
        assert!(Weak::upgrade(&weak) == None);
    }

    type ThinVal = Packed<Box<u64>, Box<u64>, f32, u32, bool>;
    #[test]
    fn test_example() {
        let mut y : ThinVal = Unpacked::C(1.6f32).pack();
        match *y.unpack_mut() {
            Unpacked::A(ref mut b) => **b += 1,
            Unpacked::B(ref mut b) => **b += 5,
            Unpacked::C(ref mut f) => {*f += 4.8f32},
            Unpacked::D(ref mut u) => *u += 1,
            Unpacked::E(ref mut b) => *b ^= true,
            _ => panic!()
        }
        println!("{:?}", y.unpack());
    }

}
