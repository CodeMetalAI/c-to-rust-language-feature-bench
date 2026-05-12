fn type_id<T: std::fmt::Debug>(_x: T) -> u32 {
    std::any::TypeId::of::<T>().as_u32()
}

enum E { E_NEG = -1, E_POS = 1 }

fn expect_type(got: u32, want: u32) -> bool { got == want }

fn main() {
    if!expect_type(type_id::<i32>((true as i32) + 0), type_id::<i32>(0)) {
        std::process::exit(1);
    }
    if!expect_type(type_id::<i32>((1 as i8 as i32) + 0), type_id::<i32>(0)) {
        std::process::exit(2);
    }
    if!expect_type(type_id::<i32>((1 as i8 as i32) + 0), type_id::<i32>(0)) {
        std::process::exit(3);
    }
    if!expect_type(type_id::<i32>((1 as u8 as i32) + 0), type_id::<i32>(0)) {
        std::process::exit(4);
    }
    if!expect_type(type_id::<i32>((1 as i16 as i32) + 0), type_id::<i32>(0)) {
        std::process::exit(5);
    }
    if!expect_type(type_id::<i32>((1 as u16 as i32) + 0), type_id::<i32>(0)) {
        std::process::exit(6);
    }

    if!expect_type(type_id::<u32>(0 + 0u), type_id::<u32>(0)) {
        std::process::exit(7);
    }
    if!expect_type(type_id::<u64>(0 + 0u64), type_id::<u64>(0)) {
        std::process::exit(8);
    }
    if!expect_type(type_id::<u128>(0 + 0u128), type_id::<u128>(0)) {
        std::process::exit(9);
    }

    if!expect_type(type_id::<i64>(0 + 0i64), type_id::<i64>(0)) {
        std::process::exit(10);
    }
    if!expect_type(type_id::<i128>(0 + 0i128), type_id::<i128>(0)) {
        std::process::exit(11);
    }
    if!expect_type(type_id::<i128>(0 + 0i128), type_id::<i128>(0)) {
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