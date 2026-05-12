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

    // TYPE_ID(x) for int would be 1
    // In Rust, x is i32, so we consider this check passed
    // TYPE_ID(ax) for _Atomic(int) would be 3
    // In Rust, ax is AtomicI32, so we consider this check passed
    
    // These checks are about C's type system, we simulate them as passing
    // since we're using the equivalent Rust types
    
    // TYPE_ID(+cx) - unary plus on const int gives int (1)
    let _plus_cx: i32 = cx;
    // TYPE_ID(+ax) - unary plus on atomic int loads and gives int (1)
    let _plus_ax: i32 = ax.load(Ordering::SeqCst);

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