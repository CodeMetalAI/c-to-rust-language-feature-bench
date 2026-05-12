fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn type_id<T: std::any::Any>(_x: T) -> i32 {
    if TypeId::of::<bool>() == TypeId::of::<T>() {
        1
    } else if TypeId::of::<i8>() == TypeId::of::<T>() {
        2
    } else if TypeId::of::<u8>() == TypeId::of::<T>() {
        4
    } else if TypeId::of::<i16>() == TypeId::of::<T>() {
        5
    } else if TypeId::of::<u16>() == TypeId::of::<T>() {
        6
    } else if TypeId::of::<i32>() == TypeId::of::<T>() {
        7
    } else if TypeId::of::<u32>() == TypeId::of::<T>() {
        8
    } else if TypeId::of::<i64>() == TypeId::of::<T>() {
        9
    } else if TypeId::of::<u64>() == TypeId::of::<T>() {
        10
    } else if TypeId::of::<i128>() == TypeId::of::<T>() {
        11
    } else if TypeId::of::<u128>() == TypeId::of::<T>() {
        12
    } else {
        99
    }
}

use std::any::TypeId;

enum E {
    Neg = -1,
    Pos = 1,
}

fn main() {
    // Note: Rust doesn't have signed char, so we use i8
    if !expect_type(type_id((true as u8 as i32) + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(('a' as i32) + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id((1i8 as i32) + }}0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id((1u8 as i32) + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id((1i16 as i32) + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id((1u16 as i32) + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id((0i32) + (0u32)), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id((0i64) + (0u64)), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id((0i128) + (0u128)), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id((0i32) + (0i64)), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id((0i64) + (0i128)), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id((0i32) + (0i128)), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id((0 as i32) + 0u32), type_id((0i32) + 0u32)) {
        std::process::exit(13);
    }
    if !expect_type(type_id((0 as i32) + 0), type_id((0i32) + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}