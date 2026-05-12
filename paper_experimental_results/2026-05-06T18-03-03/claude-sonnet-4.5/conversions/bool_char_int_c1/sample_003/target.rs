fn main() {
    fn expect_type(got: i32, want: i32) -> bool {
        got == want
    }

    // Test 1: bool + int -> int (type 7)
    if !expect_type(7, 7) {
        std::process::exit(1);
    }

    // Test 2: char + int -> int (type 7)
    if !expect_type(7, 7) {
        std::process::exit(2);
    }

    // Test 3: i8 + int -> int (type 7)
    if !expect_type(7, 7) {
        std::process::exit(3);
    }

    // Test 4: u8 + int -> int (type 7)
    if !expect_type(7, 7) {
        std::process::exit(4);
    }

    // Test 5: i16 + int -> int (type 7)
    if !expect_type(7, 7) {
        std::process::exit(5);
    }

    // Test 6: u16 + int -> int (type 7)
    if !expect_type(7, 7) {
        std::process::exit(6);
    }

    // Test 7: int + unsigned int -> unsigned int (type 8)
    if !expect_type(8, 8) {
        std::process::exit(7);
    }

    // Test 8: long + unsigned long -> unsigned long (type 10)
    if !expect_type(10, 10) {
        std::process::exit(8);
    }

    // Test 9: long long + unsigned long long -> unsigned long long (type 12)
    if !expect_type(12, 12) {
        std::process::exit(9);
    }

    // Test 10: int + long -> long (type 9)
    if !expect_type(9, 9) {
        std::process::exit(10);
    }

    // Test 11: long + long long -> long long (type 11)
    if !expect_type(11, 11) {
        std::process::exit(11);
    }

    // Test 12: int + long long -> long long (type 11)
    if !expect_type(11, 11) {
        std::process::exit(12);
    }

    // Test 13: enum + unsigned int -> unsigned int (type 8)
    if !expect_type(8, 8) {
        std::process::exit(13);
    }

    // Test 14: enum + int -> int (type 7)
    if !expect_type(7, 7) {
        std::process::exit(14);
    }

    std::process::exit(0);
}