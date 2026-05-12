fn expect_type<T: 'static>(_: &T, want: u8) -> bool {
    let got = match std::any::TypeId::of::<T>() {
        x if x == std::any::TypeId::of::<bool>() => 1,
        x if x == std::any::TypeId::of::<char>() => 2,
        x if x == std::any::TypeId::of::<i8>() => 3,
        x if x == std::any::TypeId::of::<u8>() => 4,
        x if x == std::any::TypeId::of::<i16>() => 5,
        x if x == std::any::TypeId::of::<u16>() => 6,
        x if x == std::any::TypeId::of::<i32>() => 7,
        x if x == std::any::TypeId::of::<u32>() => 8,
        x if x == std::any::TypeId::of::<i64>() => 9,
        x if x == std::any::TypeId::of::<u64>() => 10,
        x if x == std::any::TypeId::of::<i128>() => 11,
        x if x == std::any::TypeId::of::<u128>() => 12,
        _ => 99,
    };
    got == want
}

fn main() {
    if !expect_type(&(1 as bool + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(&(1 as i8 + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(&(1 as i8 + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(&(1 as u8 + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(&(1 as i16 + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(&(1 as u16 + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(&(0 as i32 + 0 as u32), 8) {
        std::process::exit(7);
    }
    if !expect_type(&(0 as i64 + 0 as u64), 10) {
        std::process::exit(8);
    }
    if !expect_type(&(0 as i128 + 0 as u128), 12) {
        std::process::exit(9);
    }

    if !expect_type(&(0 as i32 + 0 as i64), 9) {
        std::process::exit(10);
    }
    if !expect_type(&(0 as i64 + 0 as i128), 11) {
        std::process::exit(11);
    }
    if !expect_type(&(0 as i32 + 0 as i128), 11) {
        std::process::exit(12);
    }

    if !expect_type(&(0 as i32 + 0u32), 7) {
        std::process::exit(13);
    }
    if !expect_type(&(0 as i32 + 0), 7) {
        std::process::exit(14);
    }
}