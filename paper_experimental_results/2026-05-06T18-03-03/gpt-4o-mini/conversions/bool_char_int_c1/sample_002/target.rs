fn type_id<T>(_: T) -> usize {
    std::any::type_name::<T>().to_string().as_str().into()
}

fn expect_type(got: usize, want: usize) -> bool {
    got == want
}

fn main() -> i32 {
    if !expect_type(type_id(1u8), 7) { return 1; }
    if !expect_type(type_id(1i8), 7) { return 2; }
    if !expect_type(type_id(1i8), 7) { return 3; }
    if !expect_type(type_id(1u8), 7) { return 4; }
    if !expect_type(type_id(1i16), 7) { return 5; }
    if !expect_type(type_id(1u16), 7) { return 6; }

    if !expect_type(type_id(0i32 + 0u32), 8) { return 7; }
    if !expect_type(type_id(0i64 + 0u64), 10) { return 8; }
    if !expect_type(type_id(0i128 + 0u128), 12) { return 9; }

    if !expect_type(type_id(0i32 + 0i64), 9) { return 10; }
    if !expect_type(type_id(0i64 + 0i128), 11) { return 11; }
    if !expect_type(type_id(0i32 + 0i128), 11) { return 12; }

    if !expect_type(type_id(0u8), type_id(0u32)) { return 13; }
    if !expect_type(type_id(0i32), type_id(0i32)) { return 14; }

    0
}