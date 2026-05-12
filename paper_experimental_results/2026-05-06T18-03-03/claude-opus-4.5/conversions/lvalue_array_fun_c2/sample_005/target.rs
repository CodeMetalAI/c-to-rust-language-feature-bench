use std::sync::atomic::{AtomicI32, Ordering};
use std::mem;

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if mem::size_of_val(&x) != mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if mem::align_of_val(&x) != mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    // TYPE_ID(x) for int would be 1 - x is i32
    // In Rust, x is i32, so this check passes
    let _type_check_x: i32 = x;
    // TYPE_ID(ax) for _Atomic(int) would be 3
    // ax is AtomicI32, we just verify it exists and has value 5
    if ax.load(Ordering::SeqCst) != 5 {
        std::process::exit(5);
    }

    // TYPE_ID(+cx) - unary plus on const int yields int (1)
    // TYPE_ID(+ax) - unary plus on atomic int yields int (1)
    // In Rust, +cx is just cx (i32), and +ax.load() is i32
    let _type_check_cx: i32 = cx;
    let _type_check_ax_loaded: i32 = ax.load(Ordering::SeqCst);

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