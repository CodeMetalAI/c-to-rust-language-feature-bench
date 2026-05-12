#![allow(unused)]

fn type_id<T>(_: T) -> u8 {
    match std::mem::size_of::<T>() {
        4 => 1,
        8 => 2,
        _ => 99,
    }
}

fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let mut ax: std::sync::atomic::AtomicI32 = 5;

    if std::mem::size_of::<i32>() != 4 {
        return 1;
    }
    if std::align_of::<i32>() != 4 {
        return 2;
    }

    if *x != 3 {
        return 3;
    }
    if type_id(*x) != 1 {
        return 4;
    }
    if type_id(ax.load(std::sync::atomic::Ordering::SeqCst)) != 3 {
        return 5;
    }

    let cx = +cx;
    if type_id(cx) != 1 {
        return 6;
    }
    let ax = +ax.load(std::sync::atomic::Ordering::SeqCst);
    if type_id(ax) != 1 {
        return 7;
    }

    let y = x;
    if y != 3 {
        return 8;
    }

    x += 1;
    if x != 4 {
        return 9;
    }

    struct S {
        m: i32,
    }
    let s = S { m: 7 };
    if s.m != 7 {
        return 10;
    }
    s.m = 8;
    if s.m != 8 {
        return 11;
    }

    return 0;
}