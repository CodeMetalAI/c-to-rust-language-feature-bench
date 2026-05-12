fn type_id<T>(_x: T) -> u32 {
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
    E_NEG = -1,
    E_POS = 1,
}

fn expect_type(got: u32, want: u32) -> bool {
    got == want
}

fn main() {
    if!expect_type(type_id::<i32>((true as i32) + 0), 7) {
        std::process::exit(1);
    }
    if!expect_type(type_id::<i32>((1 as char) as i32 + 0), 7) {
        std::process::exit(2);
    }
    if!expect_type(type_id::<i32>((1 as i8) as i32 + 0), 7) {
        std::process::exit(3);
    }
    if!expect_type(type_id::<i32>((1 as u8) as i32 + 0), 7) {
        std::process::exit(4);
    }
    if!expect_type(type_id::<i32>((1 as i16) as i32 + 0), 7) {
        std::process::exit(5);
    }
    if!expect_type(type_id::<i32>((1 as u16) as i32 + 0), 7) {
        std::process::exit(6);
    }

    if!expect_type(type_id::<u32>((0 as i32) + (0 as u32)), 8) {
        std::process::exit(7);
    }
    if!expect_type(type_id::<u64>((0 as i64) + (0 as u64)), 10) {
        std::process::exit(8);
    }
    if!expect_type(type_id::<u128>((0 as i128) + (0 as u128)), 12) {
        std::process::exit(9);
    }

    if!expect_type(type_id::<i64>((0 as i32) + (0 as i64)), 9) {
        std::process::exit(10);
    }
    if!expect_type(type_id::<i128>((0 as i64) + (0 as i128)), 11) {
        std::process::exit(11);
    }
    if!expect_type(type_id::<i128>((0 as i32) + (0 as i128)), 11) {
        std::process::exit(12);
    }

    if!expect_type(type_id::<u32>((E::E_NEG as u32) + 0u), type_id::<i32>(0 + 0u)) {
        std::process::exit(13);
    }
    if!expect_type(type_id::<i32>((E::E_NEG as i32) + 0), type_id::<i32>(0 + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}