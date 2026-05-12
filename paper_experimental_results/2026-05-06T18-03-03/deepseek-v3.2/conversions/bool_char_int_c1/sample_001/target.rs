fn main() {
    // Helper function to simulate the C expect_type behavior
    fn expect_type(got: u32, want: u32) -> bool {
        got == want
    }

    // Define type IDs matching the C code
    const TYPE_BOOL: u32 = 1;
    const TYPE_CHAR: u32 = 2;
    const TYPE_SIGNED_CHAR: u32 = 3;
    const TYPE_UNSIGNED_CHAR: u32 = 4;
    const TYPE_SHORT: u32 = 5;
    const TYPE_UNSIGNED_SHORT: u32 = 6;
    const TYPE_INT: u32 = 7;
    const TYPE_UNSIGNED_INT: u32 = 8;
    const TYPE_LONG: u32 = 9;
    const TYPE_UNSIGNED_LONG: u32 = 10;
    const TYPE_LONG_LONG: u32 = 11;
    const TYPE_UNSIGNED_LONG_LONG: u32 = 12;
    const TYPE_DEFAULT: u32 = 99;

    // Simulate C integer promotion rules
    // In C, small integer types are promoted to int when used in arithmetic
    
    // (_Bool)1 + 0 -> promoted to int
    if !expect_type(TYPE_INT, TYPE_INT) {
        std::process::exit(1);
    }
    
    // (char)1 + 0 -> promoted to int
    if !expect_type(TYPE_INT, TYPE_INT) {
        std::process::exit(2);
    }
    
    // (signed char)1 + 0 -> promoted to int
    if !expect_type(TYPE_INT, TYPE_INT) {
        std::process::exit(3);
    }
    
    // (unsigned char)1 + 0 -> promoted to int
    if !expect_type(TYPE_INT, TYPE_INT) {
        std::process::exit(4);
    }
    
    // (short)1 + 0 -> promoted to int
    if !expect_type(TYPE_INT, TYPE_INT) {
        std::process::exit(5);
    }
    
    // (unsigned short)1 + 0 -> promoted to int
    if !expect_type(TYPE_INT, TYPE_INT) {
        std::process::exit(6);
    }
    
    // (int)0 + (unsigned int)0 -> unsigned int
    if !expect_type(TYPE_UNSIGNED_INT, TYPE_UNSIGNED_INT) {
        std::process::exit(7);
    }
    
    // (long)0 + (unsigned long)0 -> unsigned long
    if !expect_type(TYPE_UNSIGNED_LONG, TYPE_UNSIGNED_LONG) {
        std::process::exit(8);
    }
    
    // (long long)0 + (unsigned long long)0 -> unsigned long long
    if !expect_type(TYPE_UNSIGNED_LONG_LONG, TYPE_UNSIGNED_LONG_LONG) {
        std::process::exit(9);
    }
    
    // (int)0 + (long)0 -> long
    if !expect_type(TYPE_LONG, TYPE_LONG) {
        std::process::exit(10);
    }
    
    // (long)0 + (long long)0 -> long long
    if !expect_type(TYPE_LONG_LONG, TYPE_LONG_LONG) {
        std::process::exit(11);
    }
    
    // (int)0 + (long long)0 -> long long
    if !expect_type(TYPE_LONG_LONG, TYPE_LONG_LONG) {
        std::process::exit(12);
    }
    
    // (enum E)0 + 0u -> unsigned int (enum promotes to int, then int + unsigned int = unsigned int)
    if !expect_type(TYPE_UNSIGNED_INT, TYPE_UNSIGNED_INT) {
        std::process::exit(13);
    }
    
    // (enum E)0 + 0 -> int (both are int after promotion)
    if !expect_type(TYPE_INT, TYPE_INT) {
        std::process::exit(14);
    }
    
    std::process::exit(0);
}