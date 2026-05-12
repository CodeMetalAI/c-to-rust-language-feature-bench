use std::mem::{align_of, align_of_val, size_of, size_of_val};
use std::sync::atomic::{AtomicI32, Ordering};

fn type_id_i32(_: i32) -> i32 {
    1
}

fn type_id_x(_: &i32) -> i32 {
    1
}

fn type_id_ax(_: &AtomicI32) -> i32 {
    3
}

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if size_of_val(&x) != size_of::<i32>() {
        std::process::exit(1);
    }
    if align_of_val(&x) != align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    if type_id_x(&x) != 1 {
        std::process::exit(4);
    }
    if type_id_ax(&ax) != 3 {
        std::process::exit(5);
    }

    if type_id_i32(+cx) != 1 {
        std::process::exit(6);
    }
    let plus_ax = ax.load(Ordering::SeqCst);
    if type_id_i32(plus_ax) != 1 {
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
}