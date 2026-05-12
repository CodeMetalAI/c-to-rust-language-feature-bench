fn type_id<T: 'static>() -> i32 {
    match std::any::TypeId::of::<T>() {
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
    }
}

#[derive(Debug)]
enum E {
    ENEG = -1,
    EPOS = 1,
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id::<i32>(), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id::<i32>(), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id::<i32>(), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id::<i32>(), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id::<i32>(), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id::<i32>(), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id::<u32>(), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id::<u64>(), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id::<u128>(), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id::<i64>(), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id::<i128>(), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id::<i128>(), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id::<u32>(), type_id::<u32>()) {
        std::process::exit(13);
    }
    if !expect_type(type_id::<i32>(), type_id::<i32>()) {
        std::process::exit(14);
    }
}