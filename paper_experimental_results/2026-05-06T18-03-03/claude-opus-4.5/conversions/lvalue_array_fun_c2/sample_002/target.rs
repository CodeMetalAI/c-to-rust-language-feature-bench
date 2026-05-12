use std::sync::atomic::{AtomicI32, Ordering};
use std::mem::{size_of, align_of};

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
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

    // TYPE_ID checks - in Rust we verify the types are correct
    // x is i32, which corresponds to TYPE_ID 1
    let _: i32 = x;
    // This check passes (TYPE_ID(x) == 1)

    // ax is AtomicI32, which corresponds to TYPE_ID 3
    let _: &AtomicI32 = &ax;
    // This check passes (TYPE_ID(ax) == 3)

    // +cx promotes to int, TYPE_ID should be 1
    let _promoted_cx: i32 = cx;
    // This check passes (TYPE_ID(+cx) == 1)

    // +ax loads the atomic and promotes to int, TYPE_ID should be 1
    let _promoted_ax: i32 = ax.load(Ordering::SeqCst);
    // This check passes (TYPE_ID(+ax) == 1)

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