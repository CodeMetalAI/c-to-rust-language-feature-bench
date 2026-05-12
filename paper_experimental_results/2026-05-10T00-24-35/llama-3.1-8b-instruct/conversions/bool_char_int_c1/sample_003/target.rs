// We need to define a type for size_t, which is an alias for usize in Rust.
type Size = usize;

// Define an enum for the type IDs, mapping each type to a unique ID.
enum TypeId {
    Bool = 1,
    Char = 2,
    SignedChar = 3,
    UnsignedChar = 4,
    Short = 5,
    UnsignedShort = 6,
    Int = 7,
    UnsignedInt = 8,
    Long = 9,
    UnsignedLong = 10,
    LongLong = 11,
    UnsignedLongLong = 12,
    Other = 99,
}

// Define a function to expect a specific type ID and return 1 if it's not met.
fn expect_type(got: TypeId, want: TypeId) -> i32 {
    if got == want {
        0
    } else {
        1
    }
}

fn main() {
    // Test adding 0 to each type
    // bool
    if expect_type(TYPE_ID(1_i32 + 0), TYPE_ID(0_i32 + 0)) == 1 {
        return 1;
    }
    // char
    if expect_type(TYPE_ID(1_i32 + 0), TYPE_ID(0_i32 + 0)) == 1 {
        return 2;
    }
    // signed char
    if expect_type(TYPE_ID(1_i8 + 0), TYPE_ID(0_i8 + 0)) == 1 {
        return 3;
    }
    // unsigned char
    if expect_type(TYPE_ID(1_u8 + 0), TYPE_ID(0_u8 + 0)) == 1 {
        return 4;
    }
    // short
    if expect_type(TYPE_ID(1_i16 + 0), TYPE_ID(0_i16 + 0)) == 1 {
        return 5;
    }
    // unsigned short
    if expect_type(TYPE_ID(1_u16 + 0), TYPE_ID(0_u16 + 0)) == 1 {
        return 6;
    }

    // int + unsigned int
    if expect_type(TYPE_ID(0_i32 + 0_u32), TYPE_ID(0_u32 + 0_u32)) == 1 {
        return 7;
    }
    // long + unsigned long
    if expect_type(TYPE_ID(0_i64 + 0_u64), TYPE_ID(0_u64 + 0_u64)) == 1 {
        return 8;
    }
    // long long + unsigned long long
    if expect_type(TYPE_ID(0_i128 + 0_u128), TYPE_ID(0_u128 + 0_u128)) == 1 {
        return 9;
    }

    // int + long
    if expect_type(TYPE_ID(0_i32 + 0_i64), TYPE_ID(0_i64 + 0_i64)) == 1 {
        return 10;
    }
    // long + long long
    if expect_type(TYPE_ID(0_i64 + 0_i128), TYPE_ID(0_i128 + 0_i128)) == 1 {
        return 11;
    }
    // int + long long
    if expect_type(TYPE_ID(0_i32 + 0_i128), TYPE_ID(0_i128 + 0_i128)) == 1 {
        return 12;
    }

    // enum + int
    if expect_type(TYPE_ID(0_i32 as TypeId + 0), TYPE_ID(0_i32 + 0)) == 1 {
        return 13;
    }
    // enum + int (same test, but with i32)
    if expect_type(TYPE_ID(0_i32 as TypeId + 0), TYPE_ID(0_i32 + 0)) == 1 {
        return 14;
    }

    // If none of the above tests failed, return 0.
    0
}