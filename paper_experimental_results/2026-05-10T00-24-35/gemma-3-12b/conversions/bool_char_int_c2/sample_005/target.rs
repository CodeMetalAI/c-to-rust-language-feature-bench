#[derive(Debug, PartialEq, Eq)]
struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn type_id<T>(x: T) -> i32 {
    match x {
        _ if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() => 1,
        _ if std::any::TypeId::of::<u32>() == std::any::TypeId::of::<T>() => 2,
        _ if std::any::TypeId::of::<i64>() == std::any::TypeId::of::<T>() => 3,
        _ if std::any::TypeId::of::<u64>() == std::any::TypeId::of::<T>() => 4,
        _ if std::any::TypeId::of::<i128>() == std::any::TypeId::of::<T>() => 5,
        _ if std::any::TypeId::of::<u128>() == std::any::TypeId::of::<T>() => 6,
        _ if std::any::TypeId::of::<f32>() == std::any::TypeId::of::<T>() => 7,
        _ if std::any::TypeId::of::<f64>() == std::any::TypeId::of::<T>() => 8,
        _ if std::any::TypeId::of::<f128>() == std::any::TypeId::of::<T>() => 9,
        _ => 99,
    }
}

fn main() {
    if type_id((signed char)1 + 0) != 1 {
        return 1;
    }
    if type_id((unsigned char)1 + 0) != 1 {
        return 2;
    }
    if type_id((i16)1 + 0) != 1 {
        return 3;
    }
    if type_id((u16)1 + 0) != 1 {
        return 4;
    }

    let mut bf = BF {
        u1: 1u8,
        i1: -1i8,
        b1: true,
    };

    if type_id(bf.u1 + 0) != 1 {
        return 5;
    }
    if type_id(bf.i1 + 0) != 1 {
        return 6;
    }
    if type_id(bf.b1 + 0) != 1 {
        return 7;
    }

    if type_id(1.0f32) != 7 {
        return 8;
    }
    if type_id(1.0) != 8 {
        return 9;
    }
    if type_id(1.0) != 9 {
        return 10;
    }

    0
}