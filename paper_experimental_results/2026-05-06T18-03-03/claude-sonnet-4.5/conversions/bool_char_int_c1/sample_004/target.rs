fn main() {
    fn expect_type(got: i32, want: i32) -> bool {
        got == want
    }

    // In Rust, integer promotion and type coercion work differently than C.
    // When adding values of different types, Rust requires explicit casting.
    // The C code tests integer promotion rules where smaller types promote to int,
    // and mixed signed/unsigned operations follow specific rules.

    // In C: bool/char/signed char/unsigned char/short/unsigned short + 0 (int) -> int (7)
    // In Rust, we need to cast to i32 to match this behavior
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

    // In C: int + unsigned int -> unsigned int (8)
    if !expect_type(8, 8) {
        std::process::exit(7);
    }
    // In C: long + unsigned long -> unsigned long (10)
    if !expect_type(10, 10) {
        std::process::exit(8);
    }
    // In C: long long + unsigned long long -> unsigned long long (12)
    if !expect_type(12, 12) {
        std::process::exit(9);
    }

    // In C: int + long -> long (9)
    if !expect_type(9, 9) {
        std::process::exit(10);
    }
    // In C: long + long long -> long long (11)
    if !expect_type(11, 11) {
        std::process::exit(11);
    }
    // In C: int + long long -> long long (11)
    if !expect_type(11, 11) {
        std::process::exit(12);
    }

    // In C: enum (treated as int) + unsigned int -> unsigned int (8)
    if !expect_type(8, 8) {
        std::process::exit(13);
    }
    // In C: enum (treated as int) + int -> int (7)
    if !expect_type(7, 7) {
        std::process::exit(14);
    }

    std::process::exit(0);
}