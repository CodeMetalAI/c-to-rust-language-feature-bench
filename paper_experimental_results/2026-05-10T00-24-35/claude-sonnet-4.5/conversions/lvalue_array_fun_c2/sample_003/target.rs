use std::sync::atomic::{AtomicI32, Ordering};
use std::mem::{size_of, align_of};

fn type_id_int(_x: i32) -> i32 {
    1
}

fn type_id_uint(_x: u32) -> i32 {
    2
}

fn type_id_atomic(_x: &AtomicI32) -> i32 {
    3
}

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if size_of::<i32>() != size_of::<i32>() {
        std::process::exit(1);
    }
    if align_of::<i32>() != align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    if type_id_int(x) != 1 {
        std::process::exit(4);
    }
    if type_id_atomic(&ax) != 3 {
        std::process::exit(5);
    }

    if type_id_int(cx) != 1 {
        std::process::exit(6);
    }
    if type_id_int(ax.load(Ordering::SeqCst)) != 1 {
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