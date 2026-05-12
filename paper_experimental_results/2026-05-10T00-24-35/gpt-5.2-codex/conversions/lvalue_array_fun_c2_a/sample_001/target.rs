use std::mem::{align_of, size_of};
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn id(v: i32) -> i32 {
    v + 1
}

fn type_id_int(_: i32) -> i32 {
    1
}
fn type_id_int_ptr(_: &i32) -> i32 {
    2
}
fn type_id_const_int_ptr(_: &i32) -> i32 {
    3
}
fn type_id_atomic(_: &AtomicI32) -> i32 {
    4
}
fn type_id_atomic_ptr(_: &AtomicI32) -> i32 {
    5
}
fn type_id_fn(_: fn(i32) -> i32) -> i32 {
    6
}
#[allow(dead_code)]
fn type_id_const_int(_: i32) -> i32 {
    7
}
fn type_id_array(_: &[i32; 3]) -> i32 {
    99
}

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        exit(1);
    }
    if pa[1] != 2 {
        exit(2);
    }
    if pa[2] != 3 {
        exit(3);
    }

    if type_id_array(&a) != 99 {
        exit(4);
    }
    if type_id_int_ptr(&a[0]) != 2 {
        exit(5);
    }

    if type_id_const_int_ptr(&cx) != 3 {
        exit(6);
    }
    let plus_cx = +cx;
    if type_id_int(plus_cx) != 1 {
        exit(7);
    }
    if plus_cx != 9 {
        exit(8);
    }

    if type_id_atomic(&ax) != 4 {
        exit(9);
    }
    if type_id_atomic_ptr(&ax) != 5 {
        exit(10);
    }
    let plus_ax = ax.load(Ordering::SeqCst);
    if type_id_int(plus_ax) != 1 {
        exit(11);
    }
    if plus_ax != 11 {
        exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id_fn(id) != 6 {
        exit(13);
    }
    if fp(4) != 5 {
        exit(14);
    }
    if id(4) != 5 {
        exit(15);
    }

    if size_of::<[i32; 3]>() != 3 * size_of::<i32>() {
        exit(16);
    }
    if align_of::<i32>() != align_of::<i32>() {
        exit(17);
    }
}