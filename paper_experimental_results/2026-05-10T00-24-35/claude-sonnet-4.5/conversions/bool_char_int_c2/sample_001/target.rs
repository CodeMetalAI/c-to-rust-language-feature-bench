fn main() {
    // In C, integer promotions cause char, short, and bitfields to promote to int
    // when used in expressions. Rust doesn't have automatic integer promotion,
    // but we need to match the C behavior where all these expressions result in i32.
    
    // Test signed char + 0 -> int (1)
    if type_id((1i8 + 0) as i32) != 1 {
        std::process::exit(1);
    }
    
    // Test unsigned char + 0 -> int (1)
    if type_id((1u8 + 0) as i32) != 1 {
        std::process::exit(2);
    }
    
    // Test short + 0 -> int (1)
    if type_id((1i16 + 0) as i32) != 1 {
        std::process::exit(3);
    }
    
    // Test unsigned short + 0 -> int (1)
    if type_id((1u16 + 0) as i32) != 1 {
        std::process::exit(4);
    }
    
    // Bitfields: In C, bitfields undergo integer promotion to int when used in expressions
    // We simulate this by using small types and converting to i32
    let u1: u8 = 1u8;
    let i1: i8 = -1i8;
    let b1: bool = true;
    
    // Bitfield u1 + 0 -> int (1)
    if type_id((u1 as i32) + 0) != 1 {
        std::process::exit(5);
    }
    
    // Bitfield i1 + 0 -> int (1)
    if type_id((i1 as i32) + 0) != 1 {
        std::process::exit(6);
    }
    
    // Bitfield b1 + 0 -> int (1)
    if type_id((b1 as i32) + 0) != 1 {
        std::process::exit(7);
    }
    
    // Test float (7)
    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    
    // Test double (8)
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }
    
    // Rust doesn't have long double, but f64 is the largest float type
    // In the C code, this tests long double which maps to f64 in most cases
    if type_id(1.0f64) != 8 {
        std::process::exit(10);
    }
    
    std::process::exit(0);
}

fn type_id<T>(_: T) -> i32 {
    std::any::type_name::<T>().into()
}

impl From<&str> for i32 {
    fn from(s: &str) -> i32 {
        match s {
            "i32" => 1,
            "u32" => 2,
            "i64" => 3,
            "u64" => 4,
            "i128" => 5,
            "u128" => 6,
            "f32" => 7,
            "f64" => 8,
            _ => 99,
        }
    }
}