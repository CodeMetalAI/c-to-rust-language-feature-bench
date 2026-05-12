// Define an enum to match the C enum E
enum E {
    E_NEG = -1,
    E_POS = 1,
}

// Define a function to get the type ID
fn type_id<T>(_: T) -> i32
where
    T: Copy + Eq + std::fmt::Debug,
{
    match std::any::type_id::<T>() {
        std::any::TypeId::Bool => 1,
        std::any::TypeId::U8 => 2,
        std::any::TypeId::I8 => 3,
        std::any::TypeId::U16 => 4,
        std::any::TypeId::I16 => 5,
        std::any::TypeId::U32 => 6,
        std::any::TypeId::I32 => 7,
        std::any::TypeId::U64 => 8,
        std::any::TypeId::I64 => 9,
        std::any::TypeId::U128 => 10,
        std::any::TypeId::I128 => 11,
        _ => 99,
    }
}

// Define a function to check if the type ID matches the expected value
fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    // Test 1: bool + int
    if !expect_type(type_id(1 + 0), 7) {
        return 1;
    }

    // Test 2: char + int
    if !expect_type(type_id(1 + 0), 7) {
        return 2;
    }

    // Test 3: signed char + int
    if !expect_type(type_id(1 + 0), 7) {
        return 3;
    }

    // Test 4: unsigned char + int
    if !expect_type(type_id(1 + 0), 7) {
        return 4;
    }

    // Test 5: short + int
    if !expect_type(type_id(1 + 0), 7) {
        return 5;
    }

    // Test 6: unsigned short + int
    if !expect_type(type_id(1 + 0), 7) {
        return 6;
    }

    // Test 7: int + unsigned int
    if !expect_type(type_id(0 + 0), 8) {
        return 7;
    }

    // Test 8: long + unsigned long
    if !expect_type(type_id(0 + 0), 10) {
        return 8;
    }

    // Test 9: long long + unsigned long long
    if !expect_type(type_id(0 + 0), 12) {
        return 9;
    }

    // Test 10: int + long
    if !expect_type(type_id(0 + 0), 9) {
        return 10;
    }

    // Test 11: long + long long
    if !expect_type(type_id(0 + 0), 11) {
        return 11;
    }

    // Test 12: int + long long
    if !expect_type(type_id(0 + 0), 11) {
        return 12;
    }

    // Test 13: enum E + int
    if !expect_type(type_id(E::E_POS + 0), type_id(0 + 0)) {
        return 13;
    }

    // Test 14: enum E + int
    if !expect_type(type_id(E::E_POS + 0), type_id(0 + 0)) {
        return 14;
    }

    // If all tests pass, return 0
    return 0;
}