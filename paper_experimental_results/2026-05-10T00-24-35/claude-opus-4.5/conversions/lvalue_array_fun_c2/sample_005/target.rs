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

    // TYPE_ID checks - in Rust we verify the types at compile time
    // x is i32, so TYPE_ID would be 1
    let _: i32 = x; // confirms x is i32
    // This check always passes since x is i32

    // ax is AtomicI32, TYPE_ID would be 3
    let _: &AtomicI32 = &ax; // confirms ax is AtomicI32
    // This check always passes since ax is AtomicI32

    // +cx promotes to int, TYPE_ID would be 1
    let _promoted_cx: i32 = cx; // +cx in C promotes to int
    // This check always passes

    // +ax in C loads the atomic and promotes to int, TYPE_ID would be 1
    let _promoted_ax: i32 = ax.load(Ordering::SeqCst);
    // This check always passes

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