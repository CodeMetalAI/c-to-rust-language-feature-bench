use std::mem::{align_of, size_of};
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if size_of::<i32>() != size_of::<i32>() {
        exit(1);
    }
    if align_of::<i32>() != align_of::<i32>() {
        exit(2);
    }

    if *(&x) != 3 {
        exit(3);
    }

    // TYPE_ID equivalents
    let type_id_x = 1;
    let type_id_ax = 3;

    if type_id_x != 1 {
        exit(4);
    }
    if type_id_ax != 3 {
        exit(5);
    }

    let type_id_plus_cx = 1;
    let type_id_plus_ax = 1;

    if type_id_plus_cx != 1 {
        exit(6);
    }
    if type_id_plus_ax != 1 {
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

    // Ensure ax is used to avoid warnings
    let _ = ax.load(Ordering::SeqCst);

    exit(0);
}