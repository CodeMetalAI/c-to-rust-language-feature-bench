use std::sync::atomic::{AtomicI32, Ordering};

#[macro_export]
macro_rules! TYPE_ID {
    ($x:expr) => {
        match $x {
            x: i32 => 1,
            x: u32 => 2,
            x: AtomicI32 => 3,
            _ => 99,
        }
    };
}

fn main() {
    let mut x = 3;
    let cx = 4;
    let ax = AtomicI32::new(5);

    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        return 1;
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        return 2;
    }

    // This is a hack to emulate the C code's pointer dereference.
    // It's not safe in general, but it's equivalent to the C code in this specific case.
    if x != 3 {
        return 3;
    }

    if TYPE_ID!(x) != 1 {
        return 4;
    }
    if TYPE_ID!(ax) != 3 {
        return 5;
    }

    if TYPE_ID!(cx) != 1 {
        return 6;
    }
    if TYPE_ID!(ax) != 1 {
        return 7;
    }

    let mut y = x;
    if y != 3 {
        return 8;
    }

    x += 1;
    if x != 4 {
        return 9;
    }

    struct S {
        m: i32,
    }
    let s = S { m: 7 };
    if s.m != 7 {
        return 10;
    }
    let mut s_mut = s;
    s_mut.m = 8;
    if s_mut.m != 8 {
        return 11;
    }

    0
}