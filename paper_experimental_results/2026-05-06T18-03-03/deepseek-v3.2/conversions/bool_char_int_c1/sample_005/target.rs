fn type_id<T>(_x: T) -> u32 {
    std::any::TypeId::of::<T>().hash(&mut std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default().build_hasher()) as u32
}

fn expect_type(got: u32, want: u32) -> bool {
    got == want
}

fn main() {
    let bool_type = type_id(true);
    let char_type = type_id('a');
    let i8_type = type_id(0i8);
    let u8_type = type_id(0u8);
    let i16_type = type_id(0i16);
    let u16_type = type_id(0u16);
    let i32_type = type_id(0i32);
    let u32_type = type_id(0u32);
    let i64_type = type_id(0i64);
    let u64_type = type_id(0u64);
    let i128_type = type_id(0i128);
    let u128_type = type_id(0u128);
    let isize_type = type_id(0isize);
    let usize_type = type_id(0usize);

    // Test 1: (_Bool)1 + 0 -> int in C, i32 in Rust
    if !expect_type(type_id((true as i32) + 0), i32_type) {
        std::process::exit(1);
    }
    // Test 2: (char)1 + 0 -> int in C, i32 in Rust
    if !expect_type(type_id(('a' as i32) + 0), i32_type) {
        std::process::exit(2);
    }
    // Test 3: (signed char)1 + 0 -> int in C, i32 in Rust
    if !expect_type(type_id((0i8 as i32) + 0), i32_type) {
        std::process::exit(3);
    }
    // Test 4: (unsigned char)1 + 0 -> int in C, i32 in Rust
    if !expect_type(type_id((0u8 as i32) + 0), i32_type) {
        std::process::exit(4);
    }
    // Test 5: (short)1 + 0 -> int in C, i32 in Rust
    if !expect_type(type_id((0i16 as i32) + 0), i32_type) {
        std::process::exit(5);
    }
    // Test 6: (unsigned short)1 + 0 -> int in C, i32 in Rust
    if !expect_type(type_id((0u16 as i32) + 0), i32_type) {
        std::process::exit(6);
    }

    // Test 7: (int)0 + (unsigned int)0 -> unsigned int in C, u32 in Rust
    if !expect_type(type_id(0i32 as u32 + 0u32), u32_type) {
        std::process::exit(7);
    }
    // Test 8: (long)0 + (unsigned long)0 -> unsigned long in C, u64 in Rust (assuming 64-bit long)
    if !expect_type(type_id(0i64 as u64 + 0u64), u64_type) {
        std::process::exit(8);
    }
    // Test 9: (long long)0 + (unsigned long long)0 -> unsigned long long in C, u128 in Rust (closest match)
    if !expect_type(type_id(0i128 as u128 + 0u128), u128_type) {
        std::process::exit(9);
    }

    // Test 10: (int)0 + (long)0 -> long in C, i64 in Rust
    if !expect_type(type_id(0i32 as i64 + 0i64), i64_type) {
        std::process::exit(10);
    }
    // Test 11: (long)0 + (long long)0 -> long long in C, i128 in Rust
    if !expect_type(type_id(0i64 as i128 + 0i128), i128_type) {
        std::process::exit(11);
    }
    // Test 12: (int)0 + (long long)0 -> long long in C, i128 in Rust
    if !expect_type(type_id(0i32 as i128 + 0i128), i128_type) {
        std::process::exit(12);
    }

    // Test 13: (enum E)0 + 0u -> unsigned int in C, u32 in Rust
    enum E { NEG = -1, POS = 1 }
    if !expect_type(type_id((E::POS as i32) as u32 + 0u32), type_id(0i32 as u32 + 0u32)) {
        std::process::exit(13);
    }
    // Test 14: (enum E)0 + 0 -> int in C, i32 in Rust
    if !expect_type(type_id((E::POS as i32) + 0), type_id(0i32 + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}