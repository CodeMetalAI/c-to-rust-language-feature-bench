fn type_id<T>(_: T) -> u8 {
    std::any::TypeId::of::<T>().as_u64() as u8
}

enum E { E_NEG = -1, E_POS = 1 }

fn expect_type(got: u8, want: u8) -> bool { got == want }

fn main() {
    if!expect_type(type_id(true as bool + 0), type_id(0 as i32 + 0)) {
        std::process::exit(1);
    }
    if!expect_type(type_id('a' as char + 0), type_id(0 as i32 + 0)) {
        std::process::exit(2);
    }
    if!expect_type(type_id('a' as i8 + 0), type_id(0 as i32 + 0)) {
        std::process::exit(3);
    }
    if!expect_type(type_id('a' as u8 + 0), type_id(0 as i32 + 0)) {
        std::process::exit(4);
    }
    if!expect_type(type_id(0 as i16 + 0), type_id(0 as i32 + 0)) {
        std::process::exit(5);
    }
    if!expect_type(type_id(0 as u16 + 0), type_id(0 as i32 + 0)) {
        std::process::exit(6);
    }

    if!expect_type(type_id(0 as i32 + 0 as u32), type_id(0 as u32 + 0)) {
        std::process::exit(7);
    }
    if!expect_type(type_id(0 as i64 + 0 as u64), type_id(0 as u64 + 0)) {
        std::process::exit(8);
    }
    if!expect_type(type_id(0 as i128 + 0 as u128), type_id(0 as u128 + 0)) {
        std::process::exit(9);
    }

    if!expect_type(type_id(0 as i32 + 0 as i64), type_id(0 as i64 + 0)) {
        std::process::exit(10);
    }
    if!expect_type(type_id(0 as i64 + 0 as i128), type_id(0 as i128 + 0)) {
        std::process::exit(11);
    }
    if!expect_type(type_id(0 as i32 + 0 as i128), type_id(0 as i128 + 0)) {
        std::process::exit(12);
    }

    if!expect_type(type_id(E::E_NEG as i32 + 0 as u32), type_id(0 as i32 + 0 as u32)) {
        std::process::exit(13);
    }
    if!expect_type(type_id(E::E_NEG as i32 + 0), type_id(0 as i32 + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}