fn type_id<T>(_value: T) -> u8 {
    match std::any::type_name::<T>() {
        "i32" => 1,
        "u32" => 2,
        "i64" => 3,
        "u64" => 4,
        "i128" => 5,
        "u128" => 6,
        "f32" => 7,
        "f64" => 8,
        _ => 99,
    }
}

struct BitField {
    u1: bool,
    i1: bool,
    b1: bool,
}

fn main() {
    if type_id((1i8 + 0) as i32) != 1 {
        std::process::exit(1);
    }
    if type_id((1u8 + 0) as i32) != 1 {
        std::process::exit(2);
    }
    if type_id((1i16 + 0) as i32) != 1 {
        std::process::exit(3);
    }
    if type_id((1u16 + 0) as i32) != 1 {
        std::process::exit(4);
    }

    let bf = BitField {
        u1: true,
        i1: true,  // -1 is represented by true in this 1-bit field context
        b1: true,
    };

    if type_id(bf.u1 as i32) != 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1 as i32) != 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 as i32) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }

    // Rust does not have `long double`, using f64
    if type_id(1.0f64) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}