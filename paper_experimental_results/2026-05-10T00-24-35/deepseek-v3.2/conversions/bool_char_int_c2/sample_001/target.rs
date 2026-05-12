use std::mem::discriminant;

fn type_id<T>(_x: T) -> i32 {
    match discriminant(&_x) {
        d if d == discriminant(&1i32) => 1,
        d if d == discriminant(&1u32) => 2,
        d if d == discriminant(&1i64) => 3,
        d if d == discriminant(&1u64) => 4,
        d if d == discriminant(&1i128) => 5,
        d if d == discriminant(&1u128) => 6,
        d if d == discriminant(&1.0f32) => 7,
        d if d == discriminant(&1.0f64) => 8,
        d if d == discriminant(&1.0f64) => 9, // Note: Rust doesn't have long double, using f64
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
        Self {
            u1: 0,
            i1: 0,
            b1: false,
        }
    }
}

fn main() {
    if type_id((1i8 as i32) + 0) != 1 {
        std::process::exit(1);
    }
    if type_id((1u8 as i32) + 0) != 1 {
        std::process::exit(2);
    }
    if type_id((1i16 as i32) + 0) != 1 {
        std::process::exit(3);
    }
    if type_id((1u16 as i32) + 0) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF::new();
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    if type_id((bf.u1 as i32) + 0) != 1 {
        std::process::exit(5);
    }
    if type_id((bf.i1 as i32) + 0) != 1 {
        std::process::exit(6);
    }
    if type_id((bf.b1 as i32) + 0) != 1 {
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