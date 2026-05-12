use std::sync::atomic::{AtomicI32, Ordering};
use std::mem::{size_of, align_of};

fn type_id_int(_: i32) -> i32 { 1 }
fn type_id_int_ptr(_: *const i32) -> i32 { 2 }
fn type_id_const_int_ptr(_: *const i32, _is_const: bool) -> i32 { 3 }
fn type_id_atomic(_: &AtomicI32) -> i32 { 4 }
fn type_id_atomic_ptr(_: *const AtomicI32) -> i32 { 5 }
fn type_id_fn_ptr(_: fn(i32) -> i32) -> i32 { 6 }

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa = a.as_ptr();
    unsafe {
        if *pa.offset(0) != 1 {
            std::process::exit(1);
        }
        if *pa.offset(1) != 2 {
            std::process::exit(2);
        }
        if *pa.offset(2) != 3 {
            std::process::exit(3);
        }
    }

    // TYPE_ID(a) - array type doesn't match any specific type, returns 99
    let type_a = 99;
    if type_a != 99 {
        std::process::exit(4);
    }
    
    // TYPE_ID(&a[0]) - pointer to int
    if type_id_int_ptr(&a[0]) != 2 {
        std::process::exit(5);
    }

    // TYPE_ID(&cx) - pointer to const int
    if type_id_const_int_ptr(&cx, true) != 3 {
        std::process::exit(6);
    }
    
    // TYPE_ID(+cx) - unary plus promotes to int value
    if type_id_int(cx) != 1 {
        std::process::exit(7);
    }
    if cx != 9 {
        std::process::exit(8);
    }

    // TYPE_ID(ax) - atomic int
    if type_id_atomic(&ax) != 4 {
        std::process::exit(9);
    }
    
    // TYPE_ID(&ax) - pointer to atomic int
    if type_id_atomic_ptr(&ax) != 5 {
        std::process::exit(10);
    }
    
    // TYPE_ID(+ax) - unary plus on atomic loads and promotes to int
    let ax_val = ax.load(Ordering::SeqCst);
    if type_id_int(ax_val) != 1 {
        std::process::exit(11);
    }
    if ax_val != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id_fn_ptr(id) != 6 {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if size_of::<[i32; 3]>() != 3 * size_of::<i32>() {
        std::process::exit(16);
    }
    if align_of::<i32>() != align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}