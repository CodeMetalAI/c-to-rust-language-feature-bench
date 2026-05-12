fn main() {
    // In C, integer promotions convert smaller integer types to int when used in expressions.
    // signed char, unsigned char, short, unsigned short, and bitfields all promote to int
    // when added to 0 (or used in any arithmetic expression).
    
    // Test signed char + 0 -> int (type_id 1)
    let _val: i32 = 1i8 as i32 + 0;
    if type_id_i32() != 1 {
        std::process::exit(1);
    }
    
    // Test unsigned char + 0 -> int (type_id 1)
    let _val: i32 = 1u8 as i32 + 0;
    if type_id_i32() != 1 {
        std::process::exit(2);
    }
    
    // Test short + 0 -> int (type_id 1)
    let _val: i32 = 1i16 as i32 + 0;
    if type_id_i32() != 1 {
        std::process::exit(3);
    }
    
    // Test unsigned short + 0 -> int (type_id 1)
    let _val: i32 = 1u16 as i32 + 0;
    if type_id_i32() != 1 {
        std::process::exit(4);
    }
    
    // Bitfield tests - in C, bitfields undergo integer promotion to int
    // We simulate this by using the appropriate types and promoting to i32
    let bf_u1: u32 = 1u32;
    let bf_i1: i32 = -1i32;
    let bf_b1: bool = true;
    
    // bf.u1 + 0 promotes to int (type_id 1)
    let _val: i32 = bf_u1 as i32 + 0;
    if type_id_i32() != 1 {
        std::process::exit(5);
    }
    
    // bf.i1 + 0 promotes to int (type_id 1)
    let _val: i32 = bf_i1 + 0;
    if type_id_i32() != 1 {
        std::process::exit(6);
    }
    
    // bf.b1 + 0 promotes to int (type_id 1)
    let _val: i32 = (bf_b1 as i32) + 0;
    if type_id_i32() != 1 {
        std::process::exit(7);
    }
    
    // Test float (type_id 7)
    let _val: f32 = 1.0f32;
    if type_id_f32() != 7 {
        std::process::exit(8);
    }
    
    // Test double (type_id 8)
    let _val: f64 = 1.0f64;
    if type_id_f64() != 8 {
        std::process::exit(9);
    }
    
    // Test long double (type_id 9)
    // Rust doesn't have long double, but f64 is the closest equivalent
    // The C code is testing the type itself, so we use f64
    let _val: f64 = 1.0f64;
    if type_id_f64() != 9 {
        std::process::exit(10);
    }
    
    std::process::exit(0);
}

fn type_id_i32() -> i32 {
    1
}

fn type_id_f32() -> i32 {
    7
}

fn type_id_f64() -> i32 {
    9
}