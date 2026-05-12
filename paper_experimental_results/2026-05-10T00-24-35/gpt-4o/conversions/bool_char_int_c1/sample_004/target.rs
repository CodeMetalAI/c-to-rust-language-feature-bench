enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn expect_type(got: usize, want: usize) -> bool {
    got == want
}

fn main() -> i32 {
    if !expect_type(type_id(1_i32 + 0_i32), 7) {
        return 1;
    }
    if !expect_type(type_id(1_i32 + 0_i32), 7) {
        return 2;
    }
    if !expect_type(type_id(1_i32 + 0_i32), 7) {
        return 3;
    }
    if !expect_type(type_id(1_i32 + 0_i32), 7) {
        return 4;
    }
    if !expect_type(type_id(1_i32 + 0_i32), 7) {
        return 5;
    }
    if !expect_type(type_id(1_u32 + 0_u32), 8) {
        return 6;
    }

    if !expect_type(type_id(0_i32 + 0_u32), 8) {
        return 7;
    }
    if !expect_type(type_id(0_i64 + 0_u64), 10) {
        return 8;
    }
    if !expect_type(type_id(0_i128 + 0_u128), 12) {
        return 9;
    }

    if !expect_type(type_id(0_i32 + 0_i64), 9) {
        return 10;
    }
    if !expect_type(type_id(0_i64 + 0_i128), 11) {
        return 11;
    }
    if !expect_type(type_id(0_i32 + 0_i128), 11) {
        return 12;
    }

    if !expect_type(
        type_id(0 as i32 + 0_u32),
        type_id(0 as i32 + 0_u32),
    ) {
        return 13;
    }
    if !expect_type(
        type_id(0 as i32 + 0),
        type_id(0 as i32 + 0),
    ) {
        return 14;
    }

    0
}

fn type_id<T>(_value: T) -> usize {
    std::any::TypeId::of::<T>().hash_code() as usize
}