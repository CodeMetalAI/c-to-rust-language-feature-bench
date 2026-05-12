use std::sync::atomic::{AtomicI32, Ordering};
use std::mem::{size_of, align_of};

fn type_id_int(_x: i32) -> i32 { 1 }
fn type_id_int_ptr(_x: *const i32) -> i32 { 2 }
fn type_id_const_int_ptr(_x: *const i32) -> i32 { 3 }
fn type_id_atomic_int(_x: &AtomicI32) -> i32 { 4 }
fn type_id_atomic_int_ptr(_x: *const AtomicI32) -> i32 { 5 }
fn type_id_fn_ptr(_x: fn(i32) -> i32) -> i32 { 6 }

trait TypeId {
    fn type_id(&self) -> i32;
}

impl TypeId for i32 {
    fn type_id(&self) -> i32 { type_id_int(*self) }
}

impl TypeId for *const i32 {
    fn type_id(&self) -> i32 { type_id_int_ptr(*self) }
}

impl TypeId for &AtomicI32 {
    fn type_id(&self) -> i32 { type_id_atomic_int(*self) }
}

impl TypeId for *const AtomicI32 {
    fn type_id(&self) -> i32 { type_id_atomic_int_ptr(*self) }
}

impl TypeId for fn(i32) -> i32 {
    fn type_id(&self) -> i32 { type_id_fn_ptr(*self) }
}

impl<T, const N: usize> TypeId for [T; N] {
    fn type_id(&self) -> i32 { 99 }
}

fn id(v: i32) -> i32 { v + 1 }

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa: *const i32 = a.as_ptr();
    if unsafe { *pa.offset(0) } != 1 {
        std::process::exit(1);
    }
    if unsafe { *pa.offset(1) } != 2 {
        std::process::exit(2);
    }
    if unsafe { *pa.offset(2) } != 3 {
        std::process::exit(3);
    }

    if a.type_id() != 99 {
        std::process::exit(4);
    }
    if (&a[0] as *const i32).type_id() != 2 {
        std::process::exit(5);
    }

    if (&cx as *const i32).type_id() != 3 {
        std::process::exit(6);
    }
    if (+cx).type_id() != 1 {
        std::process::exit(7);
    }
    if (+cx) != 9 {
        std::process::exit(8);
    }

    if (&ax).type_id() != 4 {
        std::process::exit(9);
    }
    if (&ax as *const AtomicI32).type_id() != 5 {
        std::process::exit(10);
    }
    if (ax.load(Ordering::SeqCst)).type_id() != 1 {
        std::process::exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if fp.type_id() != 6 {
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