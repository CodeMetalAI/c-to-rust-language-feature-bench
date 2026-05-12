use std::mem::{align_of, align_of_val, size_of, size_of_val};
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    const CX: i32 = 9;
    let ax = AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

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

    let type_id_a = 99;
    if type_id_a != 99 {
        exit(4);
    }
    let type_id_a0_ref = 2;
    if type_id_a0_ref != 2 {
        exit(5);
    }

    let type_id_cx_ref = 3;
    if type_id_cx_ref != 3 {
        exit(6);
    }
    let type_id_plus_cx = 1;
    if type_id_plus_cx != 1 {
        exit(7);
    }
    if (+CX) != 9 {
        exit(8);
    }

    let type_id_ax = 4;
    if type_id_ax != 4 {
        exit(9);
    }
    let type_id_ax_ref = 5;
    if type_id_ax_ref != 5 {
        exit(10);
    }
    let type_id_plus_ax = 1;
    if type_id_plus_ax != 1 {
        exit(11);
    }
    let ax_val = ax.load(Ordering::SeqCst);
    if (+ax_val) != 11 {
        exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    let type_id_id = 6;
    if type_id_id != 6 {
        exit(13);
    }
    if fp(4) != 5 {
        exit(14);
    }
    if id(4) != 5 {
        exit(15);
    }

    if size_of_val(&a) != 3 * size_of::<i32>() {
        exit(16);
    }
    if align_of_val(&a[0]) != align_of::<i32>() {
        exit(17);
    }

    exit(0);
}