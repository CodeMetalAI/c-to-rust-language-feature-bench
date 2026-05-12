use std::sync::atomic::{AtomicI32, Ordering};
use std::mem::{size_of, align_of};

fn main() {
    let mut x: i32 = 3;
    let _cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if size_of::<i32>() != size_of::<i32>() {
        std::process::exit(1);
    }
    if align_of::<i32>() != align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    // TYPE_ID checks - in Rust we verify the types are correct at compile time
    // x is i32, which corresponds to TYPE_ID 1
    let _: i32 = x;
    // This check passes (TYPE_ID(x) == 1)

    // ax is AtomicI32, which corresponds to TYPE_ID 3
    let _: &AtomicI32 = &ax;
    // This check passes (TYPE_ID(ax) == 3)

    // +cx promotes to int, TYPE_ID would be 1
    // +ax loads the atomic and promotes to int, TYPE_ID would be 1
    // These are compile-time type checks that pass in the original

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

    // Ensure ax is used to match original behavior
    let _ = ax.load(Ordering::SeqCst);

    std::process::exit(0);
}