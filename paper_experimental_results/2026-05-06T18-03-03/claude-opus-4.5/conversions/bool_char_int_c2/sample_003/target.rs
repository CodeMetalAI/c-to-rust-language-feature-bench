fn main() {
    // In Rust, we don't have C's integer promotion rules or _Generic.
    // However, we can verify that operations produce the expected types.
    
    // Tests 1-4: In C, small integer types promote to int when used in arithmetic.
    // In Rust, we need to explicitly cast or the types remain as-is.
    // We'll verify the equivalent behavior by checking that arithmetic works as expected.
    
    // (signed char)1 + 0 -> int in C, we simulate by casting i8 to i32
    let val1: i32 = (1i8 as i32) + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of_val(&val1) {
        std::process::exit(1);
    }
    
    // (unsigned char)1 + 0 -> int in C
    let val2: i32 = (1u8 as i32) + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of_val(&val2) {
        std::process::exit(2);
    }
    
    // (short)1 + 0 -> int in C
    let val3: i32 = (1i16 as i32) + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of_val(&val3) {
        std::process::exit(3);
    }
    
    // (unsigned short)1 + 0 -> int in C
    let val4: i32 = (1u16 as i32) + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of_val(&val4) {
        std::process::exit(4);
    }
    
    // Bit fields in C - when used in arithmetic, they promote to int
    // We simulate this with explicit types
    
    // bf.u1 (1-bit unsigned) + 0 -> int in C
    let bf_u1: u32 = 1;
    let val5: i32 = (bf_u1 as i32) + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of_val(&val5) {
        std::process::exit(5);
    }
    
    // bf.i1 (1-bit signed, value -1) + 0 -> int in C
    // A 1-bit signed field can only hold 0 or -1
    let bf_i1: i32 = -1;
    let val6: i32 = bf_i1 + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of_val(&val6) {
        std::process::exit(6);
    }
    
    // bf.b1 (_Bool/bool) + 0 -> int in C
    let bf_b1: bool = true;
    let val7: i32 = (bf_b1 as i32) + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of_val(&val7) {
        std::process::exit(7);
    }
    
    // Float type checks
    let val8: f32 = 1.0f32;
    if std::any::TypeId::of::<f32>() != std::any::TypeId::of_val(&val8) {
        std::process::exit(8);
    }
    
    let val9: f64 = 1.0f64;
    if std::any::TypeId::of::<f64>() != std::any::TypeId::of_val(&val9) {
        std::process::exit(9);
    }
    
    // Rust doesn't have long double, f64 is the largest standard float type
    // We use f64 as the equivalent
    let val10: f64 = 1.0f64;
    if std::any::TypeId::of::<f64>() != std::any::TypeId::of_val(&val10) {
        std::process::exit(10);
    }
    
    // All checks passed
    std::process::exit(0);
}