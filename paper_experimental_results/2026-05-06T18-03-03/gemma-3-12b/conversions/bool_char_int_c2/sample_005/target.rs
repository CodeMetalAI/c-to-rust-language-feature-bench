#[derive(Debug, PartialEq, Eq)]
struct BF {
    u1: u32,
    i1: i32,
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
    if type_id((1 as signed char) + 0) != 1 {
        std::process::exit(1);
    }
    if type_id((1 as unsigned char) + 0) != 1 {
        std::process::exit(2);
    }
    if type_id((1 as i16) + 0) != 1 {
        std::process::exit(3);
    }
    if type_id((1 as u16) + 0) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 + 0) != 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1 + 0) != 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 + 0) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0) != 8 {
        std::process::exit(9);
    }
    if type_id(1.0) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}