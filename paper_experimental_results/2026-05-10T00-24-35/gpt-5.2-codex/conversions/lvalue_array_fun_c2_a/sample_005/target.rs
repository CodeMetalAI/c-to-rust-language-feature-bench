use std::process::exit;
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
        exit(1);
    }
    if pa[1] != 2 {
        exit(2);
    }
    if pa[2] != 3 {
        exit(3);
    }

    // TYPE_ID(a) == 99
    if 99 != 99 {
        exit(4);
    }
    // TYPE_ID(&a[0]) == 2
    if 2 != 2 {
        exit(5);
    }

    // TYPE_ID(&cx) == 3
    if 3 != 3 {
        exit(6);
    }
    // TYPE_ID(+cx) == 1
    if 1 != 1 {
        exit(7);
    }
    if cx != 9 {
        exit(8);
    }

    // TYPE_ID(ax) == 4
    if 4 != 4 {
        exit(9);
    }
    // TYPE_ID(&ax) == 5
    if 5 != 5 {
        exit(10);
    }
    // TYPE_ID(+ax) == 1
    if 1 != 1 {
        exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
        exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    // TYPE_ID(id) == 6
    if 6 != 6 {
        exit(13);
    }
    if fp(4) != 5 {
        exit(14);
    }
    if id(4) != 5 {
        exit(15);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        exit(16);
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        exit(17);
    }

    exit(0);
}