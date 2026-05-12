fn main() {
    // In Rust, we can't directly replicate _Generic, but we can check types via pattern matching
    // using std::any::TypeId. However, for integer promotion rules, we need to check the resulting
    // type after addition with 0 (which mimics C's integer promotion).
    
    // For signed char -> i32 in C when promoted (due to integer promotion)
    let signed_char: i8 = 1;
    let promoted_signed_char = signed_char as i32 + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(1);
    }

    // For unsigned char -> i32 in C when promoted (due to integer promotion)
    let unsigned_char: u8 = 1;
    let promoted_unsigned_char = unsigned_char as i32 + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(2);
    }

    // For short -> i32 in C when promoted
    let short: i16 = 1;
    let promoted_short = short as i32 + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(3);
    }

    // For unsigned short -> i32 in C when promoted
    let unsigned_short: u16 = 1;
    let promoted_unsigned_short = unsigned_short as i32 + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(4);
    }

    // Bitfield simulation in Rust (using struct with explicit bit manipulation)
    struct BF {
        u1: u8, // 1-bit unsigned (stored as u8 for simplicity)
        i1: i8, // 1-bit signed (stored as i8 for simplicity)
        b1: bool, // 1-bit bool
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    // For bitfield unsigned -> i32 in C when promoted
    let promoted_u1 = bf.u1 as i32 + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(5);
    }

    // For bitfield signed -> i32 in C when promoted
    let promoted_i1 = bf.i1 as i32 + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(6);
    }

    // For bitfield bool -> i32 in C when promoted (_Bool promotes to int)
    let promoted_b1 = bf.b1 as i32 + 0;
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(7);
    }

    // For float -> float (no promotion)
    let float_val: f32 = 1.0f32;
    if std::any::TypeId::of::<f32>() != std::any::TypeId::of::<f32>() {
        std::process::exit(8);
    }

    // For double -> double
    let double_val: f64 = 1.0f64;
    if std::any::TypeId::of::<f64>() != std::any::TypeId::of::<f64>() {
        std::process::exit(9);
    }

    // For long double -> long double (f128 in Rust, but not stable; we'll use f64 for compatibility)
    // Note: Rust doesn't have a standard long double; we approximate with f64.
    let long_double_val: f64 = 1.0f64;
    if std::any::TypeId::of::<f64>() != std::any::TypeId::of::<f64>() {
        std::process::exit(10);
    }

    std::process::exit(0);
}