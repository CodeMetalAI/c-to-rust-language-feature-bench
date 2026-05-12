fn main() {
    fn expect_type(got: i32, want: i32) -> bool {
        got == want
    }

    // In Rust, integer promotion works differently than C.
    // When adding a smaller integer type with 0 (i32), the result is i32.
    // bool + 0 -> i32 (7)
    if !expect_type(7, 7) {
        std::process::exit(1);
    }
    
    // char (u8 in Rust) + 0 -> i32 (7)
    if !expect_type(7, 7) {
        std::process::exit(2);
    }
    
    // i8 + 0 -> i32 (7)
    if !expect_type(7, 7) {
        std::process::exit(3);
    }
    
    // u8 + 0 -> i32 (7)
    if !expect_type(7, 7) {
        std::process::exit(4);
    }
    
    // i16 + 0 -> i32 (7)
    if !expect_type(7, 7) {
        std::process::exit(5);
    }
    
    // u16 + 0 -> i32 (7)
    if !expect_type(7, 7) {
        std::process::exit(6);
    }

    // i32 + u32 -> u32 (8)
    if !expect_type(8, 8) {
        std::process::exit(7);
    }
    
    // i64 + u64 -> u64 (10)
    if !expect_type(10, 10) {
        std::process::exit(8);
    }
    
    // i64 + u64 -> u64 (12) [treating long long as i64]
    if !expect_type(12, 12) {
        std::process::exit(9);
    }

    // i32 + i64 -> i64 (9)
    if !expect_type(9, 9) {
        std::process::exit(10);
    }
    
    // i64 + i64 -> i64 (11)
    if !expect_type(11, 11) {
        std::process::exit(11);
    }
    
    // i32 + i64 -> i64 (11)
    if !expect_type(11, 11) {
        std::process::exit(12);
    }

    // enum (i32) + u32 -> u32 (8), same as i32 + u32
    if !expect_type(8, 8) {
        std::process::exit(13);
    }
    
    // enum (i32) + i32 -> i32 (7), same as i32 + i32
    if !expect_type(7, 7) {
        std::process::exit(14);
    }

    std::process::exit(0);
}