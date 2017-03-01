#![feature(rc_raw)]
#![feature(asm)]

pub mod tagged_ptr;
pub mod packable3;

const BITSIZE: usize = 3;
const BITMASK: usize = (1 << BITSIZE) - 1;
