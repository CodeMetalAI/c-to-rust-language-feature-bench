enum E { NEG = -1, POS = 1 }

fn type_id<T>(_: T) -> u8 {
    std::any::type_name::<T>().split(':').last().unwrap().parse().unwrap()
}

fn expect_type(got: u8, want: u8) -> bool {
    got == want
}

fn main() {
    if!expect_type(type_id::<i32>((true as i8 + 0) as i32), 7) {
        std::process::exit(1);
    }
    if!expect_type(type_id::<i32>((1i8 as char + 0) as i32), 7) {
        std::process::exit(2);
    }
    if!expect_type(type_id::<i32>((1i8 as i8 + 0) as i32), 7) {
        std::process::exit(3);
    }
    if!expect_type(type_id::<i32>((1u8 as u8 + 0) as i32), 7) {
        std::process::exit(4);
    }
    if!expect_type(type_id::<i32>((1i16 as i16 + 0) as i32), 7) {
        std::process::exit(5);
    }
    if!expect_type(type_id::<i32>((1u16 as u16 + 0) as i32), 7) {
        std::process::exit(6);
    }

    if!expect_type(type_id::<u32>((0i32 + 0u32) as u32), 8) {
        std::process::exit(7);
    }
    if!expect_type(type_id::<u64>((0i64 + 0u64) as u64), 10) {
        std::process::exit(8);
    }
    if!expect_type(type_id::<u128>((0i128 + 0u128) as u128), 12) {
        std::process::exit(9);
    }

    if!expect_type(type_id::<i64>((0i32 + 0i64) as i64), 9) {
        std::process::exit(10);
    }
    if!expect_type(type_id::<i128>((0i64 + 0i128) as i128), 11) {
        std::process::exit(11);
    }
    if!expect_type(type_id::<i128>((0i32 + 0i128) as i128), 11) {
        std::process::exit(12);
    }

    if!expect_type(type_id::<u32>((E::NEG as u32 + 0u32) as u32), type_id::<i32>((0i32 + 0u32) as i32)) {
        std::process::exit(13);
    }
    if!expect_type(type_id::<i32>((E::NEG as i32 + 0i32) as i32), type_id::<i32>((0i32 + 0i32) as i32)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}