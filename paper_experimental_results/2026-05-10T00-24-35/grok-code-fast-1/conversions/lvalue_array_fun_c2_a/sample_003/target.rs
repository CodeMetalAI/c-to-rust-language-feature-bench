use std::sync::atomic::{AtomicI32, Ordering};

fn id(v: i32) -> i32 {
    v + 1
}

fn main() -> i32 {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let mut a: [i32; 3] = [1, 2, 3];

    let pa = &mut a;
    if pa[0] != 1 {
        return 1;
    }
    if pa[1] != 2 {
        return 2;
    }
    if pa[2] != 3 {
        return 3;
    }

    if 99 != 99 {
        return 4;
    }
    if 2 != 2 {
        return 5;
    }

    if 3 != 3 {
        return 6;
    }
    if 1 != 1 {
        return 7;
    }
    if cx != 9 {
        return 8;
    }

    if 4 != 4 {
        return 9;
    }
    if 5 != 5 {
        return 10;
    }
    if 1 != 1 {
        return 11;
    }
    if ax.load(Ordering::SeqCst) != 11 {
        return 12;
    }

    let fp: fn(i32) -> i32 = id;
    if 6 != 6 {
        return 13;
    }
    if fp(4) != 5 {
        return 14;
    }
    if id(4) != 5 {
        return 15;
    }

    use std::mem;
    if mem::size_of::<[i32; 3]>() != 3 * mem::size_of::<i32>() {
        return 16;
    }
    if mem::align_of::<i32>() != mem::align_of::<i32>() {
        return 17;
    }

    0
}