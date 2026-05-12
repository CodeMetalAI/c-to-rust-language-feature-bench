fn type_id<T>(_x: T) -> i32 {
    match std::any::TypeId::of::<T>() {
        t if t == std::any::TypeId::of::<bool>() => 1,
        t if t == std::any::TypeId::of::<char>() => 2,
        t if t == std::any::TypeId::of::<i8>() => 3,
        t if t == std::any::TypeId::of::<u8>() => 4,
        t if t == std::any::TypeId::of::<i16>() => 5,
        t if t == std::any::TypeId::of::<u16>() => 6,
        t if t == std::any::TypeId::of::<i32>() => 7,
        t if t == std::any::TypeId::of::<u32>() => 8,
        t if t == std::any::TypeId::of::<i64>() => 9,
        t if t == std::any::TypeId::of::<u64>() => 10,
        t if t == std::any::TypeId::of::<i128>() => 11,
        t if t == std::any::TypeId::of::<u128>() => 12,
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
    if !expect_type(type_id((1 as i32) + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id((1 as i8) + 0), 7) {
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

    if !expect_type(type_id((E::E_NEG as i32) + 0u32), type_id((0 as i32) + 0u32)) {
        std::process::exit(13);
    }
    if !expect_type(type_id((E::E_NEG as i32) + 0), type_id((0 as i32) + 0)) {
        std::process::exit(14);
    }
}