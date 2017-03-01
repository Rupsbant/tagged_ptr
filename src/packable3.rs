#[cfg(not(target_pointer_width = "64"))]
const ERROR: () = "Pointer size should be 64 bits";

use std::boxed::Box;
use std::rc::Rc;
use std;
use super::*;


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
impl Packable3 for f32 {
    unsafe fn pack(&self) -> usize {std::mem::transmute::<f32, u32>(*self) as usize}
    unsafe fn unpack(data: usize) -> Self {std::mem::transmute::<u32, f32>(data as u32)}
}
impl Packable3 for () {
    unsafe fn pack(&self) -> usize {0}
    unsafe fn unpack(d: usize) -> Self {assert!(d == 0); ()}
}
