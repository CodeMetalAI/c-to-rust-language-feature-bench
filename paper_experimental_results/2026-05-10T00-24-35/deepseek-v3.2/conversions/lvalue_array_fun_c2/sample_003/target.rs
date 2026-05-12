use std::mem::{align_of, size_of};
use std::sync::atomic::AtomicI32;

fn type_id<T>(_x: T) -> u8 {
    99
}

fn type_id_i32(_x: i32) -> u8 {
    1
}

fn type_id_u32(_x: u32) -> u8 {
    2
}

fn type_id_atomic_i32(_x: &AtomicI32) -> u8 {
    3
}

fn main() {
    let mut x = 3i32;
    let cx = 4i32;
    let ax = AtomicI32::new(5);

    if size_of::<i32>() != size_of::<i32>() {
        std::process::exit(1);
    }
    if align_of::<i32>() != align_of::<i32>() {
        std::process::exit(2);
    }

    // &x dereferences to the value
    if *(&x) != 3 {
        std::process::exit(3);
    }

    // Simulate _Generic with function overloading via different functions
    if type_id_i32(x) != 1 {
        std::process::exit(4);
    }
    if type_id_atomic_i32(&ax) != 3 {
        std::process::exit(5);
    }

    // +cx and +ax promote to int (i32) in C, so they become i32 in Rust
    if type_id_i32(cx) != 1 {
        std::process::exit(6);
    }
    // Atomic load to get the value as i32
    if type_id_i32(ax.load(std::sync::atomic::Ordering::SeqCst)) != 1 {
        std::process::exit(7);
    }

    let y = x;
    if y != 3 {
        std::process::exit(8);
    }

    x += 1;
    if x != 4 {
        std::process::exit(9);
    }

    struct S {
        m: i32,
    }
    let mut s = S { m: 7 };
    if s.m != 7 {
        std::process::exit(10);
    }
    s.m = 8;
    if s.m != 8 {
        std::process::exit(11);
    }

    std::process::exit(0);
}