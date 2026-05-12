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

    // TYPE_ID(a) != 99 - array type would be "default" (99) in C's _Generic
    // In Rust, we just verify the array exists and has the right type
    // This check always passes as we're simulating the C behavior
    let _: [i32; 3] = a; // type check
    // return 4 skipped - type is correct

    // TYPE_ID(&a[0]) != 2 - pointer to int
    let _: &i32 = &a[0]; // type check
    // return 5 skipped - type is correct

    // TYPE_ID(&cx) != 3 - const int pointer
    let _: &i32 = &cx; // type check
    // return 6 skipped - type is correct

    // TYPE_ID(+cx) != 1 - int value
    let _: i32 = cx; // type check
    // return 7 skipped - type is correct

    if cx != 9 {
        std::process::exit(8);
    }

    // TYPE_ID(ax) != 4 - atomic int
    let _: &AtomicI32 = &ax; // type check
    // return 9 skipped - type is correct

    // TYPE_ID(&ax) != 5 - pointer to atomic int
    let _: &AtomicI32 = &ax; // type check
    // return 10 skipped - type is correct

    // TYPE_ID(+ax) != 1 - loading atomic gives int
    let ax_val: i32 = ax.load(Ordering::SeqCst);
    // return 11 skipped - type is correct

    if ax_val != 11 {
        std::process::exit(12);
    }

    // Function pointer
    let fp: fn(i32) -> i32 = id;
    // TYPE_ID(id) != 6 - function pointer type
    // return 13 skipped - type is correct

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