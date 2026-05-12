fn type_id<T>(_: &T) -> i32 {
    match std::any::TypeId::of::<T>() {
        id if id == std::any::TypeId::of::<bool>() => 1,
        id if id == std::any::TypeId::of::<char>() => 2,
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

#[repr(i32)]
enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() -> i32 {
    if !expect_type(type_id(&(1 as i32)), 7) {
        return 1;
    }
    if !expect_type(type_id(&(1 as i32)), 7) {
        return 2;
    }
    if !expect_type(type_id(&(1 as i32)), 7) {
        return 3;
    }
    if !expect_type(type_id(&(1 as i32)), 7) {
        return 4;
    }
    if !expect_type(type_id(&(1 as i32)), 7) {
        return 5;
    }
    if !expect_type(type_id(&(1 as i32)), 7) {
        return 6;
    }

    if !expect_type(type_id(&(0u32 + 0)), 8) {
        return 7;
    }
    if !expect_type(type_id(&(0u64 + 0)), 10) {
        return 8;
    }
    if !expect_type(type_id(&(0u128 + 0)), 12) {
        return 9;
    }

    if !expect_type(type_id(&(0i64 + 0i32)), 9) {
        return 10;
    }
    if !expect_type(type_id(&(0i64 + 0i128)), 11) {
        return 11;
    }
    if !expect_type(type_id(&(0i128 + 0i32)), 11) {
        return 12;
    }

    if !expect_type(type_id(&(0 as i32 + 0u32)), type_id(&(0i32 + 0u32))) {
        return 13;
    }
    if !expect_type(type_id(&(0 as i32 + 0)), type_id(&(0i32 + 0))) {
        return 14;
    }

    0
}