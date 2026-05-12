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
fn type_id<T>(_: T) -> TypeId {
    match std::any::type_id_of::<T>() {
        std::any::TypeId::U8 => TypeId::UnsignedChar,
        std::any::TypeId::I8 => TypeId::SignedChar,
        std::any::TypeId::U16 => TypeId::UnsignedShort,
        std::any::TypeId::I16 => TypeId::Short,
        std::any::TypeId::U32 => TypeId::UnsignedInt,
        std::any::TypeId::I32 => TypeId::Int,
        std::any::TypeId::U64 => TypeId::UnsignedLongLong,
        std::any::TypeId::I64 => TypeId::LongLong,
        _ => TypeId::Default,
    }
}

// Define an enum to represent the error codes
enum ErrorCode {
    Error1 = 1,
    Error2 = 2,
    Error3 = 3,
    Error4 = 4,
    Error5 = 5,
    Error6 = 6,
    Error7 = 7,
    Error8 = 8,
    Error9 = 9,
    Error10 = 10,
    Error11 = 11,
    Error12 = 12,
    Error13 = 13,
    Error14 = 14,
}

// Define a function to check if the type ID of two values is equal
fn expect_type<T, U>(got: TypeId, want: TypeId) -> ErrorCode {
    if got != want {
        return ErrorCode::Error1;
    }
    ErrorCode::Success
}

fn main() {
    // Test cases
    if expect_type(type_id(1u8 + 0), TypeId::Int) != ErrorCode::Success {
        return ErrorCode::Error1 as i32;
    }
    if expect_type(type_id(1i8 + 0), TypeId::Int) != ErrorCode::Success {
        return ErrorCode::Error2 as i32;
    }
    if expect_type(type_id(1u8 + 0), TypeId::Int) != ErrorCode::Success {
        return ErrorCode::Error3 as i32;
    }
    if expect_type(type_id(1u16 + 0), TypeId::Int) != ErrorCode::Success {
        return ErrorCode::Error4 as i32;
    }
    if expect_type(type_id(1i16 + 0), TypeId::Int) != ErrorCode::Success {
        return ErrorCode::Error5 as i32;
    }
    if expect_type(type_id(1u32 + 0), TypeId::Int) != ErrorCode::Success {
        return ErrorCode::Error6 as i32;
    }

    if expect_type(type_id(0u32 + 0u32), TypeId::UnsignedInt) != ErrorCode::Success {
        return ErrorCode::Error7 as i32;
    }
    if expect_type(type_id(0u64 + 0u64), TypeId::UnsignedLongLong) != ErrorCode::Success {
        return ErrorCode::Error8 as i32;
    }
    if expect_type(type_id(0i64 + 0i64), TypeId::LongLong) != ErrorCode::Success {
        return ErrorCode::Error9 as i32;
    }

    if expect_type(type_id(0i32 + 0u32), TypeId::UnsignedInt) != ErrorCode::Success {
        return ErrorCode::Error10 as i32;
    }
    if expect_type(type_id(0i64 + 0u64), TypeId::UnsignedLongLong) != ErrorCode::Success {
        return ErrorCode::Error11 as i32;
    }
    if expect_type(type_id(0i32 + 0i64), TypeId::LongLong) != ErrorCode::Success {
        return ErrorCode::Error12 as i32;
    }

    if expect_type(type_id(0i32 as i64 + 0u32 as i64), type_id(0i32 + 0u32)) != ErrorCode::Success {
        return ErrorCode::Error13 as i32;
    }
    if expect_type(type_id(0i32 as i64 + 0i32), type_id(0i32 + 0i32)) != ErrorCode::Success {
        return ErrorCode::Error14 as i32;
    }

    // If all tests pass, return 0
    0
}