fn type_id<T: 'static>(_: T) -> usize {
    match std::any::TypeId::of::<T>() {
        x if x == std::any::TypeId::of::<i32>() => 1,
        x if x == std::any::TypeId::of::<u32>() => 2,
        x if x == std::any::TypeId::of::<i64>() => 3,
        x if x == std::any::TypeId::of::<u64>() => 4,
        x if x == std::any::TypeId::of::<i128>() => 5,
        x if x == std::any::TypeId::of::<u128>() => 6,
        x if x == std::any::TypeId::of::<f32>() => 7,
        x if x == std::any::TypeId::of::<f64>() => 8,
        _ => 99,
    }
}

struct BF {
    u1: u8, // Here we use u8 type to simulate bit-fields
    i1: i8,
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

    let bf = BF { u1: 1, i1: -1, b1: true };

    if type_id(bf.u1 as i32 + 0) != 1 {
        return 5;
    }
    if type_id(bf.i1 as i32 + 0) != 1 {
        return 6;
    }
    if type_id(bf.b1 as i32 + 0) != 1 {
        return 7;
    }

    if type_id(1.0f32) != 7 {
        return 8;
    }
    if type_id(1.0f64) != 8 {
        return 9;
    }
    if type_id(1.0f64) != 9 {
        return 10;
    }

    0
}