use std::mem::discriminant;

fn type_id<T>(_x: T) -> u32 {
    match discriminant(&_x) {
        _ if std::mem::discriminant(&0i32) == std::mem::discriminant(&_x) => 1,
        _ if std::mem::discriminant(&0u32) == std::mem::discriminant(&_x) => 2,
        _ if std::mem::discriminant(&0i64) == std::mem::discriminant(&_x) => 3,
        _ if std::mem::discriminant(&0u64) == std::mem::discriminant(&_x) => 4,
        _ if std::mem::discriminant(&0i128) == std::mem::discriminant(&_x) => 5,
        _ if std::mem::discriminant(&0u128) == std::mem::discriminant(&_x) => 6,
        _ if std::mem::discriminant(&0.0f32) == std::mem::discriminant(&_x) => 7,
        _ if std::mem::discriminant(&0.0f64) == std::mem::discriminant(&_x) => 8,
        _ if std::mem::discriminant(&0.0f64) == std::mem::discriminant(&_x) => 9, // Note: Rust doesn't have long double
        _ => 99,
    }
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

impl BF {
    fn new() -> Self {
        BF {
            u1: 0,
            i1: 0,
            b1: false,
        }
    }
}

fn main() {
    if type_id((1 as i8) + 0) != 1 {
        std::process::exit(1);
    }
    if type_id((1 as u8) + 0) != 1 {
        std::process::exit(2);
    }
    if type_id((1 as i16) + 0) != 1 {
        std::process::exit(3);
    }
    if type_id((1 as u16) + 0) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF::new();
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    if type_id(bf.u1 + 0) != 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1 + 0) != 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 as i32 + 0) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }
    if type_id(1.0f64) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}