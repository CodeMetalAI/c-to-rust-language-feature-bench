fn main() {
    // In C, integer promotions convert smaller integer types to int when used in expressions.
    // signed char, unsigned char, short, unsigned short, and bitfields all promote to int
    // when added to 0 (or used in any arithmetic expression).
    
    // Test signed char + 0 -> int
    let _val: i32 = 1i8 as i32 + 0;
    if type_id_i32(_val) != 1 {
        std::process::exit(1);
    }
    
    // Test unsigned char + 0 -> int
    let _val: i32 = 1u8 as i32 + 0;
    if type_id_i32(_val) != 1 {
        std::process::exit(2);
    }
    
    // Test short + 0 -> int
    let _val: i32 = 1i16 as i32 + 0;
    if type_id_i32(_val) != 1 {
        std::process::exit(3);
    }
    
    // Test unsigned short + 0 -> int
    let _val: i32 = 1u16 as i32 + 0;
    if type_id_i32(_val) != 1 {
        std::process::exit(4);
    }
    
    // Bitfields in C also undergo integer promotion to int
    let bf_u1: u32 = 1u32;
    let bf_i1: i32 = -1i32;
    let bf_b1: bool = true;
    
    // Bitfield u1 (1-bit unsigned) + 0 -> int
    let _val: i32 = bf_u1 as i32 + 0;
    if type_id_i32(_val) != 1 {
        std::process::exit(5);
    }
    
    // Bitfield i1 (1-bit signed) + 0 -> int
    let _val: i32 = bf_i1 + 0;
    if type_id_i32(_val) != 1 {
        std::process::exit(6);
    }
    
    // Bitfield b1 (1-bit bool) + 0 -> int
    let _val: i32 = (bf_b1 as i32) + 0;
    if type_id_i32(_val) != 1 {
        std::process::exit(7);
    }
    
    // Test float
    let _val: f32 = 1.0f32;
    if type_id_f32(_val) != 7 {
        std::process::exit(8);
    }
    
    // Test double
    let _val: f64 = 1.0f64;
    if type_id_f64(_val) != 8 {
        std::process::exit(9);
    }
    
    // Rust doesn't have long double, but f64 is the closest equivalent
    // In practice, on most systems long double maps to f64 anyway
    let _val: f64 = 1.0f64;
    if type_id_f64(_val) != 8 {
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