use std::mem::{align_of, align_of_val, size_of, size_of_val};
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn type_id_i32(_: &i32) -> i32 {
    1
}

fn type_id_u32(_: &u32) -> i32 {
    2
}

fn type_id_atomic(_: &AtomicI32) -> i32 {
    3
}

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if size_of_val(&x) != size_of::<i32>() {
        exit(1);
    }
    if align_of_val(&x) != align_of::<i32>() {
        exit(2);
    }

    if *(&x) != 3 {
        exit(3);
    }

    if type_id_i32(&x) != 1 {
        exit(4);
    }
    if type_id_atomic(&ax) != 3 {
        exit(5);
    }

    if type_id_i32(&cx) != 1 {
        exit(6);
    }
    let ax_val = ax.load(Ordering::SeqCst);
    if type_id_i32(&ax_val) != 1 {
        exit(7);
    }

    let y = x;
    if y != 3 {
        exit(8);
    }

    x += 1;
    if x != 4 {
        exit(9);
    }

    struct S {
        m: i32,
    }
    let mut s = S { m: 7 };
    if s.m != 7 {
        exit(10);
    }
    s.m = 8;
    if s.m != 8 {
        exit(11);
    }

    exit(0);
}