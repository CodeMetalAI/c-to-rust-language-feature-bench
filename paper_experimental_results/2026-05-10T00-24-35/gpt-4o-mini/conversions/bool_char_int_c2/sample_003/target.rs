fn type_id<T: 'static>() -> u8 {
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
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() { // Rust has no long double, using f64
        9
    } else {
        99
    }
}

struct BF {
    u1: u8, // 1 bit storage can be used as a regular u8 for simplicity
    i1: i8, // 1 bit storage can be used as a regular i8 for simplicity
    b1: bool,
}

fn main() -> i32 {
    if type_id::<i8>() != 1 {
        return 1;
    }
    if type_id::<u8>() != 1 {
        return 2;
    }
    if type_id::<i16>() != 1 {
        return 3;
    }
    if type_id::<u16>() != 1 {
        return 4;
    }

    let bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id::<u8>() != 1 {
        return 5;
    }
    if type_id::<i8>() != 1 {
        return 6;
    }
    if type_id::<bool>() != 1 {
        return 7;
    }

    if type_id::<f32>() != 7 {
        return 8;
    }
    if type_id::<f64>() != 8 {
        return 9;
    }
    if type_id::<f64>() != 9 { // Rust has no long double, using f64
        return 10;
    }

    0
}