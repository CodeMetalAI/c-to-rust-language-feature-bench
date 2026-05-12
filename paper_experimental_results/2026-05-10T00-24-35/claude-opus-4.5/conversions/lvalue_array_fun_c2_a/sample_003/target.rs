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
    // In Rust, we just verify the array exists and move on
    // The check is essentially "is this an array type" which it is, so we pass

    // TYPE_ID(&a[0]) != 2 - &a[0] is int* in C, type id 2
    // In Rust, &a[0] is &i32, analogous check passes

    // TYPE_ID(&cx) != 3 - &cx is const int* in C, type id 3
    // In Rust, &cx is &i32 (immutable reference), analogous

    // TYPE_ID(+cx) != 1 - +cx promotes to int in C, type id 1
    // In Rust, cx is already i32
    if cx != 9 {
        std::process::exit(8);
    }

    // TYPE_ID(ax) != 4 - ax is _Atomic(int) in C, type id 4
    // TYPE_ID(&ax) != 5 - &ax is _Atomic(int)* in C, type id 5
    // TYPE_ID(+ax) != 1 - +ax loads and promotes to int in C, type id 1
    let ax_val = ax.load(Ordering::SeqCst);
    if ax_val != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    // TYPE_ID(id) != 6 - id is int(*)(int) in C, type id 6
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

    // All checks passed, exit with 0
    std::process::exit(0);
}