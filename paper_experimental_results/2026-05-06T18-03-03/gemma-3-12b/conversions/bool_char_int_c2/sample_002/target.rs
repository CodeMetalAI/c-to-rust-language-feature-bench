#[derive(Debug, Copy, Clone)]
struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn type_id<T>(x: T) -> i32 {
    match std::any::type_name::<T>() {
        "i8" => 1,
        "u8" => 1,
        "i16" => 1,
        "u16" => 1,
        "i32" => 1,
        "u32" => 1,
        "i64" => 1,
        "u64" => 1,
        "f32" => 7,
        "f64" => 8,
        "f128" => 9,
        _ => 99,
    }
}

fn main() {
    if type_id((1 as signed char) + 0) != 1 {
        return 1;
    }
    if type_id((1 as unsigned char) + 0) != 1 {
        return 2;
    }
    if type_id((1 as i16) + 0) != 1 {
        return 3;
    }
    if type_id((1 as u16) + 0) != 1 {
        return 4;
    }

    let bf = BF {
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
    if type_id(1.0) != 8 {
        return 9;
    }
    if type_id(1.0) != 9 {
        return 10;
    }

    0
}