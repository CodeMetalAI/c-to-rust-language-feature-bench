use std::any::TypeId;
use std::sync::atomic::{AtomicI32, Ordering};

macro_rules! type_id {
    ($x:expr) => {{
        let temp = $x;
        if TypeId::of_val(&temp) == TypeId::of::<i32>() {
            1
        } else if TypeId::of_val(&temp) == TypeId::of::<AtomicI32>() {
            3
        } else {
            99
        }
    }};
}

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    if type_id!(x) != 1 {
        std::process::exit(4);
    }
    if type_id!(ax) != 3 {
        std::process::exit(5);
    }

    if type_id!(cx) != 1 {
        std::process::exit(6);
    }
    if type_id!(ax.load(Ordering::SeqCst)) != 1 {
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