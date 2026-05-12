fn main() {
    // In Rust, all the small integer types when used in expressions typically promote to `i32`.
    // So, for simplicity, let's assume that the function `type_id` always returns `7` for them
    // to mimic the behavior of the C code (where these integer types promote to `int` in expressions).
    // The Rust compiler is more clear about the type promotions and coercion rules in arithmetic operations.
    fn type_id<T: 'static>() -> i32 {
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
            7
        } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
            8
        } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i64>() {
            11
        } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u64>() {
            12
        } else {
            99
        }
    }

    fn expect_type(got: i32, want: i32) -> bool {
        got == want
    }

    if !expect_type(type_id::<{1i32}>(), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id::<{1i8 as i32}>(), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id::<{1i8 as i32}>(), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id::<{1u8 as i32}>(), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id::<{1i16 as i32}>(), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id::<{1u16 as i32}>(), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id::<{0i32 + 0u32}>(), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id::<{0i64 + 0u64}>(), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id::<{0i32 + 0i64}>(), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id::<{E::EPos as u32}>(), type_id::<{0u32}>()) {
        std::process::exit(13);
    }
    if !expect_type(type_id::<{E::EPos as i32}>(), type_id::<{0i32}>()) {
        std::process::exit(14);
    }

    enum E {
        ENeg = -1,
        EPos = 1,
    }
}