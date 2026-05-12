use std::process;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum CType {
    Bool,
    Char,
    SChar,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    LongLong,
    ULongLong,
}

fn type_id(t: CType) -> i32 {
    match t {
        CType::Bool => 1,
        CType::Char => 2,
        CType::SChar => 3,
        CType::UChar => 4,
        CType::Short => 5,
        CType::UShort => 6,
        CType::Int => 7,
        CType::UInt => 8,
        CType::Long => 9,
        CType::ULong => 10,
        CType::LongLong => 11,
        CType::ULongLong => 12,
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn integer_promotion(t: CType) -> CType {
    match t {
        CType::Bool
        | CType::Char
        | CType::SChar
        | CType::UChar
        | CType::Short
        | CType::UShort => CType::Int,
        _ => t,
    }
}

fn is_unsigned(t: CType) -> bool {
    matches!(t, CType::UInt | CType::ULong | CType::ULongLong)
}

fn rank(t: CType) -> i32 {
    match t {
        CType::Int | CType::UInt => 1,
        CType::Long | CType::ULong => 2,
        CType::LongLong | CType::ULongLong => 3,
        _ => 0,
    }
}

const INT_BITS: u32 = 32;
const LONG_BITS: u32 = if cfg!(target_pointer_width = "64") { 64 } else { 32 };
const LONGLONG_BITS: u32 = 64;

fn size_bits(t: CType) -> u32 {
    match t {
        CType::Int | CType::UInt => INT_BITS,
        CType::Long | CType::ULong => LONG_BITS,
        CType::LongLong | CType::ULongLong => LONGLONG_BITS,
        _ => INT_BITS,
    }
}

fn to_unsigned(t: CType) -> CType {
    match t {
        CType::Int => CType::UInt,
        CType::Long => CType::ULong,
        CType::LongLong => CType::ULongLong,
        _ => t,
    }
}

fn usual_arithmetic_conversion(a: CType, b: CType) -> CType {
    let a = integer_promotion(a);
    let b = integer_promotion(b);

    if a == b {
        return a;
    }

    if is_unsigned(a) && rank(a) >= rank(b) {
        return a;
    }
    if is_unsigned(b) && rank(b) >= rank(a) {
        return b;
    }

    if !is_unsigned(a) && !is_unsigned(b) {
        return if rank(a) >= rank(b) { a } else { b };
    }

    let (signed, unsigned) = if is_unsigned(a) { (b, a) } else { (a, b) };

    if size_bits(signed) > size_bits(unsigned) {
        return signed;
    }

    to_unsigned(signed)
}

fn main() {
    let mut code = 0;

    if !expect_type(type_id(usual_arithmetic_conversion(CType::Bool, CType::Int)), 7) {
        code = 1;
    } else if !expect_type(type_id(usual_arithmetic_conversion(CType::Char, CType::Int)), 7) {
        code = 2;
    } else if !expect_type(type_id(usual_arithmetic_conversion(CType::SChar, CType::Int)), 7) {
        code = 3;
    } else if !expect_type(type_id(usual_arithmetic_conversion(CType::UChar, CType::Int)), 7) {
        code = 4;
    } else if !expect_type(type_id(usual_arithmetic_conversion(CType::Short, CType::Int)), 7) {
        code = 5;
    } else if !expect_type(type_id(usual_arithmetic_conversion(CType::UShort, CType::Int)), 7) {
        code = 6;
    } else if !expect_type(type_id(usual_arithmetic_conversion(CType::Int, CType::UInt)), 8) {
        code = 7;
    } else if !expect_type(type_id(usual_arithmetic_conversion(CType::Long, CType::ULong)), 10) {
        code = 8;
    } else if !expect_type(
        type_id(usual_arithmetic_conversion(CType::LongLong, CType::ULongLong)),
        12,
    ) {
        code = 9;
    } else if !expect_type(type_id(usual_arithmetic_conversion(CType::Int, CType::Long)), 9) {
        code = 10;
    } else if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Long, CType::LongLong)),
        11,
    ) {
        code = 11;
    } else if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::LongLong)),
        11,
    ) {
        code = 12;
    } else if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::UInt)),
        type_id(usual_arithmetic_conversion(CType::Int, CType::UInt)),
    ) {
        code = 13;
    } else if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::Int)),
        type_id(usual_arithmetic_conversion(CType::Int, CType::Int)),
    ) {
        code = 14;
    }

    if code != 0 {
        process::exit(code);
    }
}