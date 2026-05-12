fn main() {
    fn expect_type(got: &str, want: &str) -> bool {
        got == want
    }

    fn type_id<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    // Test 1: bool + 0 promotes to i32
    if !expect_type(type_id(&(true as i32 + 0)), type_id(&0i32)) {
        std::process::exit(1);
    }

    // Test 2: char (i8 in Rust) + 0 promotes to i32
    if !expect_type(type_id(&(1i8 + 0)), type_id(&0i32)) {
        std::process::exit(2);
    }

    // Test 3: signed char (i8) + 0 promotes to i32
    if !expect_type(type_id(&(1i8 + 0)), type_id(&0i32)) {
        std::process::exit(3);
    }

    // Test 4: unsigned char (u8) + 0 promotes to i32
    if !expect_type(type_id(&(1u8 as i32 + 0)), type_id(&0i32)) {
        std::process::exit(4);
    }

    // Test 5: short (i16) + 0 promotes to i32
    if !expect_type(type_id(&(1i16 as i32 + 0)), type_id(&0i32)) {
        std::process::exit(5);
    }

    // Test 6: unsigned short (u16) + 0 promotes to i32
    if !expect_type(type_id(&(1u16 as i32 + 0)), type_id(&0i32)) {
        std::process::exit(6);
    }

    // Test 7: int + unsigned int -> unsigned int
    if !expect_type(type_id(&(0i32 + 0u32)), type_id(&0u32)) {
        std::process::exit(7);
    }

    // Test 8: long + unsigned long -> unsigned long
    if !expect_type(type_id(&(0i64 + 0u64)), type_id(&0u64)) {
        std::process::exit(8);
    }

    // Test 9: long long + unsigned long long -> unsigned long long
    if !expect_type(type_id(&(0i64 + 0u64)), type_id(&0u64)) {
        std::process::exit(9);
    }

    // Test 10: int + long -> long
    if !expect_type(type_id(&(0i32 as i64 + 0i64)), type_id(&0i64)) {
        std::process::exit(10);
    }

    // Test 11: long + long long -> long long
    if !expect_type(type_id(&(0i64 + 0i64)), type_id(&0i64)) {
        std::process::exit(11);
    }

    // Test 12: int + long long -> long long
    if !expect_type(type_id(&(0i32 as i64 + 0i64)), type_id(&0i64)) {
        std::process::exit(12);
    }

    // Test 13: enum + unsigned int -> same as int + unsigned int
    if !expect_type(type_id(&(0i32 + 0u32)), type_id(&(0i32 + 0u32))) {
        std::process::exit(13);
    }

    // Test 14: enum + int -> same as int + int
    if !expect_type(type_id(&(0i32 + 0i32)), type_id(&(0i32 + 0i32))) {
        std::process::exit(14);
    }

    std::process::exit(0);
}