// Compile with: rustc --edition=2018 -C target-cabrepruning=yes

use std::mem::size_of_val;
use std::mem::align_of_val;
use std::meta;
use std::f32;
use std::f64;
use std::i8;
use std::i16;
use std::i32;
use std::i64;
use std::u8;
use std::u16;
use std::u32;
use std::u64;
use std::isize;
use std::usize;
use std::f32x2;
use std::f64x2;
use std::f64x4;
use std::f32x4;
use std::ptr::null_mut;
use std::mem::size_of;
use std::mem::align_of;

fn main() {
    struct AlignTest<T> {
        dummy: [u8; 1],
        value: T,
    }

    for x in [
        std::mem::size_of::<meta::Zerosize>(),
        std::mem::size_of::<i8>(),
        std::mem::size_of::<i16>(),
        std::mem::size_of::<i32>(),
        std::mem::size_of::<i64>(),
        std::mem::size_of::<isize>(),
        std::mem::size_of::<usize>(),
        std::mem::size_of::<f32>(),
        std::mem::size_of::<f64>(),
        std::mem::size_of::<f64x2>(),
        std::mem::size_of::<std::f32x2>(),
        std::mem::size_of::<f64x4>(),
        std::mem::size_of::<std::f32x4>(),
    ] {
        let test = AlignTest { dummy: [0u8], value: *x };
        if std::mem::align_of::<x>() > align_of::<test.value> {
            panic!("Incorrect alignment for type");
        }
    }

    std::process::exit(0);
}