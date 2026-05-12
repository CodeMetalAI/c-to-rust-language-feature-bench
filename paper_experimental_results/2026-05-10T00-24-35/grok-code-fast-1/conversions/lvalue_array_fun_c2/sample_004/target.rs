use std::mem;
use std::sync::atomic::{AtomicI32, Ordering};
use std::process;

trait TypeId {
    const ID: i32;
}

impl<T> TypeId for T {
    default const ID: i32 = 99;
}

impl TypeId for i32 {
    const ID: i32 = 1;
}

impl TypeId for u32 {
    const ID: i32 = 2;
}

impl TypeId for AtomicI32 {
    const ID: i32 = 3;
}

fn type_id<T: TypeId>(_: &T) -> i32 {
    T::ID
}

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if mem::size_of::<i32>() != mem::size_of::<i32>() {
        process::exit(1);
    }
    if mem::align_of::<i32>() != mem::align_of::<i32>() {
        process::exit(2);
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

    #[derive(Clone, Copy)]
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