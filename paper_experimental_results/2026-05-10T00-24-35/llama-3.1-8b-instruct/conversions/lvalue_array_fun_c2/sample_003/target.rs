#![allow(unused)]

fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let ax: i32 = 5;

    if std::mem::size_of_val(&x)!= std::mem::size_of::<i32>() {
        return 1;
    }
    if std::mem::align_of::<i32>()!= std::mem::align_of_val(&x) {
        return 2;
    }

    if *x!= 3 {
        return 3;
    }

    enum TypeId {
        Int,
        Uint,
        AtomicInt,
        Default,
    }

    let type_id = match x {
        _ => TypeId::Default,
        _ => TypeId::Int,
        _ => TypeId::Int,
        _ => unreachable!(),
    };

    if type_id!= TypeId::Int {
        return 4;
    }

    let type_id = match ax {
        _ => TypeId::Default,
        _ => TypeId::AtomicInt,
        _ => unreachable!(),
    };

    if type_id!= TypeId::AtomicInt {
        return 5;
    }

    let type_id = match cx + 0 {
        _ => TypeId::Default,
        _ => TypeId::Int,
        _ => unreachable!(),
    };

    let type_id = match ax + 0 {
        _ => TypeId::Default,
        _ => TypeId::Int,
        _ => unreachable!(),
    };

    let y = x;
    if y!= 3 {
        return 8;
    }

    x += 1;
    if x!= 4 {
        return 9;
    }

    struct S {
        m: i32,
    }

    let s = S { m: 7 };

    if s.m!= 7 {
        return 10;
    }

    s.m = 8;
    if s.m!= 8 {
        return 11;
    }

    (0..=0).for_each(|_| {});
}