#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

#[derive(Copy, Clone)]
struct TypeInfo {
    signed: bool,
    rank: i32,
    bits: u32,
}

fn info(t: CType) -> TypeInfo {
    match t {
        CType::Int => TypeInfo { signed: true, rank: 3, bits: 32 },
        CType::UInt => TypeInfo { signed: false, rank: 3, bits: 32 },
        CType::Long => TypeInfo { signed: true, rank: 4, bits: 64 },
        CType::ULong => TypeInfo { signed: false, rank: 4, bits: 64 },
        CType::LongLong => TypeInfo { signed: true, rank: 5, bits: 64 },
        CType::ULongLong => TypeInfo { signed: false, rank: 5, bits: 64 },
        _ => TypeInfo { signed: true, rank: 0, bits: 0 },
    }
}

fn unsigned_version(t: CType) -> CType {
    match t {
        CType::Int => CType::UInt,
        CType::Long => CType::ULong,
        CType::LongLong => CType::ULongLong,
        _ => t,
    }
}

fn integer_promotion(t: CType) -> CType {
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

fn usual_arithmetic_conversion(a: CType, b: CType) -> CType {
    let a = integer_promotion(a);
    let b = integer_promotion(b);
    if a == b {
        return a;
    }

    let ia = info(a);
    let ib = info(b);

    if ia.signed == ib.signed {
        return if ia.rank >= ib.rank { a } else { b };
    }

    // one signed, one unsigned
    let (signed_t, unsigned_t, signed_info, unsigned_info) = if ia.signed {
        (a, b, ia, ib)
    } else {
        (b, a, ib, ia)
    };

    if unsigned_info.rank >= signed_info.rank {
        return unsigned_t;
    }

    if signed_info.bits > unsigned_info.bits {
        return signed_t;
    }

    unsigned_version(signed_t)
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id(usual_arithmetic_conversion(CType::Bool, CType::Int)), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(usual_arithmetic_conversion(CType::Char, CType::Int)), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id(usual_arithmetic_conversion(CType::SChar, CType::Int)), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id(usual_arithmetic_conversion(CType::UChar, CType::Int)), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id(usual_arithmetic_conversion(CType::Short, CType::Int)), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id(usual_arithmetic_conversion(CType::UShort, CType::Int)), 7) {
        std::process::exit(6);
    }

    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::UInt)),
        8,
    ) {
        std::process::exit(7);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Long, CType::ULong)),
        10,
    ) {
        std::process::exit(8);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::LongLong, CType::ULongLong)),
        12,
    ) {
        std::process::exit(9);
    }

    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::Long)),
        9,
    ) {
        std::process::exit(10);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Long, CType::LongLong)),
        11,
    ) {
        std::process::exit(11);
    }
    if !expect_type(
        type_id(usual_arithmetic_conversion(CType::Int, CType::LongLong)),
        11,
    ) {
        std::process::exit(12);
    }

    let t1 = type_id(usual_arithmetic_conversion(CType::EnumE, CType::UInt));
    let t2 = type_id(usual_arithmetic_conversion(CType::Int, CType::UInt));
    if !expect_type(t1, t2) {
        std::process::exit(13);
    }

    let t3 = type_id(usual_arithmetic_conversion(CType::EnumE, CType::Int));
    let t4 = type_id(usual_arithmetic_conversion(CType::Int, CType::Int));
    if !expect_type(t3, t4) {
        std::process::exit(14);
    }

    std::process::exit(0);
}