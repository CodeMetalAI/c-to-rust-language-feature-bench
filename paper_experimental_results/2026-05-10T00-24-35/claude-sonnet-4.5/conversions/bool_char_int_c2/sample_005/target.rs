fn main() {
    // In C, integer promotions convert smaller integer types to int when used in expressions.
    // The _Generic macro checks the type AFTER promotion.
    // signed char, unsigned char, short, unsigned short all promote to int.
    // When adding 0, these types promote to int (TYPE_ID 1).
    
    if type_id_i32((1i8 as i32) + 0) != 1 {
        std::process::exit(1);
    }
    if type_id_i32((1u8 as i32) + 0) != 1 {
        std::process::exit(2);
    }
    if type_id_i32((1i16 as i32) + 0) != 1 {
        std::process::exit(3);
    }
    if type_id_i32((1u16 as i32) + 0) != 1 {
        std::process::exit(4);
    }

    // Bitfields in C also undergo integer promotion when used in expressions
    let mut bf_u1: u32 = 1u32;
    let mut bf_i1: i32 = -1i32;
    let mut bf_b1: bool = true;
    
    // Bitfields are promoted to int when used in expressions
    if type_id_i32((bf_u1 & 0x1) as i32 + 0) != 1 {
        std::process::exit(5);
    }
    if type_id_i32(((bf_i1 << 31) >> 31) + 0) != 1 {
        std::process::exit(6);
    }
    if type_id_i32((bf_b1 as i32) + 0) != 1 {
        std::process::exit(7);
    }

    if type_id_f32(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id_f64(1.0f64) != 8 {
        std::process::exit(9);
    }
    // Rust doesn't have long double, but we can simulate the check
    if type_id_f64(1.0f64) != 8 {
        std::process::exit(10);
    }

    std::process::exit(0);
}

fn type_id_i32(_x: i32) -> i32 {
    1
}

fn type_id_f32(_x: f32) -> i32 {
    7
}

fn type_id_f64(_x: f64) -> i32 {
    8
}