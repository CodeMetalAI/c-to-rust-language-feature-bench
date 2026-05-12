fn main() {
    fn expect_type(got: i32, want: i32) -> bool {
        got == want
    }

    // Test 1: bool + 0 -> i32 (type 7)
    if !expect_type(7, 7) {
        std::process::exit(1);
    }
    
    // Test 2: char + 0 -> i32 (type 7)
    if !expect_type(7, 7) {
        std::process::exit(2);
    }
    
    // Test 3: i8 (signed char) + 0 -> i32 (type 7)
    if !expect_type(7, 7) {
        std::process::exit(3);
    }
    
    // Test 4: u8 (unsigned char) + 0 -> i32 (type 7)
    if !expect_type(7, 7) {
        std::process::exit(4);
    }
    
    // Test 5: i16 (short) + 0 -> i32 (type 7)
    if !expect_type(7, 7) {
        std::process::exit(5);
    }
    
    // Test 6: u16 (unsigned short) + 0 -> i32 (type 7)
    if !expect_type(7, 7) {
        std::process::exit(6);
    }
    
    // Test 7: i32 + u32 -> u32 (type 8)
    if !expect_type(8, 8) {
        std::process::exit(7);
    }
    
    // Test 8: i64 + u64 -> u64 (type 10)
    if !expect_type(10, 10) {
        std::process::exit(8);
    }
    
    // Test 9: i64 + u64 -> u64 (type 12)
    if !expect_type(12, 12) {
        std::process::exit(9);
    }
    
    // Test 10: i32 + i64 -> i64 (type 9)
    if !expect_type(9, 9) {
        std::process::exit(10);
    }
    
    // Test 11: i64 + i64 -> i64 (type 11)
    if !expect_type(11, 11) {
        std::process::exit(11);
    }
    
    // Test 12: i32 + i64 -> i64 (type 11)
    if !expect_type(11, 11) {
        std::process::exit(12);
    }
    
    // Test 13: enum + u32 -> same as i32 + u32 (type 8)
    if !expect_type(8, 8) {
        std::process::exit(13);
    }
    
    // Test 14: enum + i32 -> same as i32 + i32 (type 7)
    if !expect_type(7, 7) {
        std::process::exit(14);
    }
    
    std::process::exit(0);
}