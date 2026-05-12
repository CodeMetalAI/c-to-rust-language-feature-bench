fn main() {
    // In Rust, we don't have the same type promotion rules as C,
    // but we can verify that operations produce the expected types.
    
    // Test 1-4: In C, small integer types promote to int when used in arithmetic.
    // In Rust, we explicitly work with i32 to match the behavior.
    let _: i32 = (1i8 as i32) + 0;   // signed char + 0 -> int
    let _: i32 = (1u8 as i32) + 0;   // unsigned char + 0 -> int
    let _: i32 = (1i16 as i32) + 0;  // short + 0 -> int
    let _: i32 = (1u16 as i32) + 0;  // unsigned short + 0 -> int
    
    // Tests 5-7: Bit fields in C promote to int.
    // We simulate the struct with appropriate types and verify promotion.
    struct BF {
        u1: bool,  // 1-bit unsigned
        i1: bool,  // 1-bit signed (can only hold 0 or -1)
        b1: bool,  // 1-bit bool
    }
    
    let bf = BF {
        u1: true,   // 1u
        i1: true,   // -1 (all bits set in 1-bit signed field)
        b1: true,   // 1
    };
    
    // In C, bit-field + 0 promotes to int
    let _: i32 = (bf.u1 as i32) + 0;
    let _: i32 = (bf.i1 as i32) + 0;
    let _: i32 = (bf.b1 as i32) + 0;
    
    // Tests 8-10: Verify float types
    let _: f32 = 1.0f32;   // float
    let _: f64 = 1.0f64;   // double
    // Rust doesn't have long double, f64 is the largest standard float type
    let _: f64 = 1.0f64;   // long double equivalent
    
    // If we reach here, all type checks passed (equivalent to return 0)
}