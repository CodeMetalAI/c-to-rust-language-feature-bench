fn main() {
    macro_rules! type_id {
        ($x:expr) => {{
            let x = $x;
            std::any::TypeId::of(&x).as_u64()
        }};
    }

    enum E { Neg = -1, Pos = 1 }

    let expect_type = |got: u64, want: u64| got == want;

    if !expect_type(type_id!((true as bool) + 0), type_id!(0i32 + 0)) {
        return 1;
    }
    if !expect_type(type_id!((1 as char) + 0), type_id!(0i32 + 0)) {
        return 2;
    }
    if !expect_type(type_id!((1 as i8) + 0), type_id!(0i32 + 0)) {
        return 3;
    }
    if !expect_type(type_id!((1 as u8) + 0), type_id!(0i32 + 0)) {
        return 4;
    }
    if !expect_type(type_id!((1 as i16) + 0), type_id!(0i32 + 0)) {
        return 5;
    }
    if !expect_type(type_id!((1 as u16) + 0), type_id!(0i32 + 0)) {
        return 6;
    }

    if !expect_type(type_id!(0i32 + 0u32), type_id!(0u32 + 0u32)) {
        return 7;
    }
    if !expect_type(type_id!(0i64 + 0u64), type_id!(0u64 + 0u64)) {
        return 8;
    }
    if !expect_type(type_id!(0i128 + 0u128), type_id!(0u128 + 0u128)) {
        return 9;
    }

    if !expect_type(type_id!(0i32 + 0i64), type_id!(0i64 + 0i64)) {
        return 10;
    }
    if !expect_type(type_id!(0i64 + 0i128), type_id!(0i128 + 0i128)) {
        return 11;
    }
    if !expect_type(type_id!(0i32 + 0i128), type_id!(0i128 + 0i128)) {
        return 12;
    }

    if !expect_type(type_id!(E::Neg as i32 + 0u32), type_id!(0i32 + 0u32)) {
        return 13;
    }
    if !expect_type(type_id!(E::Neg as i32 + 0), type_id!(0i32 + 0)) {
        return 14;
    }

    std::process::exit(0);
}