use std::process::exit;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    Enum,
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
        _ => 99,
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn is_signed(t: CType) -> bool {
    matches!(
        t,
        CType::Bool
            | CType::Char
            | CType::SChar
            | CType::Short
            | CType::Int
            | CType::Long
            | CType::LongLong
            | CType::Enum
    )
}

fn rank(t: CType) -> i32 {
    match t {
        CType::Int | CType::UInt => 1,
        CType::Long | CType::ULong => 2,
        CType::LongLong | CType::ULongLong => 3,
        _ => 0,
    }
}

fn bits(t: CType) -> u32 {
    match t {
        CType::Bool => 1,
        CType::Char | CType::SChar | CType::UChar => 8,
        CType::Short | CType::UShort => 16,
        CType::Int | CType::UInt => 32,
        CType::Long | CType::ULong => 64,
        CType::LongLong | CType::ULongLong => 64,
        CType::Enum => 32,
    }
}

fn unsigned_of_rank(r: i32) -> CType {
    match r {
        1 => CType::UInt,
        2 => CType::ULong,
        3 => CType::ULongLong,
        _ => CType::UInt,
    }
}

fn can_signed_represent_all_values(signed: CType, unsigned: CType) -> bool {
    let sbits = bits(signed);
    let ubits = bits(unsigned);
    let signed_max: u128 = if sbits == 0 {
        0
    } else {
        (1u128 << (sbits - 1)) - 1
    };
    let unsigned_max: u128 = if ubits == 0 {
        0
    } else {
        (1u128 << ubits) - 1
    };
    signed_max >= unsigned_max
}

fn integer_promotion(t: CType) -> CType {
    match t {
        CType::Bool
        | CType::Char
        | CType::SChar
        | CType::UChar
        | CType::Short
        | CType::UShort
        | CType::Enum => {
            let t_bits = bits(t);
            let int_bits = bits(CType::Int);
            if t_bits < int_bits {
                let unsigned_max: u128 = if t_bits == 0 {
                    0
                } else {
                    (1u128 << t_bits) - 1
                };
                let signed_int_max: u128 =
                    (1u128 << (int_bits - 1)) - 1;
                if unsigned_max <= signed_int_max {
                    CType::Int
                } else {
                    CType::UInt
                }
            } else {
                CType::Int
            }
        }
        _ => t,
    }
}

fn usual_arithmetic_conversion(a: CType, b: CType) -> CType {
    if a == b {
        return a;
    }
    let a_signed = is_signed(a);
    let b_signed = is_signed(b);
    if a_signed == b_signed {
        return if rank(a) >= rank(b) { a } else { b };
    }
    let (signed, unsigned) = if a_signed { (a, b) } else { (b, a) };
    if rank(unsigned) >= rank(signed) {
        return unsigned;
    }
    if can_signed_represent_all_values(signed, unsigned) {
        signed
    } else {
        unsigned_of_rank(rank(signed))
    }
}

fn add_type(t1: CType, t2: CType) -> CType {
    let a = integer_promotion(t1);
    let b = integer_promotion(t2);
    usual_arithmetic_conversion(a, b)
}

fn main() {
    let mut code = 0;

    if !expect_type(type_id(add_type(CType::Bool, CType::Int)), 7) {
        code = 1;
    } else if !expect_type(type_id(add_type(CType::Char, CType::Int)), 7) {
        code = 2;
    } else if !expect_type(type_id(add_type(CType::SChar, CType::Int)), 7) {
        code = 3;
    } else if !expect_type(type_id(add_type(CType::UChar, CType::Int)), 7) {
        code = 4;
    } else if !expect_type(type_id(add_type(CType::Short, CType::Int)), 7) {
        code = 5;
    } else if !expect_type(type_id(add_type(CType::UShort, CType::Int)), 7) {
        code = 6;
    } else if !expect_type(type_id(add_type(CType::Int, CType::UInt)), 8) {
        code = 7;
    } else if !expect_type(type_id(add_type(CType::Long, CType::ULong)), 10) {
        code = 8;
    } else if !expect_type(
        type_id(add_type(CType::LongLong, CType::ULongLong)),
        12,
    ) {
        code = 9;
    } else if !expect_type(type_id(add_type(CType::Int, CType::Long)), 9) {
        code = 10;
    } else if !expect_type(
        type_id(add_type(CType::Long, CType::LongLong)),
        11,
    ) {
        code = 11;
    } else if !expect_type(
        type_id(add_type(CType::Int, CType::LongLong)),
        11,
    ) {
        code = 12;
    } else if !expect_type(
        type_id(add_type(CType::Enum, CType::UInt)),
        type_id(add_type(CType::Int, CType::UInt)),
    ) {
        code = 13;
    } else if !expect_type(
        type_id(add_type(CType::Enum, CType::Int)),
        type_id(add_type(CType::Int, CType::Int)),
    ) {
        code = 14;
    }

    exit(code);
}