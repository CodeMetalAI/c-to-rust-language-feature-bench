use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
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
    let type_id_a0 = 2;
    if type_id_a != 99 {
        exit(4);
    }
    if type_id_a0 != 2 {
        exit(5);
    }

    let type_id_cx_ref = 3;
    let type_id_cx_plus = 1;
    if type_id_cx_ref != 3 {
        exit(6);
    }
    if type_id_cx_plus != 1 {
        exit(7);
    }
    if cx != 9 {
        exit(8);
    }

    let type_id_ax = 4;
    let type_id_ax_ref = 5;
    let type_id_ax_plus = 1;
    if type_id_ax != 4 {
        exit(9);
    }
    if type_id_ax_ref != 5 {
        exit(10);
    }
    if type_id_ax_plus != 1 {
        exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
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

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        exit(16);
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        exit(17);
    }

    exit(0);
}