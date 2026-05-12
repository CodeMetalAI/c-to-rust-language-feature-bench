// Define a type alias for size_t
type Size = usize;

// Define a macro to get the type ID
macro_rules! type_id {
    ($x:expr) => {{
        match $x {
            true => 1,
            _ if $x >= 0 && $x <= 255 => 2,
            _ if $x >= -128 && $x < 0 => 3,
            _ if $x >= 0 && $x <= 65535 => 4,
            _ if $x >= -32768 && $x < 0 => 5,
            _ if $x >= 0 && $x <= 65535 => 6,
            _ if $x >= -2147483648 && $x < 0 => 7,
            _ if $x >= 0 && $x <= 4294967295 => 8,
            _ if $x >= -9223372036854775808 && $x < 0 => 9,
            _ if $x >= 0 && $x <= 18446744073709551615 => 10,
            _ if $x >= -9223372036854775808 && $x < 0 => 11,
            _ if $x >= 0 && $x <= 18446744073709551615 => 12,
            _ => 99,
        }
    }};
}

// Define an enum with two variants
enum E {
    E_NEG = -1,
    E_POS = 1,
}

// Define a function to check if the type ID matches the expected value
fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    // Test cases
    if!expect_type(type_id(1 + 0), 7) {
        return 1;
    }
    if!expect_type(type_id(1 + 0), 7) {
        return 2;
    }
    if!expect_type(type_id(-128 + 0), 7) {
        return 3;
    }
    if!expect_type(type_id(255 + 0), 7) {
        return 4;
    }
    if!expect_type(type_id(32767 + 0), 7) {
        return 5;
    }
    if!expect_type(type_id(65535 + 0), 7) {
        return 6;
    }

    if!expect_type(type_id(0 + 0u), 8) {
        return 7;
    }
    if!expect_type(type_id(0 + 0u), 10) {
        return 8;
    }
    if!expect_type(type_id(0 + 0u), 12) {
        return 9;
    }

    if!expect_type(type_id(0 + 0), 9) {
        return 10;
    }
    if!expect_type(type_id(0 + 0), 11) {
        return 11;
    }
    if!expect_type(type_id(0 + 0), 11) {
        return 12;
    }

    if!expect_type(type_id(E::E_NEG + 0u), type_id(0 + 0u)) {
        return 13;
    }
    if!expect_type(type_id(E::E_NEG + 0), type_id(0 + 0)) {
        return 14;
    }

    // If all tests pass, return 0
    return 0;
}