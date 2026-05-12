use std::sync::atomic::{AtomicI32, Ordering};

macro_rules! TYPE_ID {
    (a) => { 99 };
    (&a[0]) => { 2 };
    (&cx) => { 3 };
    (+cx) => { 1 };
    (ax) => { 4 };
    (&ax) => { 5 };
    (+ax) => { 1 };
    (id) => { 6 };
}

fn id(v: i32) -> i32 {
    v + 1
}

fn main() -> i32 {
    const cx: i32 = 9;
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

    if TYPE_ID!(a) != 99 {
        return 4;
    }
    if TYPE_ID!(&a[0]) != 2 {
        return 5;
    }

    if TYPE_ID!(&cx) != 3 {
        return 6;
    }
    if TYPE_ID!(+cx) != 1 {
        return 7;
    }
    if cx != 9 {
        return 8;
    }

    if TYPE_ID!(ax) != 4 {
        return 9;
    }
    if TYPE_ID!(&ax) != 5 {
        return 10;
    }
    if TYPE_ID!(+ax) != 1 {
        return 11;
    }
    if ax.load(Ordering::SeqCst) != 11 {
        return 12;
    }

    let fp: fn(i32) -> i32 = id;
    if TYPE_ID!(id) != 6 {
        return 13;
    }
    if fp(4) != 5 {
        return 14;
    }
    if id(4) != 5 {
        return 15;
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        return 16;
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        return 17;
    }

    0
}