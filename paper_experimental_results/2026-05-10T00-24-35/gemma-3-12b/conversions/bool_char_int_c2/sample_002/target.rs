#[derive(Debug)]
struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn type_id<T>(x: T) -> i32 {
    match std::any::type_name::<T>() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize" => 1,
        "f32" => 7,
        "f64" => 8,
        "f128" => 9,
        _ => 99,
    }
}

fn main() {
    if type_id((1i8 as signed char) + 0) != 1 {
        return 1;
    }
    if type_id((1u8 as unsigned char) + 0) != 1 {
        return 2;
    }
    if type_id((1i16 as short) + 0) != 1 {
        return 3;
    }
    if type_id((1u16 as unsigned short) + 0) != 1 {
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
    if type_id(1.0) != 8 {
        return 9;
    }
    if type_id(1.0) != 9 {
        return 10;
    }

    0
}