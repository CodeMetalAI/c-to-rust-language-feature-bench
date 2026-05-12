fn type_id<T: 'static>(_: &T) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        1
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
        2
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i64>() {
        3
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u64>() {
        4
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i128>() {
        5
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u128>() {
        6
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f32>() {
        7
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() {
        8
    } else {
        99
    }
}

struct BF {
    u1: bool,
    i1: bool,
    b1: bool,
}

fn main() {
    if type_id(&((1i8 + 0) as i32)) != 1 {
        std::process::exit(1);
    }
    if type_id(&((1u8 + 0) as i32)) != 1 {
        std::process::exit(2);
    }
    if type_id(&((1i16 + 0) as i32)) != 1 {
        std::process::exit(3);
    }
    if type_id(&((1u16 + 0) as i32)) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF {
        u1: true,
        i1: true,
        b1: true,
    };

    if type_id(&((bf.u1 as i32) + 0)) != 1 {
        std::process::exit(5);
    }
    if type_id(&((bf.i1 as i32) + 0)) != 1 {
        std::process::exit(6);
    }
    if type_id(&((bf.b1 as i32) + 0)) != 1 {
        std::process::exit(7);
    }

    if type_id(&(1.0f32)) != 7 {
        std::process::exit(8);
    }
    if type_id(&(1.0f64)) != 8 {
        std::process::exit(9);
    }
    if type_id(&(1.0f64)) != 9 {  // Rust does not support `long double`, using `f64` as closest equivalent.
        std::process::exit(10);
    }

    std::process::exit(0);
}