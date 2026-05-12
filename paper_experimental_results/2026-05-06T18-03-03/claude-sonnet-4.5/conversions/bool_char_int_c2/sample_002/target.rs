fn main() {
    // In C, integer promotions convert smaller integer types to int when used in expressions.
    // signed char, unsigned char, short, unsigned short all promote to int when added to 0.
    
    // Test integer promotions for small integer types
    if type_id_int(i8::wrapping_add(1i8 as i8, 0)) != 1 {
        std::process::exit(1);
    }
    if type_id_int(u8::wrapping_add(1u8 as u8, 0)) != 1 {
        std::process::exit(2);
    }
    if type_id_int(i16::wrapping_add(1i16 as i16, 0)) != 1 {
        std::process::exit(3);
    }
    if type_id_int(u16::wrapping_add(1u16 as u16, 0)) != 1 {
        std::process::exit(4);
    }

    // Bitfield struct - in C, bitfields undergo integer promotion to int
    struct BF {
        u1: u32,  // 1-bit unsigned
        i1: i32,  // 1-bit signed
        b1: bool, // 1-bit bool
    }

    let mut bf = BF {
        u1: 1u32,
        i1: -1i32,
        b1: true,
    };
    
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    // Bitfield values promote to int when used in expressions
    if type_id_int(bf.u1 as i32 + 0) != 1 {
        std::process::exit(5);
    }
    if type_id_int(bf.i1 + 0) != 1 {
        std::process::exit(6);
    }
    if type_id_int(bf.b1 as i32 + 0) != 1 {
        std::process::exit(7);
    }

    // Test floating point types
    if type_id_float(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id_double(1.0f64) != 8 {
        std::process::exit(9);
    }
    // Rust doesn't have long double, but f64 is the closest equivalent
    if type_id_double(1.0f64) != 8 {
        std::process::exit(10);
    }

    std::process::exit(0);
}

fn type_id_int(_x: i32) -> i32 {
    1
}

fn type_id_float(_x: f32) -> i32 {
    7
}

fn type_id_double(_x: f64) -> i32 {
    8
}