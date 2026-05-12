fn type_id<T: std::any::Any>(_: T) -> u8 {
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
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() { // Rust doesn't have long double, we use f64
        9
    } else {
        99
    }
}

struct BF {
    u1: u8, // Use u8 for bit-fields simulation
    i1: i8, // Use i8 for bit-fields simulation
    b1: bool,
}

fn main() -> i32 {
    if type_id(1i8 + 0) != 1 {
        return 1;
    }
    if type_id(1u8 + 0) != 1 {
        return 2;
    }
    if type_id(1i16 + 0) != 1 {
        return 3;
    }
    if type_id(1u16 + 0) != 1 {
        return 4;
    }

    let mut bf = BF { u1: 1, i1: -1, b1: true };

    if type_id(bf.u1 + 0) != 1 {
        return 5;
    }
    if type_id(bf.i1 + 0) != 1 {
        return 6;
    }
    if type_id(bf.b1 as u8 + 0) != 1 { // Convert bool to u8
        return 7;
    }

    if type_id(1.0f32) != 7 {
        return 8;
    }
    if type_id(1.0f64) != 8 {
        return 9;
    }
    if type_id(1.0f64) != 9 { // No direct long double, using f64
        return 10;
    }

    return 0;
}