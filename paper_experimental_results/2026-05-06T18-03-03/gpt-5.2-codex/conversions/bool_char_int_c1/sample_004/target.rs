use std::mem::size_of;
use std::process::exit;

#[derive(Copy, Clone, PartialEq, Eq)]
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
    EnumE,
}

#[derive(Copy, Clone)]
struct Info {
    signed: bool,
    bits: u32,
    rank: u32,
}

fn type_info(t: CType) -> Info {
    match t {
        CType::Bool => Info {
            signed: false,
            bits: 1,
            rank: 1,
        },
        CType::Char => Info {
            signed: true,
            bits: (size_of::<i8>() * 8) as u32,
            rank: 2,
        },
        CType::SChar => Info {
            signed: true,
            bits: (size_of::<i8>() * 8) as u32,
            rank: 2,
        },
        CType::UChar => Info {
            signed: false,
            bits: (size_of::<u8>() * 8) as u32,
            rank: 2,
        },
        CType::Short => Info {
            signed: true,
            bits: (size_of::<i16>() * 8) as u32,
            rank: 3,
        },
        CType::UShort => Info {
            signed: false,
            bits: (size_of::<u16>() * 8) as u32,
            rank: 3,
        },
        CType::Int => Info {
            signed: true,
            bits: (size_of::<i32>() * 8) as u32,
            rank: 4,
        },
        CType::UInt => Info {
            signed: false,
            bits: (size_of::<u32>() * 8) as u32,
            rank: 4,
        },
        CType::Long => Info {
            signed: true,
            bits: (size_of::<isize>() * 8) as u32,
            rank: 5,
        },
        CType::ULong => Info {
            signed: false,
            bits: (size_of::<usize>() * 8) as u32,
            rank: 5,
        },
        CType::LongLong => Info {
            signed: true,
            bits: (size_of::<i64>() * 8) as u32,
            rank: 6,
        },
        CType::ULongLong => Info {
            signed: false,
            bits: (size_of::<u64>() * 8) as u32,
            rank: 6,
        },
        CType::EnumE => Info {
            signed: true,
            bits: (size_of::<i32>() * 8) as u32,
            rank: 4,
        },
    }
}

fn signed_max(bits: u32) -> i128 {
    if bits == 0 {
        0
    } else {
        (1i128 << (bits - 1)) - 1
    }
}

fn signed_min(bits: u32) -> i128 {
    if bits == 0 {
        0
    } else {
        -(1i128 << (bits - 1))
    }
}

fn unsigned_max(bits: u32) -> u128 {
    if bits == 0 {
        0
    } else {
        (1u128 << bits) - 1
    }
}

fn integer_promotion(t: CType) -> CType {
    if t == CType::EnumE {
        return CType::Int;
    }
    let info = type_info(t);
    let int_info = type_info(CType::Int);

    if info.rank < int_info.rank {
        let int_max = signed_max(int_info.bits);
        let int_min = signed_min(int_info.bits);
        if info.signed {
            let min = signed_min(info.bits);
            let max = signed_max(info.bits);
            if min >= int_min && max <= int_max {
                CType::Int
            } else {
                CType::UInt
            }
        } else {
            let max = unsigned_max(info.bits);
            if (int_max as u128) >= max {
                CType::Int
            } else {
                CType::UInt
            }
        }
    } else {
        t
    }
}

fn unsigned_type_of_rank(rank: u32) -> CType {
    match rank {
        4 => CType::UInt,
        5 => CType::ULong,
        6 => CType::ULongLong,
        _ => CType::UInt,
    }
}

fn usual_arithmetic_conversion(t1: CType, t2: CType) -> CType {
    let a = integer_promotion(t1);
    let b = integer_promotion(t2);
    if a == b {
        return a;
    }
    let ia = type_info(a);
    let ib = type_info(b);

    if ia.signed == ib.signed {
        return if ia.rank >= ib.rank { a } else { b };
    }

    let (signed_t, signed_i, unsigned_t, unsigned_i) = if ia.signed {
        (a, ia, b, ib)
    } else {
        (b, ib, a, ia)
    };

    if unsigned_i.rank >= signed_i.rank {
        return unsigned_t;
    }

    let signed_max_val = signed_max(signed_i.bits);
    let unsigned_max_val = unsigned_max(unsigned_i.bits);
    if (signed_max_val as u128) >= unsigned_max_val {
        return signed_t;
    }

    unsigned_type_of_rank(signed_i.rank)
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
        CType::EnumE => 99,
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Bool, CType::Int)),
        7,
    ) {
        exit(1);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Char, CType::Int)),
        7,
    ) {
        exit(2);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::SChar, CType::Int)),
        7,
    ) {
        exit(3);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::UChar, CType::Int)),
        7,
    ) {
        exit(4);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Short, CType::Int)),
        7,
    ) {
        exit(5);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::UShort, CType::Int)),
        7,
    ) {
        exit(6);
    }

    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::UInt)),
        8,
    ) {
        exit(7);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Long, CType::ULong)),
        10,
    ) {
        exit(8);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::LongLong, CType::ULongLong)),
        12,
    ) {
        exit(9);
    }

    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::Long)),
        9,
    ) {
        exit(10);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Long, CType::LongLong)),
        11,
    ) {
        exit(11);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::LongLong)),
        11,
    ) {
        exit(12);
    }

    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::EnumE, CType::UInt)),
        type_id(usual_arithmetic_conversion(CType::Int, CType::UInt)),
    ) {
        exit(13);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::EnumE, CType::Int)),
        type_id(usual_arithmetic_conversion(CType::Int, CType::Int)),
    ) {
        exit(14);
    }

    exit(0);
}