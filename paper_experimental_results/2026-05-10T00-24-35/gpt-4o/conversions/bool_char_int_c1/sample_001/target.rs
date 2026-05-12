#[derive(Debug, PartialEq, Eq)]
enum TypeId {
    Bool,
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Default,
}

fn type_id<T>(_value: T) -> TypeId {
    use TypeId::*;
    match std::any::type_name::<T>() {
        "bool" => Bool,
        "i8" => SignedChar,
        "u8" => UnsignedChar,
        "i16" => Short,
        "u16" => UnsignedShort,
        "i32" => Int,
        "u32" => UnsignedInt,
        "i64" => Long,
        "u64" => UnsignedLong,
        "i128" => LongLong,
        "u128" => UnsignedLongLong,
        _ => Default,
    }
}

#[derive(Debug, Copy, Clone)]
enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn expect_type(got: TypeId, want: TypeId) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id(1u8 + 0), TypeId::Int) {
        std::process::exit(1);
    }
    if !expect_type(type_id(1i8 + 0), TypeId::Int) {
        std::process::exit(2);
    }
    if !expect_type(type_id(1u8 + 0), TypeId::Int) {
        std::process::exit(4);
    }
    if !expect_type(type_id(1i16 + 0), TypeId::Int) {
        std::process::exit(5);
    }
    if !expect_type(type_id(1u16 + 0), TypeId::Int) {
        std::process::exit(6);
    }

    if !expect_type(type_id(0i32 + 0u32), TypeId::UnsignedInt) {
        std::process::exit(7);
    }
    if !expect_type(type_id(0i64 + 0u64), TypeId::UnsignedLong) {
        std::process::exit(8);
    }
    if !expect_type(type_id(0i128 + 0u128), TypeId::UnsignedLongLong) {
        std::process::exit(9);
    }

    if !expect_type(type_id(0i32 + 0i64), TypeId::Long) {
        std::process::exit(10);
    }
    if !expect_type(type_id(0i64 + 0i128), TypeId::LongLong) {
        std::process::exit(11);
    }
    if !expect_type(type_id(0i32 + 0i128), TypeId::LongLong) {
        std::process::exit(12);
    }

    if !expect_type(
        type_id(E::E_POS as i32 + 0u32),
        type_id(0i32 + 0u32),
    ) {
        std::process::exit(13);
    }
    if !expect_type(type_id(E::E_POS as i32 + 0), type_id(0i32 + 0)) {
        std::process::exit(14);
    }
}