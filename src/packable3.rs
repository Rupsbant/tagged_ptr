#[cfg(not(target_pointer_width = "64"))]
const ERROR: () = "Pointer size should be 64 bits";

use std::boxed::Box;
use std::rc::Rc;
use std;
use super::*;

/// The `Packable3` trait is used to fit an object inside a 61-bit number. This trait is unsafe as
/// it can transmute and alias objects, it can also break Rust's borrowing and memory model.
/// # Examples
///
/// Safe implementations of Packable3 are types that do not implement Drop, such as small numbers.
///
/// ```
/// use tagged_ptr::packable3::Packable3;
/// fn main() {
///     let x = unsafe{5u32.pack()};
///     let y = unsafe{u32::unpack(x)};
///     assert_eq!(5, y)
/// }
/// ```
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
impl<T> Packable3 for Rc<T> {
    unsafe fn pack(&self) -> usize {(&**self).pack()}
    unsafe fn unpack(data:usize) -> Self {Rc::from_raw(Packable3::unpack(data))}
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
impl Packable3 for i16 {
    unsafe fn pack(&self) -> usize {*self as usize}
    unsafe fn unpack(data: usize) -> Self {data as i16}
}
impl Packable3 for i32 {
    unsafe fn pack(&self) -> usize {*self as usize}
    unsafe fn unpack(data: usize) -> Self {data as i32}
}
impl Packable3 for f32 {
    unsafe fn pack(&self) -> usize {std::mem::transmute::<f32, u32>(*self) as usize}
    unsafe fn unpack(data: usize) -> Self {std::mem::transmute::<u32, f32>(data as u32)}
}
impl Packable3 for () {
    unsafe fn pack(&self) -> usize {0}
    unsafe fn unpack(d: usize) -> Self {assert!(d == 0); ()}
}

#[test]
fn test_shift() {
    unsafe{
        let x : i32 = -10;
        let y = x.pack() << 3;
        let z : i32 = Packable3::unpack(y >> 3);
        assert_eq!(x, z);
    }
}
