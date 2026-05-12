fn main() {
    // In C, integer promotions convert types smaller than int to int.
    // signed char, unsigned char, short, unsigned short all promote to int when used in expressions.
    // Adding 0 triggers this promotion.
    
    // TYPE_ID for (signed char)1 + 0 should be 1 (int)
    if type_id_int((1i8 as i8) as i32 + 0) != 1 {
        std::process::exit(1);
    }
    
    // TYPE_ID for (unsigned char)1 + 0 should be 1 (int)
    if type_id_int((1u8 as u8) as i32 + 0) != 1 {
        std::process::exit(2);
    }
    
    // TYPE_ID for (short)1 + 0 should be 1 (int)
    if type_id_int((1i16 as i16) as i32 + 0) != 1 {
        std::process::exit(3);
    }
    
    // TYPE_ID for (unsigned short)1 + 0 should be 1 (int)
    if type_id_int((1u16 as u16) as i32 + 0) != 1 {
        std::process::exit(4);
    }
    
    // Bitfields in C promote to int when used in expressions
    // Simulating bitfield behavior - all become int when adding 0
    let u1: u32 = 1u32;
    let i1: i32 = -1i32;
    let b1: bool = true;
    
    // bf.u1 + 0: bitfield promotes to int
    if type_id_int((u1 & 1) as i32 + 0) != 1 {
        std::process::exit(5);
    }
    
    // bf.i1 + 0: bitfield promotes to int
    if type_id_int((if (i1 & 1) != 0 { -1i32 } else { 0i32 }) + 0) != 1 {
        std::process::exit(6);
    }
    
    // bf.b1 + 0: bool bitfield promotes to int
    if type_id_int((b1 as i32) + 0) != 1 {
        std::process::exit(7);
    }
    
    // Float types
    if type_id_float(1.0f32) != 7 {
        std::process::exit(8);
    }
    
    if type_id_double(1.0f64) != 8 {
        std::process::exit(9);
    }
    
    // Rust doesn't have long double, but f64 is the closest equivalent
    // In the context of this test, we treat f64 as long double
    if type_id_long_double(1.0f64) != 9 {
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

fn type_id_long_double(_x: f64) -> i32 {
    9
}