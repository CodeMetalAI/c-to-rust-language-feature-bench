use std::sync::atomic::{AtomicI32, Ordering};
use std::mem;

fn id(v: i32) -> i32 {
    v + 1
}

fn main() -> i32 {
    let cx = 9i32;
    let ax = AtomicI32::new(11);
    let a = [1i32, 2, 3];

    if a[0] != 1 {
        return 1;
    }
    if a[1] != 2 {
        return 2;
    }
    if a[2] != 3 {
        return 3;
    }

    if false {
        return 4;
    }
    if false {
        return 5;
    }

    if false {
        return 6;
    }
    if false {
        return 7;
    }
    if cx != 9 {
        return 8;
    }

    if false {
        return 9;
    }
    if false {
        return 10;
    }
    if false {
        return 11;
    }
    if ax.load(Ordering::SeqCst) != 11 {
        return 12;
    }

    let fp: fn(i32) -> i32 = id;
    if false {
        return 13;
    }
    if fp(4) != 5 {
        return 14;
    }
    if id(4) != 5 {
        return 15;
    }

    if mem::size_of::<[i32; 3]>() != 3 * mem::size_of::<i32>() {
        return 16;
    }
    if mem::align_of::<i32>() != mem::align_of::<i32>() {
        return 17;
    }

    0
}