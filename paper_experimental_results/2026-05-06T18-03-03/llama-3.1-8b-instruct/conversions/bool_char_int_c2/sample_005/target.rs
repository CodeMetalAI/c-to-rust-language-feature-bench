fn type_id<T>(_: T) -> i32 {
    match std::any::type_id_of::<T>() {
        std::any::TypeId::of::<i32>() => 1,
        std::any::TypeId::of::<u32>() => 2,
        std::any::TypeId::of::<i64>() => 3,
        std::any::TypeId::of::<u64>() => 4,
        std::any::TypeId::of::<i128>() => 5,
        std::any::TypeId::of::<u128>() => 6,
        std::any::TypeId::of::<f32>() => 7,
        std::any::TypeId::of::<f64>() => 8,
        std::any::TypeId::of::<f128>() => 9,
        _ => 99,
    }
}

struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    if type_id((1i8 + 0) as i32) != 1 {
        return 1;
    }
    if type_id((1u8 + 0) as i32) != 1 {
        return 2;
    }
    if type_id((1i16 + 0) as i32) != 1 {
        return 3;
    }
    if type_id((1u16 + 0) as i32) != 1 {
        return 4;
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
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
    if type_id(1.0f64) != 8 {
        return 9;
    }
    if type_id(1.0f64) != 8 {
        return 10;
    }

    println!("All tests passed");
}