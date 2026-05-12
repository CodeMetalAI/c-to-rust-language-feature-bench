fn type_id<T>(_: T) -> u8 {
    std::any::TypeId::of::<T>().as_u64() as u8
}

enum E { E_NEG = -1, E_POS = 1 }

fn expect_type(got: u8, want: u8) -> bool {
    got == want
}

fn main() {
    if!expect_type(type_id((_Bool)1 + 0), type_id(0i32)) {
        std::process::exit(1);
    }
    if!expect_type(type_id((char)1 + 0), type_id(0i32)) {
        std::process::exit(2);
    }
    if!expect_type(type_id((i8)1 + 0), type_id(0i32)) {
        std::process::exit(3);
    }
    if!expect_type(type_id((u8)1 + 0), type_id(0i32)) {
        std::process::exit(4);
    }
    if!expect_type(type_id((i16)1 + 0), type_id(0i32)) {
        std::process::exit(5);
    }
    if!expect_type(type_id((u16)1 + 0), type_id(0i32)) {
        std::process::exit(6);
    }

    if!expect_type(type_id((0i32 + 0u32)), type_id(0u32)) {
        std::process::exit(7);
    }
    if!expect_type(type_id((0i64 + 0u64)), type_id(0u64)) {
        std::process::exit(8);
    }
    if!expect_type(type_id((0i64 + 0u64)), type_id(0u64)) {
        std::process::exit(9);
    }

    if!expect_type(type_id((0i32 + 0i64)), type_id(0i64)) {
        std::process::exit(10);
    }
    if!expect_type(type_id((0i64 + 0i64)), type_id(0i64)) {
        std::process::exit(11);
    }
    if!expect_type(type_id((0i32 + 0i64)), type_id(0i64)) {
        std::process::exit(12);
    }

    if!expect_type(type_id((E::E_NEG as i32 + 0u32)), type_id(0i32 + 0u32)) {
        std::process::exit(13);
    }
    if!expect_type(type_id((E::E_NEG as i32 + 0i32)), type_id(0i32 + 0i32)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}