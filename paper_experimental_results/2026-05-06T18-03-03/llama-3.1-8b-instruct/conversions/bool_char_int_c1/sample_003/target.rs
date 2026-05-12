// Define an enum to represent the type IDs
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
    Default = 99,
}

// Define a function to get the type ID of a value
fn type_id<T>(_: T) -> TypeId
where
    T: Copy + Eq + std::ops::Add<Output = T>,
{
    match std::mem::size_of::<T>() {
        1 => TypeId::Char,
        2 => TypeId::Short,
        4 => TypeId::Int,
        8 => TypeId::LongLong,
        _ => TypeId::Default,
    }
}

// Define an enum to represent the E type
enum E {
    E_NEG = -1,
    E_POS = 1,
}

// Define a function to check if the type ID of two values is the same
fn expect_type<T>(got: TypeId, want: TypeId) -> bool
where
    T: Copy + Eq + std::ops::Add<Output = T>,
{
    got == want
}

fn main() {
    // Test cases
    if!expect_type(type_id(1 + 0), TypeId::Int) {
        panic!("Test case 1 failed");
    }
    if!expect_type(type_id(1 + 0), TypeId::Int) {
        panic!("Test case 2 failed");
    }
    if!expect_type(type_id(1 + 0), TypeId::Int) {
        panic!("Test case 3 failed");
    }
    if!expect_type(type_id(1 + 0), TypeId::Int) {
        panic!("Test case 4 failed");
    }
    if!expect_type(type_id(1 + 0), TypeId::Int) {
        panic!("Test case 5 failed");
    }
    if!expect_type(type_id(1 + 0), TypeId::Int) {
        panic!("Test case 6 failed");
    }

    if!expect_type(type_id(0 + 0u), TypeId::UnsignedInt) {
        panic!("Test case 7 failed");
    }
    if!expect_type(type_id(0 + 0u), TypeId::UnsignedLong) {
        panic!("Test case 8 failed");
    }
    if!expect_type(type_id(0 + 0u), TypeId::UnsignedLongLong) {
        panic!("Test case 9 failed");
    }

    if!expect_type(type_id(0 + 0), TypeId::Long) {
        panic!("Test case 10 failed");
    }
    if!expect_type(type_id(0 + 0), TypeId::LongLong) {
        panic!("Test case 11 failed");
    }
    if!expect_type(type_id(0 + 0), TypeId::LongLong) {
        panic!("Test case 12 failed");
    }

    if!expect_type(type_id(E::E_NEG + 0u), type_id(0 + 0u)) {
        panic!("Test case 13 failed");
    }
    if!expect_type(type_id(E::E_NEG + 0), type_id(0 + 0)) {
        panic!("Test case 14 failed");
    }
}