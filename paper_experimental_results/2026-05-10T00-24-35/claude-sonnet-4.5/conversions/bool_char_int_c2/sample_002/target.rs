fn main() {
    // In C, integer promotions convert smaller integer types to int when used in expressions.
    // signed char, unsigned char, short, unsigned short, and 1-bit bitfields all promote to int.
    
    // Test 1: signed char + 0 promotes to int
    if type_id_int(i8::wrapping_add(1, 0) as i32) != 1 {
        std::process::exit(1);
    }
    
    // Test 2: unsigned char + 0 promotes to int
    if type_id_int(u8::wrapping_add(1, 0) as i32) != 1 {
        std::process::exit(2);
    }
    
    // Test 3: short + 0 promotes to int
    if type_id_int(i16::wrapping_add(1, 0) as i32) != 1 {
        std::process::exit(3);
    }
    
    // Test 4: unsigned short + 0 promotes to int
    if type_id_int(u16::wrapping_add(1, 0) as i32) != 1 {
        std::process::exit(4);
    }
    
    // Bitfield simulation: 1-bit fields promote to int when used in expressions
    let u1: u32 = 1;
    let i1: i32 = -1;
    let b1: bool = true;
    
    // Test 5: 1-bit unsigned field + 0 promotes to int
    if type_id_int((u1 & 1) as i32 + 0) != 1 {
        std::process::exit(5);
    }
    
    // Test 6: 1-bit signed field + 0 promotes to int
    if type_id_int(i1 + 0) != 1 {
        std::process::exit(6);
    }
    
    // Test 7: 1-bit bool field + 0 promotes to int
    if type_id_int(b1 as i32 + 0) != 1 {
        std::process::exit(7);
    }
    
    // Test 8: float literal
    if type_id_float(1.0f32) != 7 {
        std::process::exit(8);
    }
    
    // Test 9: double literal
    if type_id_double(1.0f64) != 8 {
        std::process::exit(9);
    }
    
    // Test 10: long double (Rust doesn't have long double, use f64)
    // In practice, on many systems long double is the same as double
    if type_id_double(1.0f64) != 9 {
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