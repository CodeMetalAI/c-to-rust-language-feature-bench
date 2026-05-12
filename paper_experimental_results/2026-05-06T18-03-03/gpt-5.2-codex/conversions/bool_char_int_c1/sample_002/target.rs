#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    LLong,
    ULLong,
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
        CType::LLong => 11,
        CType::ULLong => 12,
        CType::EnumE => 99, // not used directly
    }
}

fn promote_integer(t: CType) -> CType {
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

fn is_unsigned(t: CType) -> bool {
    matches!(t, CType::UInt | CType::ULong | CType::ULLong)
}

fn rank(t: CType) -> i32 {
    match t {
        CType::Int | CType::UInt => 1,
        CType::Long | CType::ULong => 2,
        CType::LLong | CType::ULLong => 3,
        _ => 0,
    }
}

fn usual_arithmetic(t1: CType, t2: CType) -> CType {
    if t1 == t2 {
        return t1;
    }
    let p1 = promote_integer(t1);
    let p2 = promote_integer(t2);

    if p1 == p2 {
        return p1;
    }

    let u1 = is_unsigned(p1);
    let u2 = is_unsigned(p2);

    if !u1 && !u2 {
        return if rank(p1) >= rank(p2) { p1 } else { p2 };
    }
    if u1 && u2 {
        return if rank(p1) >= rank(p2) { p1 } else { p2 };
    }

    let (u, s) = if u1 { (p1, p2) } else { (p2, p1) };

    if rank(u) >= rank(s) {
        u
    } else {
        s
    }
}

fn type_of_add(t1: CType, t2: CType) -> CType {
    usual_arithmetic(t1, t2)
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id(type_of_add(CType::Bool, CType::Int)), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(type_of_add(CType::Char, CType::Int)), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id(type_of_add(CType::SChar, CType::Int)), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id(type_of_add(CType::UChar, CType::Int)), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id(type_of_add(CType::Short, CType::Int)), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id(type_of_add(CType::UShort, CType::Int)), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id(type_of_add(CType::Int, CType::UInt)), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id(type_of_add(CType::Long, CType::ULong)), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id(type_of_add(CType::LLong, CType::ULLong)), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id(type_of_add(CType::Int, CType::Long)), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id(type_of_add(CType::Long, CType::LLong)), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id(type_of_add(CType::Int, CType::LLong)), 11) {
        std::process::exit(12);
    }

    if !expect_type(
        type_id(type_of_add(CType::EnumE, CType::UInt)),
        type_id(type_of_add(CType::Int, CType::UInt)),
    ) {
        std::process::exit(13);
    }
    if !expect_type(
        type_id(type_of_add(CType::EnumE, CType::Int)),
        type_id(type_of_add(CType::Int, CType::Int)),
    ) {
        std::process::exit(14);
    }

    std::process::exit(0);
}