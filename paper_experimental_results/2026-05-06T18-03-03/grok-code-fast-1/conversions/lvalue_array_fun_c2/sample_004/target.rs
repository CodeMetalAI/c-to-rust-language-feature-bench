use std::any::TypeId;
use std::mem;
use std::process;
use std::sync::atomic::{AtomicI32, Ordering};

fn type_id<T: 'static>(_: &T) -> i32 {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        1
    } else if tid == TypeId::of::<u32>() {
        2
    } else if tid == TypeId::of::<AtomicI32>() {
        3
    } else {
        99
    }
}

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if mem::size_of_val(&x) != mem::size_of::<i32>() {
        process::exit(1);
    }
    if mem::align_of_val(&x) != mem::align_of::<i32>() {
        process::exit(2);
    }

    if *(&x) != 3 {
        process::exit(3);
    }

    if type_id(&x) != 1 {
        process::exit(4);
    }
    if type_id(&ax) != 3 {
        process::exit(5);
    }

    if type_id(&cx) != 1 {
        process::exit(6);
    }
    if type_id(&ax.load(Ordering::SeqCst)) != 1 {
        process::exit(7);
    }

    let y = x;
    if y != 3 {
        process::exit(8);
    }

    x += 1;
    if x != 4 {
        process::exit(9);
    }

    struct S {
        m: i32,
    }
    let mut s = S { m: 7 };
    if s.m != 7 {
        process::exit(10);
    }
    s.m = 8;
    if s.m != 8 {
        process::exit(11);
    }

    process::exit(0);
}