fn type_id<T>(_: T) -> u8 {
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
        "f80" => 9,
        _ => 99,
    }
}

#[derive(Debug, Default)]
struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if type_id((1i8 + 0) as _)!= 1 {
        std::process::exit(1);
    }
    if type_id((1u8 + 0) as _)!= 1 {
        std::process::exit(2);
    }
    if type_id((1i16 + 0) as _)!= 1 {
        std::process::exit(3);
    }
    if type_id((1u16 + 0) as _)!= 1 {
        std::process::exit(4);
    }

    let mut bf = BF::default();
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    if type_id(bf.u1 + 0)!= 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1 as i32 + 0)!= 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 as u8 + 0)!= 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32)!= 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64)!= 8 {
        std::process::exit(9);
    }
    if type_id(1.0f80)!= 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}