#![allow(unused_imports)]

use std::sync::atomic::{AtomicIsize, Ordering};

fn main() {
    let x: isize = 3;
    let cx: isize = 4;
    let ax: AtomicIsize = AtomicIsize::new(5);

    if std::mem::size_of::<isize>()!= std::mem::size_of::<i32>() {
        return 1;
    }
    if std::mem::align_of::<isize>()!= std::mem::align_of::<i32>() {
        return 2;
    }

    if *(&x)!= 3 {
        return 3;
    }

    enum TypeId {
        Int,
        UnsignedInt,
        AtomicInt,
        Default,
    }

    let type_id = match (x, cx, ax) {
        (isize, _, _) => TypeId::Int,
        (_, _, AtomicIsize) => TypeId::AtomicInt,
        _ => TypeId::Default,
    };

    match type_id {
        TypeId::Int => {}
        TypeId::UnsignedInt => {}
        TypeId::AtomicInt => {}
        TypeId::Default => return 99,
    }

    let y = x;
    if y!= 3 {
        return 8;
    }

    x += 1;
    if x!= 4 {
        return 9;
    }

    struct S {
        m: isize,
    }

    let s = S { m: 7 };
    if s.m!= 7 {
        return 10;
    }
    s.m = 8;
    if s.m!= 8 {
        return 11;
    }

    return 0;
}