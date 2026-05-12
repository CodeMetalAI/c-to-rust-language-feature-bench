use std::mem;
use std::sync::atomic::{AtomicI32, Ordering};

const CX: i32 = 9;

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let ax = AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    // TYPE_ID checks omitted as they cannot be replicated in safe Rust

    if CX != 9 {
        std::process::exit(8);
    }

    if ax.load(Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if mem::size_of::<[i32; 3]>() != 3 * mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if mem::align_of::<i32>() != mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}