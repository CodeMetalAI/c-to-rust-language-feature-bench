enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn expect_type(got: usize, want: usize) -> bool {
    got == want
}

fn type_id<T>() -> usize {
    std::any::type_name::<T>().to_string().as_str().into()
}

fn main() -> i32 {
    if !expect_type(type_id::<bool>(), 7) {
        return 1;
    }
    if !expect_type(type_id::<char>(), 7) {
        return 2;
    }
    if !expect_type(type_id::<i8>(), 7) {
        return 3;
    }
    if !expect_type(type_id::<u8>(), 7) {
        return 4;
    }
    if !expect_type(type_id::<i16>(), 7) {
        return 5;
    }
    if !expect_type(type_id::<u16>(), 7) {
        return 6;
    }

    if !expect_type(type_id::<i32>() + type_id::<u32>(), 8) {
        return 7;
    }
    if !expect_type(type_id::<i64>() + type_id::<u64>(), 10) {
        return 8;
    }
    if !expect_type(type_id::<i128>() + type_id::<u128>(), 12) {
        return 9;
    }

    if !expect_type(type_id::<i32>() + type_id::<i64>(), 9) {
        return 10;
    }
    if !expect_type(type_id::<i64>() + type_id::<i128>(), 11) {
        return 11;
    }
    if !expect_type(type_id::<i32>() + type_id::<i128>(), 11) {
        return 12;
    }

    if !expect_type(type_id::<E>() as usize + 0u32 as usize, type_id::<i32>() as usize + 0u32 as usize) {
        return 13;
    }
    if !expect_type(type_id::<E>() as usize + 0, type_id::<i32>() as usize + 0) {
        return 14;
    }

    return 0;
}