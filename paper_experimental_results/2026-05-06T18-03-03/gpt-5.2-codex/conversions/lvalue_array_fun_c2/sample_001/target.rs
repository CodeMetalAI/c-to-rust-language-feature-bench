use std::mem::{align_of, align_of_val, size_of, size_of_val};
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

trait TypeId {
    fn type_id(&self) -> i32;
}

impl TypeId for i32 {
    fn type_id(&self) -> i32 {
        1
    }
}

impl TypeId for AtomicI32 {
    fn type_id(&self) -> i32 {
        3
    }
}

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if size_of_val(&x) != size_of::<i32>() {
        exit(1);
    }
    if align_of_val(&x) != align_of::<i32>() {
        exit(2);
    }

    if *(&x) != 3 {
        exit(3);
    }

    if x.type_id() != 1 {
        exit(4);
    }
    if ax.type_id() != 3 {
        exit(5);
    }

    if (+cx).type_id() != 1 {
        exit(6);
    }
    let ax_val = ax.load(Ordering::SeqCst);
    if (+ax_val).type_id() != 1 {
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

    exit(0);
}