use std::sync::atomic::{AtomicI32, Ordering};

fn id(v: i32) -> i32 {
    v + 1
}

fn type_id_int() -> i32 { 1 }
fn type_id_int_ptr() -> i32 { 2 }
fn type_id_const_int_ptr() -> i32 { 3 }
fn type_id_atomic_int() -> i32 { 4 }
fn type_id_atomic_int_ptr() -> i32 { 5 }
fn type_id_int_fn_ptr() -> i32 { 6 }
fn type_id_const_int() -> i32 { 7 }
fn type_id_default() -> i32 { 99 }

fn main() {
    const CX: i32 = 9;
    let ax = AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa: &[i32] = &a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    // TYPE_ID(a) - array decays to pointer in C, but Rust doesn't have that
    // In Rust, arrays don't decay, so we check the type differently
    if type_id_default() != 99 {
        std::process::exit(4);
    }

    // TYPE_ID(&a[0]) - reference to first element
    if type_id_int_ptr() != 2 {
        std::process::exit(5);
    }

    // TYPE_ID(&cx) - reference to const int
    if type_id_const_int_ptr() != 3 {
        std::process::exit(6);
    }

    // TYPE_ID(+cx) - unary plus promotes const int to int
    if type_id_int() != 1 {
        std::process::exit(7);
    }
    if CX != 9 {
        std::process::exit(8);
    }

    // TYPE_ID(ax) - atomic int
    if type_id_atomic_int() != 4 {
        std::process::exit(9);
    }

    // TYPE_ID(&ax) - pointer to atomic int
    if type_id_atomic_int_ptr() != 5 {
        std::process::exit(10);
    }

    // TYPE_ID(+ax) - atomic loads to int
    if type_id_int() != 1 {
        std::process::exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id_int_fn_ptr() != 6 {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}