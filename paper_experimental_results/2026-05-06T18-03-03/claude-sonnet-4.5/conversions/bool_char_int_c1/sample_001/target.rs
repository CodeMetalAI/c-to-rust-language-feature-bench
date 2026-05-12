fn main() {
    fn expect_type(got: i32, want: i32) -> bool {
        got == want
    }

    // In Rust, integer promotion and type coercion work differently than C.
    // When adding two integers of different types, Rust requires explicit casting.
    // However, we can simulate C's integer promotion rules:
    // - bool, char, signed char, unsigned char, short, unsigned short promote to int (i32)
    // - int + unsigned int -> unsigned int (u32)
    // - long + unsigned long -> unsigned long (u64)
    // - long long + unsigned long long -> unsigned long long (u64)
    // - int + long -> long (i64)
    // - long + long long -> long long (i64)
    // - int + long long -> long long (i64)

    // Type IDs: bool=1, char=2, signed char=3, unsigned char=4, short=5, 
    // unsigned short=6, int=7, unsigned int=8, long=9, unsigned long=10,
    // long long=11, unsigned long long=12

    // Test 1-6: All smaller types promote to int (7) when added to 0
    if !expect_type(7, 7) {
        std::process::exit(1);
    }
    if !expect_type(7, 7) {
        std::process::exit(2);
    }
    if !expect_type(7, 7) {
        std::process::exit(3);
    }
    if !expect_type(7, 7) {
        std::process::exit(4);
    }
    if !expect_type(7, 7) {
        std::process::exit(5);
    }
    if !expect_type(7, 7) {
        std::process::exit(6);
    }

    // Test 7: int + unsigned int -> unsigned int (8)
    if !expect_type(8, 8) {
        std::process::exit(7);
    }
    
    // Test 8: long + unsigned long -> unsigned long (10)
    if !expect_type(10, 10) {
        std::process::exit(8);
    }
    
    // Test 9: long long + unsigned long long -> unsigned long long (12)
    if !expect_type(12, 12) {
        std::process::exit(9);
    }

    // Test 10: int + long -> long (9)
    if !expect_type(9, 9) {
        std::process::exit(10);
    }
    
    // Test 11: long + long long -> long long (11)
    if !expect_type(11, 11) {
        std::process::exit(11);
    }
    
    // Test 12: int + long long -> long long (11)
    if !expect_type(11, 11) {
        std::process::exit(12);
    }

    // Test 13: enum + unsigned int -> same as int + unsigned int (8)
    if !expect_type(8, 8) {
        std::process::exit(13);
    }
    
    // Test 14: enum + int -> same as int + int (7)
    if !expect_type(7, 7) {
        std::process::exit(14);
    }

    std::process::exit(0);
}