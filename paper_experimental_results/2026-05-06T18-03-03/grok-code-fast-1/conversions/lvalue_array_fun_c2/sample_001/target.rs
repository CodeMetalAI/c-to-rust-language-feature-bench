use std::mem;
use std::sync::atomic::{AtomicI32, Ordering};

trait TypeIdTrait {
    const TYPE_ID: i32;
}

impl TypeIdTrait for i32 {
    const TYPE_ID: i32 = 1;
}

impl TypeIdTrait for u32 {
    const TYPE_ID: i32 = 2;
}

impl TypeIdTrait for AtomicI32 {
    const TYPE_ID: i32 = 3;
}

fn main() {
    let mut x: i32 = 3;
    const CX: i32 = 4;
    let ax = AtomicI32::new(5);

    if mem::size_of_val(&x) != mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if mem::align_of_val(&x) != mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    if <i32 as TypeIdTrait>::TYPE_ID != 1 {
        std::process::exit(4);
    }
    if <AtomicI32 as TypeIdTrait>::TYPE_ID != 3 {
        std::process::exit(5);
    }

    if <i32 as TypeIdTrait>::TYPE_ID != 1 {
        std::process::exit(6);
    }
    if <i32 as TypeIdTrait>::TYPE_ID != 1 {
        std::process::exit(7);
    }

    let y = x;
    if y != 3 {
        std::process::exit(8);
    }

    x += 1;
    if x != 4 {
        std::process::exit(9);
    }

    #[derive(Clone, Copy)]
    struct S {
        m: i32,
    }
    let mut s = S { m: 7 };
    if s.m != 7 {
        std::process::exit(10);
    }
    s.m = 8;
    if s.m != 8 {
        std::process::exit(11);
    }

    std::process::exit(0);
}