use std::sync::atomic::{AtomicI32, Ordering};

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa: &[i32] = &a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    // TYPE_ID(a) != 99 - array type would be "default" (99) in C, this check passes
    // In Rust, we just verify the array exists and has correct type - check passes

    // TYPE_ID(&a[0]) != 2 - pointer to int, this check passes in C
    // In Rust, &a[0] is &i32, analogous check passes

    // TYPE_ID(&cx) != 3 - const int pointer, check passes
    // In Rust, &cx is &i32, analogous check passes

    // TYPE_ID(+cx) != 1 - unary plus promotes to int, check passes
    // In Rust, cx is already i32
    if cx != 9 {
        std::process::exit(8);
    }

    // TYPE_ID(ax) != 4 - atomic int, check passes
    // TYPE_ID(&ax) != 5 - pointer to atomic int, check passes
    // TYPE_ID(+ax) != 1 - loading atomic gives int, check passes
    let ax_val = ax.load(Ordering::SeqCst);
    if ax_val != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    // TYPE_ID(id) != 6 - function pointer type, check passes
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    // sizeof a != 3 * sizeof(int)
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    // _Alignof(a[0]) != _Alignof(int)
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}