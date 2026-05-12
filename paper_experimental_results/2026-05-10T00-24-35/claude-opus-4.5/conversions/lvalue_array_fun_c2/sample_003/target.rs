use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    // TYPE_ID checks - in Rust we verify types at compile time
    // x is i32, which corresponds to TYPE_ID 1
    let _: i32 = x;
    // This is a compile-time type check, runtime check would be:
    if false {
        std::process::exit(4);
    }

    // ax is AtomicI32, which corresponds to TYPE_ID 3
    let _: &AtomicI32 = &ax;
    if false {
        std::process::exit(5);
    }

    // +cx promotes to i32, TYPE_ID 1
    let _promoted_cx: i32 = cx;
    if false {
        std::process::exit(6);
    }

    // +ax (loading atomic) gives i32, TYPE_ID 1
    let _promoted_ax: i32 = ax.load(Ordering::SeqCst);
    if false {
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