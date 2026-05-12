// Define an enum to represent the type ids
enum TypeId {
    Bool = -1,
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
}

// Define a trait to represent the types
trait Type {
    fn type_id(&self) -> TypeId;
}

// Implement the Type trait for various types
impl Type for bool {
    fn type_id(&self) -> TypeId {
        TypeId::Bool
    }
}

impl Type for u8 {
    fn type_id(&self) -> TypeId {
        TypeId::UnsignedChar
    }
}

impl Type for i8 {
    fn type_id(&self) -> TypeId {
        TypeId::SignedChar
    }
}

impl Type for i16 {
    fn type_id(&self) -> TypeId {
        TypeId::Int
    }
}

impl Type for u16 {
    fn type_id(&self) -> TypeId {
        TypeId::UnsignedInt
    }
}

impl Type for i32 {
    fn type_id(&self) -> TypeId {
        TypeId::Int
    }
}

impl Type for u32 {
    fn type_id(&self) -> TypeId {
        TypeId::UnsignedInt
    }
}

impl Type for i64 {
    fn type_id(&self) -> TypeId {
        TypeId::LongLong
    }
}

impl Type for u64 {
    fn type_id(&self) -> TypeId {
        TypeId::UnsignedLongLong
    }
}

// Define a function to check if two types are the same
fn expect_type(got: TypeId, want: TypeId) -> bool {
    got == want
}

// Define an enum to represent the E enum
enum E {
    E_NEG = -1,
    E_POS = 1,
}

impl Type for E {
    fn type_id(&self) -> TypeId {
        TypeId::Int
    }
}

fn main() {
    // Check if the type of true + 0 is int
    if !expect_type(true.type_id() + 0, TypeId::Int as i32) {
        return 1;
    }
    
    // Check if the type of char + 0 is int
    if !expect_type(char::type_id() + 0, TypeId::Int as i32) {
        return 2;
    }
    
    // Check if the type of signed char + 0 is int
    if !expect_type(i8::type_id() + 0, TypeId::Int as i32) {
        return 3;
    }
    
    // Check if the type of unsigned char + 0 is int
    if !expect_type(u8::type_id() + 0, TypeId::Int as i32) {
        return 4;
    }
    
    // Check if the type of short + 0 is int
    if !expect_type(i16::type_id() + 0, TypeId::Int as i32) {
        return 5;
    }
    
    // Check if the type of unsigned short + 0 is int
    if !expect_type(u16::type_id() + 0, TypeId::Int as i32) {
        return 6;
    }
    
    // Check if the type of int + 0 is unsigned int
    if !expect_type(i32::type_id() + 0, TypeId::UnsignedInt as i32) {
        return 7;
    }
    
    // Check if the type of long + 0 is unsigned long
    if !expect_type(i64::type_id() + 0, TypeId::UnsignedLong as i32) {
        return 8;
    }
    
    // Check if the type of long long + 0 is unsigned long long
    if !expect_type(i64::type_id() + 0, TypeId::UnsignedLongLong as i32) {
        return 9;
    }
    
    // Check if the type of int + long is long
    if !expect_type(i32::type_id() + i64::type_id(), TypeId::Long as i32) {
        return 10;
    }
    
    // Check if the type of long + long long is long long
    if !expect_type(i64::type_id() + i64::type_id(), TypeId::LongLong as i32) {
        return 11;
    }
    
    // Check if the type of int + long long is long long
    if !expect_type(i32::type_id() + i64::type_id(), TypeId::LongLong as i32) {
        return 12;
    }
    
    // Check if the type of enum E + 0u is int
    if !expect_type(E::E_NEG.type_id() + 0u32 as i32, i32::type_id() + 0) {
        return 13;
    }
    
    // Check if the type of enum E + 0 is int
    if !expect_type(E::E_NEG.type_id() + 0, i32::type_id() + 0) {
        return 14;
    }
    
    // If all checks pass, return 0
    return 0;
}