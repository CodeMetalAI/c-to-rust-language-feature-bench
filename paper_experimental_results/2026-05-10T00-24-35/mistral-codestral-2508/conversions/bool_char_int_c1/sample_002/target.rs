fn type_id<T>(_: T) -> i32 {
    match std::any::TypeId::of::<T>() {
        id if id == std::any::TypeId::of::<bool>() => 1,
        id if id == std::any::TypeId::of::<i8>() => 2,
        id if id == std::any::TypeId::of::<i8>() => 3,
        id if id == std::any::TypeId::of::<u8>() => 4,
        id if id == std::any::TypeId::of::<i16>() => 5,
        id if id == std::any::TypeId::of::<u16>() => 6,
        id if id == std::any::TypeId::of::<i32>() => 7,
        id if id == std::any::TypeId::of::<u32>() => 8,
        id if id == std::any::TypeId::of::<i64>() => 9,
        id if id == std::any::TypeId::of::<u64>() => 10,
        id if id == std::any::TypeId::of::<i128>() => 11,
        id if id == std::any::TypeId::of::<u128>() => 12,
        _ => 99,
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

#[derive(Clone, Copy)]
enum E {
    ENeg = -1,
    EPos = 1,
}

fn main() {
    if !expect_type(type_id(1i32 + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(1i8 as i32 + 0), 7) {
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

    if !expect_type(type_id(E::ENeg as i32 + 0u32), type_id(0i32 + 0u32)) {
        std::process::exit(13);
    }
    if !expect_type(type_id(E::ENeg as i32 + 0), type_id(0i32 + 0)) {
        std::process::exit(14);
    }
}