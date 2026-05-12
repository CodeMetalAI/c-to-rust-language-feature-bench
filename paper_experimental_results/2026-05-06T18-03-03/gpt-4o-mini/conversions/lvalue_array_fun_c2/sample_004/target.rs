fn main() -> i32 {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        return 1;
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        return 2;
    }

    if x != 3 {
        return 3;
    }

    if type_id(&x) != 1 {
        return 4;
    }
    if type_id(&ax) != 3 {
        return 5;
    }

    if type_id(&cx) != 1 {
        return 6;
    }
    if type_id(&ax) != 1 {
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
    let mut s = S { m: 7 };
    if s.m != 7 {
        return 10;
    }
    s.m = 8;
    if s.m != 8 {
        return 11;
    }

    0
}

fn type_id<T>(_: &T) -> u32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        1
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<std::sync::atomic::AtomicI32>() {
        3
    } else {
        99
    }
}