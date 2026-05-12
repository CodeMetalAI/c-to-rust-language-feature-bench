fn main() {
    #[derive(Debug)]
    struct BF {
        u1: u32, // 1-bit unsigned packed field
        i1: i32, // 1-bit signed packed field
        b1: bool, // 1-bit boolean packed field
    }

    let mut bf = BF { u1: 0, i1: 0, b1: false };
    bf.u1 = 1u32;
    bf.i1 = -1i32;
    bf.b1 = true;

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

    if type_id(bf.u1 as i32) != 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1) != 1 {
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
    if type_id(1.0f64) != 8 { // Rust does not have a native long double type.
        std::process::exit(10);
    }
}

fn type_id<T>(_: T) -> i32 {
    let type_id = std::any::type_name::<T>();
    match type_id {
        "i32" | "i8" | "i16" | "bool" => 1,
        "u32" | "u8" | "u16" => 2,
        "i64" => 3,
        "u64" => 4,
        "f32" => 7,
        "f64" => 8,
        _ => 99,
    }
}