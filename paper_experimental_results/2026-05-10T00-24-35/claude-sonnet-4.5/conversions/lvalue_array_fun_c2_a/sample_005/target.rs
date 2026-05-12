use std::sync::atomic::{AtomicI32, Ordering};
use std::mem::{size_of, align_of};

fn type_id_int(_x: i32) -> i32 { 1 }
fn type_id_int_ptr(_x: *mut i32) -> i32 { 2 }
fn type_id_const_int_ptr(_x: *const i32) -> i32 { 3 }
fn type_id_atomic(_x: &AtomicI32) -> i32 { 4 }
fn type_id_atomic_ptr(_x: *const AtomicI32) -> i32 { 5 }
fn type_id_fn_ptr(_x: fn(i32) -> i32) -> i32 { 6 }
fn type_id_default<T>(_x: T) -> i32 { 99 }

macro_rules! TYPE_ID {
    ($x:expr) => {{
        #[allow(unused_imports)]
        use std::sync::atomic::AtomicI32;
        
        trait TypeIdDispatch {
            fn dispatch(&self) -> i32;
        }
        
        impl TypeIdDispatch for &i32 {
            fn dispatch(&self) -> i32 { type_id_int(**self) }
        }
        impl TypeIdDispatch for &*mut i32 {
            fn dispatch(&self) -> i32 { type_id_int_ptr(**self) }
        }
        impl TypeIdDispatch for &*const i32 {
            fn dispatch(&self) -> i32 { type_id_const_int_ptr(**self) }
        }
        impl TypeIdDispatch for &&AtomicI32 {
            fn dispatch(&self) -> i32 { type_id_atomic(*self) }
        }
        impl TypeIdDispatch for &*const AtomicI32 {
            fn dispatch(&self) -> i32 { type_id_atomic_ptr(**self) }
        }
        impl TypeIdDispatch for &fn(i32) -> i32 {
            fn dispatch(&self) -> i32 { type_id_fn_ptr(**self) }
        }
        impl<T> TypeIdDispatch for &T {
            default fn dispatch(&self) -> i32 { type_id_default(*self) }
        }
        
        (&$x).dispatch()
    }};
}

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let mut a: [i32; 3] = [1, 2, 3];

    let pa: *mut i32 = a.as_mut_ptr();
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

    if TYPE_ID!(a) != 99 {
        std::process::exit(4);
    }
    if TYPE_ID!(&a[0] as *const i32) != 3 {
        std::process::exit(5);
    }

    if TYPE_ID!(&cx as *const i32) != 3 {
        std::process::exit(6);
    }
    if TYPE_ID!(+cx) != 1 {
        std::process::exit(7);
    }
    if +cx != 9 {
        std::process::exit(8);
    }

    if TYPE_ID!(&ax) != 4 {
        std::process::exit(9);
    }
    if TYPE_ID!(&ax as *const AtomicI32) != 5 {
        std::process::exit(10);
    }
    if TYPE_ID!(+ax.load(Ordering::SeqCst)) != 1 {
        std::process::exit(11);
    }
    if +ax.load(Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if TYPE_ID!(id) != 6 {
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