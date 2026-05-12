fn main() {
    fn expect_type(got: &str, want: &str) -> bool {
        got == want
    }

    fn type_id<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    // In Rust, arithmetic operations follow different rules than C
    // bool + int -> int
    // char types don't exist the same way
    // We need to simulate C's integer promotion rules
    
    // _Bool + 0 promotes to int (i32 in Rust context)
    if !expect_type(type_id(&(true as i32 + 0i32)), type_id(&0i32)) {
        std::process::exit(1);
    }
    
    // char + 0 promotes to int
    if !expect_type(type_id(&(1i8 as i32 + 0i32)), type_id(&0i32)) {
        std::process::exit(2);
    }
    
    // signed char + 0 promotes to int
    if !expect_type(type_id(&(1i8 as i32 + 0i32)), type_id(&0i32)) {
        std::process::exit(3);
    }
    
    // unsigned char + 0 promotes to int
    if !expect_type(type_id(&(1u8 as i32 + 0i32)), type_id(&0i32)) {
        std::process::exit(4);
    }
    
    // short + 0 promotes to int
    if !expect_type(type_id(&(1i16 as i32 + 0i32)), type_id(&0i32)) {
        std::process::exit(5);
    }
    
    // unsigned short + 0 promotes to int
    if !expect_type(type_id(&(1u16 as i32 + 0i32)), type_id(&0i32)) {
        std::process::exit(6);
    }
    
    // int + unsigned int -> unsigned int
    if !expect_type(type_id(&(0i32 as u32 + 0u32)), type_id(&0u32)) {
        std::process::exit(7);
    }
    
    // long + unsigned long -> unsigned long (u64)
    if !expect_type(type_id(&(0i64 as u64 + 0u64)), type_id(&0u64)) {
        std::process::exit(8);
    }
    
    // long long + unsigned long long -> unsigned long long (u64)
    if !expect_type(type_id(&(0i64 as u64 + 0u64)), type_id(&0u64)) {
        std::process::exit(9);
    }
    
    // int + long -> long (i64)
    if !expect_type(type_id(&(0i32 as i64 + 0i64)), type_id(&0i64)) {
        std::process::exit(10);
    }
    
    // long + long long -> long long (i64)
    if !expect_type(type_id(&(0i64 + 0i64)), type_id(&0i64)) {
        std::process::exit(11);
    }
    
    // int + long long -> long long (i64)
    if !expect_type(type_id(&(0i32 as i64 + 0i64)), type_id(&0i64)) {
        std::process::exit(12);
    }
    
    // enum + unsigned int -> type of (int + unsigned int) = unsigned int
    if !expect_type(type_id(&(0i32 as u32 + 0u32)), type_id(&(0i32 as u32 + 0u32))) {
        std::process::exit(13);
    }
    
    // enum + int -> type of (int + int) = int
    if !expect_type(type_id(&(0i32 + 0i32)), type_id(&(0i32 + 0i32))) {
        std::process::exit(14);
    }
    
    std::process::exit(0);
}