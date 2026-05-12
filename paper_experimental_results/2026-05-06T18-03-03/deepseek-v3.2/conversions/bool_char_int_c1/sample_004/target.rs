fn type_id<T>(_x: T) -> u32 {
    std::any::TypeId::of::<T>().hash(&mut std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default().build_hasher()) as u32
}

fn expect_type(got: u32, want: u32) -> bool {
    got == want
}

fn main() {
    // Get the type IDs for the expected types
    let id_bool = type_id(true);
    let id_char = type_id('a' as i8);
    let id_signed_char = type_id(0i8);
    let id_unsigned_char = type_id(0u8);
    let id_short = type_id(0i16);
    let id_unsigned_short = type_id(0u16);
    let id_int = type_id(0i32);
    let id_unsigned_int = type_id(0u32);
    let id_long = type_id(0i64);
    let id_unsigned_long = type_id(0u64);
    let id_long_long = type_id(0i128);
    let id_unsigned_long_long = type_id(0u128);

    // Test 1: (_Bool)1 + 0 -> int promotion
    if !expect_type(type_id((true as i32) + 0), id_int) {
        std::process::exit(1);
    }
    // Test 2: (char)1 + 0 -> int promotion
    if !expect_type(type_id(('a' as i8 as i32) + 0), id_int) {
        std::process::exit(2);
    }
    // Test 3: (signed char)1 + 0 -> int promotion
    if !expect_type(type_id((0i8 as i32) + 0), id_int) {
        std::process::exit(3);
    }
    // Test 4: (unsigned char)1 + 0 -> int promotion
    if !expect_type(type_id((0u8 as i32) + 0), id_int) {
        std::process::exit(4);
    }
    // Test 5: (short)1 + 0 -> int promotion
    if !expect_type(type_id((0i16 as i32) + 0), id_int) {
        std::process::exit(5);
    }
    // Test 6: (unsigned short)1 + 0 -> int promotion
    if !expect_type(type_id((0u16 as i32) + 0), id_int) {
        std::process::exit(6);
    }

    // Test 7: (int)0 + (unsigned int)0 -> unsigned int
    if !expect_type(type_id(0i32 + 0u32), id_unsigned_int) {
        std::process::exit(7);
    }
    // Test 8: (long)0 + (unsigned long)0 -> unsigned long
    if !expect_type(type_id(0i64 + 0u64), id_unsigned_long) {
        std::process::exit(8);
    }
    // Test 9: (long long)0 + (unsigned long long)0 -> unsigned long long
    if !expect_type(type_id(0i128 + 0u128), id_unsigned_long_long) {
        std::process::exit(9);
    }

    // Test 10: (int)0 + (long)0 -> long
    if !expect_type(type_id(0i32 + 0i64), id_long) {
        std::process::exit(10);
    }
    // Test 11: (long)0 + (long long)0 -> long long
    if !expect_type(type_id(0i64 + 0i128), id_long_long) {
        std::process::exit(11);
    }
    // Test 12: (int)0 + (long long)0 -> long long
    if !expect_type(type_id(0i32 + 0i128), id_long_long) {
        std::process::exit(12);
    }

    // Test 13: (enum E)0 + 0u -> same as (int)0 + 0u
    enum E { NEG = -1, POS = 1 }
    if !expect_type(type_id((E::POS as i32) + 0u32), type_id(0i32 + 0u32)) {
        std::process::exit(13);
    }
    // Test 14: (enum E)0 + 0 -> same as (int)0 + 0
    if !expect_type(type_id((E::POS as i32) + 0), type_id(0i32 + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}