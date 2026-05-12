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

struct Info {
    signed: bool,
    rank: u8,
    bits: u32,
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

fn promote(t: CType) -> CType {
    match t {
        CType::Bool
        | CType::Char
        | CType::SChar
        | CType::UChar
        | CType::Short
        | CType::UShort
        | CType::EnumE => CType::Int,
        _ => t,
    }
}

fn info(t: CType) -> Info {
    match t {
        CType::Bool => Info {
            signed: false,
            rank: 1,
            bits: 1,
        },
        CType::Char => Info {
            signed: true,
            rank: 2,
            bits: 8,
        },
        CType::SChar => Info {
            signed: true,
            rank: 2,
            bits: 8,
        },
        CType::UChar => Info {
            signed: false,
            rank: 2,
            bits: 8,
        },
        CType::Short => Info {
            signed: true,
            rank: 3,
            bits: 16,
        },
        CType::UShort => Info {
            signed: false,
            rank: 3,
            bits: 16,
        },
        CType::Int => Info {
            signed: true,
            rank: 4,
            bits: 32,
        },
        CType::UInt => Info {
            signed: false,
            rank: 4,
            bits: 32,
        },
        CType::Long => Info {
            signed: true,
            rank: 5,
            bits: 64,
        },
        CType::ULong => Info {
            signed: false,
            rank: 5,
            bits: 64,
        },
        CType::LongLong => Info {
            signed: true,
            rank: 6,
            bits: 64,
        },
        CType::ULongLong => Info {
            signed: false,
            rank: 6,
            bits: 64,
        },
        CType::EnumE => Info {
            signed: true,
            rank: 4,
            bits: 32,
        },
    }
}

fn make_unsigned(t: CType) -> CType {
    match t {
        CType::Int => CType::UInt,
        CType::Long => CType::ULong,
        CType::LongLong => CType::ULongLong,
        _ => t,
    }
}

fn usual_arithmetic_conversion(a: CType, b: CType) -> CType {
    let a = promote(a);
    let b = promote(b);

    if a == b {
        return a;
    }

    let ai = info(a);
    let bi = info(b);

    if ai.signed == bi.signed {
        return if ai.rank >= bi.rank { a } else { b };
    }

    let (signed_t, unsigned_t, signed_info, unsigned_info) = if ai.signed {
        (a, b, ai, bi)
    } else {
        (b, a, bi, ai)
    };

    if unsigned_info.rank >= signed_info.rank {
        return unsigned_t;
    }

    if signed_info.bits > unsigned_info.bits {
        return signed_t;
    }

    make_unsigned(signed_t)
}

fn type_id_of_expr(a: CType, b: CType) -> i32 {
    let t = usual_arithmetic_conversion(a, b);
    type_id(t)
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id_of_expr(CType::Bool, CType::Int), 7) {
        exit(1);
    }
    if !expect_type(type_id_of_expr(CType::Char, CType::Int), 7) {
        exit(2);
    }
    if !expect_type(type_id_of_expr(CType::SChar, CType::Int), 7) {
        exit(3);
    }
    if !expect_type(type_id_of_expr(CType::UChar, CType::Int), 7) {
        exit(4);
    }
    if !expect_type(type_id_of_expr(CType::Short, CType::Int), 7) {
        exit(5);
    }
    if !expect_type(type_id_of_expr(CType::UShort, CType::Int), 7) {
        exit(6);
    }

    if !expect_type(type_id_of_expr(CType::Int, CType::UInt), 8) {
        exit(7);
    }
    if !expect_type(type_id_of_expr(CType::Long, CType::ULong), 10) {
        exit(8);
    }
    if !expect_type(type_id_of_expr(CType::LongLong, CType::ULongLong), 12) {
        exit(9);
    }

    if !expect_type(type_id_of_expr(CType::Int, CType::Long), 9) {
        exit(10);
    }
    if !expect_type(type_id_of_expr(CType::Long, CType::LongLong), 11) {
        exit(11);
    }
    if !expect_type(type_id_of_expr(CType::Int, CType::LongLong), 11) {
        exit(12);
    }

    if !expect_type(
        type_id_of_expr(CType::EnumE, CType::UInt),
        type_id_of_expr(CType::Int, CType::UInt),
    ) {
        exit(13);
    }
    if !expect_type(
        type_id_of_expr(CType::EnumE, CType::Int),
        type_id_of_expr(CType::Int, CType::Int),
    ) {
        exit(14);
    }

    exit(0);
}