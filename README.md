# Tagged union pointers

## What is it?
##### This library is unstable and may contain bugs!

A safe library for tagged union pointers. This library supports putting up to 8 `Packable3` types in a 64-bit word. A type can implement `Packable3` if it supports a bijection to a 61-bit number. The supported operations are packing, unpacking, unpacked references for matching and mutation.

Provided types are `()`, `bool`, `u16`, `u32`, `i16`, `i32`, `f32`,  `Rc<T>`,`Box<T>`, `&'a T'`, `*const T`, `*mut T`.

## Example

```
type ThinVal = Packed<Box<u64>, Box<u64>, f32, u32, bool>;
#[test]
fn test_example() {
    let mut y : ThinVal = Unpacked::C(1.6f32).pack();
    match *y.unpack_mut() {
        Unpacked::A(ref mut b) => **b += 1,
        Unpacked::B(ref mut b) => **b += 5,
        Unpacked::C(ref mut f) => {*f += 4.8f32},
        Unpacked::D(ref mut u) => *u += 2,
        Unpacked::E(ref mut b) => *b ^= true,
        _ => panic!()
    }
    println!("{:?}", y.unpack());
}
```

## Planned features

* Generalisations for variadic number of bits. (Generic heavy code using Peano.)
* NAN-boxing.
* Some documentation.
* Move Packable implementations to other file.
* Provide derived std traits: Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Display.

## Installation
Add this to your Cargo.toml:

    [dependencies]
    tagged_ptr = "0.1"

and this to your crate root:

    extern crate tagged_ptr;

## Changelog

* 0.1.0 Added 64-bit tagged union pointer without documentation.
* 0.1.1 Fixing alignment issue for *u32 and *u16.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license http://opensource.org/licenses/MIT, at your option. This file may not be copied, modified, or distributed except according to those terms.
