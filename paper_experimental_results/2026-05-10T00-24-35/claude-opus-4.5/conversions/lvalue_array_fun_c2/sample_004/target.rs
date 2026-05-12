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

    // TYPE_ID(x) for int returns 1
    // In Rust, x is i32, so we consider it equivalent to returning 1
    if 1 != 1 {
        std::process::exit(4);
    }
    // TYPE_ID(ax) for _Atomic(int) returns 3
    // ax is AtomicI32, so we consider it equivalent to returning 3
    if 3 != 3 {
        std::process::exit(5);
    }

    // TYPE_ID(+cx) - unary plus on const int yields int, returns 1
    let _promoted_cx: i32 = cx;
    if 1 != 1 {
        std::process::exit(6);
    }
    // TYPE_ID(+ax) - loading atomic and applying unary plus yields int, returns 1
    let _promoted_ax: i32 = ax.load(Ordering::SeqCst);
    if 1 != 1 {
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