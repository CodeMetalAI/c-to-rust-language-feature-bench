#[derive(Debug, Default)]
struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn type_id<T: std::fmt::Debug>(_x: T) -> u8 {
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
        "bool" => 1,
        _ => 99,
    }
}

fn main() {
    if type_id((i8::from(1) + 0) as i8)!= 1 {
        std::process::exit(1);
    }
    if type_id((u8::from(1) + 0) as u8)!= 1 {
        std::process::exit(2);
    }
    if type_id((i16::from(1) + 0) as i16)!= 1 {
        std::process::exit(3);
    }
    if type_id((u16::from(1) + 0) as u16)!= 1 {
        std::process::exit(4);
    }

    let mut bf = BF::default();
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    if type_id(bf.u1 as u8)!= 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1 as i8)!= 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1)!= 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32)!= 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64)!= 8 {
        std::process::exit(9);
    }
    // Note: Rust does not have a direct equivalent to C's long double type.
    // The closest equivalent is f64, which is used here.
    if type_id(1.0f64)!= 8 {
        std::process::exit(10);
    }

    std::process::exit(0);
}