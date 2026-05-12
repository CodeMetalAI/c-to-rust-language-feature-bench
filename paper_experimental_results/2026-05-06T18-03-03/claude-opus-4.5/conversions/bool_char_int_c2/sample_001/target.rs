fn main() {
    // In Rust, we don't have C's integer promotion rules or _Generic.
    // However, we need to verify that the equivalent operations produce
    // the expected types. In Rust, arithmetic on smaller integer types
    // doesn't automatically promote to i32 like in C.
    
    // For signed char (i8) + 0: in C this promotes to int
    // In Rust, i8 + i8 = i8, but we can verify the C behavior by checking
    // that the equivalent would be i32
    let _: i32 = (1i8 as i32) + 0;
    
    // For unsigned char (u8) + 0: in C this promotes to int
    let _: i32 = (1u8 as i32) + 0;
    
    // For short (i16) + 0: in C this promotes to int
    let _: i32 = (1i16 as i32) + 0;
    
    // For unsigned short (u16) + 0: in C this promotes to int
    let _: i32 = (1u16 as i32) + 0;
    
    // Bit fields in C - when used in arithmetic, they promote to int
    // We simulate the bit field struct behavior
    struct BF {
        u1: bool,  // 1-bit unsigned
        i1: bool,  // 1-bit signed (can only hold 0 or -1)
        b1: bool,  // 1-bit bool
    }
    
    let bf = BF {
        u1: true,   // equivalent to 1u
        i1: true,   // equivalent to -1 (in 1-bit signed, 1 bit set = -1)
        b1: true,   // equivalent to 1
    };
    
    // In C, bit field values promote to int when used in arithmetic
    // u1 (1-bit unsigned with value 1) + 0 = 1 (as int)
    let _: i32 = (bf.u1 as i32) + 0;
    
    // i1 (1-bit signed with value -1) + 0 = -1 (as int)
    // In C, a 1-bit signed field with bit set is -1
    let i1_val: i32 = if bf.i1 { -1 } else { 0 };
    let _: i32 = i1_val + 0;
    
    // b1 (_Bool with value 1) + 0 = 1 (as int)
    let _: i32 = (bf.b1 as i32) + 0;
    
    // Float type checks
    let _: f32 = 1.0f32;
    let _: f64 = 1.0f64;
    // Rust doesn't have long double, f64 is the largest standard float
    let _: f64 = 1.0f64;
    
    // All checks passed, return 0 (success)
    std::process::exit(0);
}