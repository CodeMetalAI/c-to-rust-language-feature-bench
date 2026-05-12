fn main() {
    if !expect_type(type_id(1u32 + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(1i8 + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id(1i8 + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id(1u8 + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id(1i16 + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id(1u16 + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id(0i32 + 0u32), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id(0i64 + 0u64), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id(0i128 + 0u128), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id(0i32 + 0i64), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id(0i64 + 0i128), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id(0i32 + 0i128), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id(E::EPos as i32 + 0u32), type_id(0i32 + 0u32)) {
        std::process::exit(13);
    }
    if !expect_type(type_id(E::ENeg as i32 + 0), type_id(0i32 + 0)) {
        std::process::exit(14);
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn type_id<T: 'static>(_: T) -> i32 {
    use std::any::TypeId;
    if TypeId::of::<T>() == TypeId::of::<bool>() { 1 }
    else if TypeId::of::<T>() == TypeId::of::<char>() { 2 }
    else if TypeId::of::<T>() == TypeId::of::<i8>() { 3 }
    else if TypeId::of::<T>() == TypeId::of::<u8>() { 4 }
    else if TypeId::of::<T>() == TypeId::of::<i16>() { 5 }
    else if TypeId::of::<T>() == TypeId::of::<u16>() { 6 }
    else if TypeId::of::<T>() == TypeId::of::<i32>() { 7 }
    else if TypeId::of::<T>() == TypeId::of::<u32>() { 8 }
    else if TypeId::of::<T>() == TypeId::of::<i64>() { 9 }
    else if TypeId::of::<T>() == TypeId::of::<u64>() { 10 }
    else if TypeId::of::<T>() == TypeId::of::<i128>() { 11 }
    else if TypeId::of::<T>() == TypeId::of::<u128>() { 12 }
    else { 99 }
}

enum E {
    ENeg = -1,
    EPos = 1,
}