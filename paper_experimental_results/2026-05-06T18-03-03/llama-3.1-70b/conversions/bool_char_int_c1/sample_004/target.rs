fn type_id<T>(_: T) -> u8 {
    match std::any::type_name::<T>() {
        "bool" => 1,
        "char" => 2,
        "i8" => 3,
        "u8" => 4,
        "i16" => 5,
        "u16" => 6,
        "i32" => 7,
        "u32" => 8,
        "i64" => 9,
        "u64" => 10,
        "i128" => 11,
        "u128" => 12,
        _ => 99,
    }
}

enum E {
    NEG = -1,
    POS = 1,
}

fn expect_type(got: u8, want: u8) -> bool {
    got == want
}

fn main() {
    if!expect_type(type_id((_Bool)1 + 0), 7) {
        std::process::exit(1);
    }
    if!expect_type(type_id((char)1 + 0), 7) {
        std::process::exit(2);
    }
    if!expect_type(type_id((i8)1 + 0), 7) {
        std::process::exit(3);
    }
    if!expect_type(type_id((u8)1 + 0), 7) {
        std::process::exit(4);
    }
    if!expect_type(type_id((i16)1 + 0), 7) {
        std::process::exit(5);
    }
    if!expect_type(type_id((u16)1 + 0), 7) {
        std::process::exit(6);
    }

    if!expect_type(type_id((i32)0 + (u32)0), 8) {
        std::process::exit(7);
    }
    if!expect_type(type_id((i64)0 + (u64)0), 10) {
        std::process::exit(8);
    }
    if!expect_type(type_id((i128)0 + (u128)0), 12) {
        std::process::exit(9);
    }

    if!expect_type(type_id((i32)0 + (i64)0), 9) {
        std::process::exit(10);
    }
    if!expect_type(type_id((i64)0 + (i128)0), 11) {
        std::process::exit(11);
    }
    if!expect_type(type_id((i32)0 + (i128)0), 11) {
        std::process::exit(12);
    }

    if!expect_type(type_id((E)0 + 0u), type_id((i32)0 + 0u)) {
        std::process::exit(13);
    }
    if!expect_type(type_id((E)0 + 0), type_id((i32)0 + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}