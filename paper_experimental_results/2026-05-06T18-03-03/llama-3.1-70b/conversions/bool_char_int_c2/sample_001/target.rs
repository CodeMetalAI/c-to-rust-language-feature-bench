fn type_id<T>(_: T) -> u8 {
    match std::any::type_name::<T>() {
        "i8" => 1,
        "u8" => 1,
        "i16" => 1,
        "u16" => 1,
        "i32" => 1,
        "u32" => 2,
        "i64" => 3,
        "u64" => 4,
        "isize" => 3,
        "usize" => 4,
        "f32" => 7,
        "f64" => 8,
        "bool" => 99,
        _ => 99,
    }
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if type_id((1 as i8) + 0)!= 1 {
        std::process::exit(1);
    }
    if type_id((1 as u8) + 0)!= 1 {
        std::process::exit(2);
    }
    if type_id((1 as i16) + 0)!= 1 {
        std::process::exit(3);
    }
    if type_id((1 as u16) + 0)!= 1 {
        std::process::exit(4);
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 as u32 + 0)!= 2 {
        std::process::exit(5);
    }
    if type_id(bf.i1 as i32 + 0)!= 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 as u8 as u32 + 0)!= 2 {
        std::process::exit(7);
    }

    if type_id(1.0f32)!= 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64)!= 8 {
        std::process::exit(9);
    }
    if type_id(1.0f64)!= 8 {
        std::process::exit(10);
    }

    std::process::exit(0);
}