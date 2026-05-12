fn expect_type(got: usize, want: usize) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id(1u8 as bool as i32 + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(1u8 as char as i32 + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id(1i8 as i32 + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id(1u8 as i32 + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id(1i16 as i32 + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id(1u16 as i32 + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id(0i32 + 0u32 as i32), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id(0i64 + 0u64 as i64), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id(0i64 + 0u64 as i64), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id(0i32 + 0i64 as i32), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id(0i64 + 0i128 as i64), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id(0i32 + 0i128 as i32), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id(0i32 + 0u32 as i32), type_id(0i32 + 0u32 as i32)) {
        std::process::exit(13);
    }
    if !expect_type(type_id(0i32 + 0), type_id(0i32 + 0)) {
        std::process::exit(14);
    }
}

fn type_id<T: 'static>(_val: T) -> usize {
    use std::any::TypeId;
    match TypeId::of::<T>() {
        id if id == TypeId::of::<bool>() => 1,
        id if id == TypeId::of::<char>() => 2,
        id if id == TypeId::of::<i8>() => 3,
        id if id == TypeId::of::<u8>() => 4,
        id if id == TypeId::of::<i16>() => 5,
        id if id == TypeId::of::<u16>() => 6,
        id if id == TypeId::of::<i32>() => 7,
        id if id == TypeId::of::<u32>() => 8,
        id if id == TypeId::of::<i64>() => 9,
        id if id == TypeId::of::<u64>() => 10,
        id if id == TypeId::of::<i128>() => 11,
        id if id == TypeId::of::<u128>() => 12,
        _ => 99,
    }
}