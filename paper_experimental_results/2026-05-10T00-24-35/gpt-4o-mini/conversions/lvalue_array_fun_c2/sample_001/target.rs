fn type_id<T: TypeId>() -> u8 {
    T::type_id()
}

trait TypeId {
    fn type_id() -> u8;
}

impl TypeId for i32 {
    fn type_id() -> u8 {
        1
    }
}

impl TypeId for u32 {
    fn type_id() -> u8 {
        2
    }
}

impl TypeId for std::sync::atomic::AtomicI32 {
    fn type_id() -> u8 {
        3
    }
}

impl TypeId for () {
    fn type_id() -> u8 {
        99
    }
}

struct S {
    m: i32,
}

fn main() -> i32 {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        return 1;
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        return 2;
    }

    if x != 3 {
        return 3;
    }

    if type_id::<i32>() != 1 {
        return 4;
    }
    if type_id::<std::sync::atomic::AtomicI32>() != 3 {
        return 5;
    }

    if type_id::<i32>() != 1 {
        return 6;
    }
    if type_id::<std::sync::atomic::AtomicI32>() != 1 {
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