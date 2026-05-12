fn type_id<T>(_: T) -> i32 {
    match std::any::TypeId::of::<T>() {
        _ if std::any::TypeId::of::<bool>() == std::any::TypeId::of::<T>() => 1,
        _ if std::any::TypeId::of::<char>() == std::any::TypeId::of::<T>() => 2,
        _ if std::any::TypeId::of::<i8>() == std::any::TypeId::of::<T>() => 3,
        _ if std::any::TypeId::of::<u8>() == std::any::TypeId::of::<T>() => 4,
        _ if std::any::TypeId::of::<i16>() == std::any::TypeId::of::<T>() => 5,
        _ if std::any::TypeId::of::<u16>() == std::any::TypeId::of::<T>() => 6,
        _ if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() => 7,
        _ if std::any::TypeId::of::<u32>() == std::any::TypeId::of::<T>() => 8,
        _ if std::any::TypeId::of::<i64>() == std::any::TypeId::of::<T>() => 9,
        _ if std::any::TypeId::of::<u64>() == std::any::TypeId::of::<T>() => 10,
        _ if std::any::TypeId::of::<i128>() == std::any::TypeId::of::<T>() => 11,
        _ if std::any::TypeId::of::<u128>() == std::any::TypeId::of::<T>() => 12,
        _ => 99,
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

#[derive(Debug, Clone, Copy)]
enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn main() {
    if !expect_type(type_id((1 as bool) + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id((1 as char) + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id((1 as i8) + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id((1 as u8) + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id((1 as i16) + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id((1 as u16) + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id((0 as i32) + (0 as u32)), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id((0 as i64) + (0 as u64)), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id((0 as i128) + (0 as u128)), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id((0 as i32) + (0 as i64)), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id((0 as i64) + (0 as i128)), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id((0 as i32) + (0 as i128)), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id((E::E_NEG as i32) + 0u), type_id((0 as i32) + 0u)) {
        std::process::exit(13);
    }
    if !expect_type(type_id((E::E_NEG as i32) + 0), type_id((0 as i32) + 0)) {
        std::process::exit(14);
    }
}