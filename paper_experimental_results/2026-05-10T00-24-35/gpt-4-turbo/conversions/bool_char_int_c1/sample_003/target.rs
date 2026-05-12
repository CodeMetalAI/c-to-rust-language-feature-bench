fn main() {
    // Helper function to check the types and ensure the correct behavior
    fn expect_type<T: 'static, U: 'static>(_: &T, _: &U) -> bool {
        std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>()
    }

    if !expect_type(&(true as i32), &0i32) {
        std::process::exit(1);
    }
    if !expect_type(&(1i8 as i32), &0i32) {
        std::process::exit(2);
    }
    if !expect_type(&(1i8 as i32), &0i32) {
        std::process::exit(3);
    }
    if !expect_type(&(1u8 as i32), &0i32) {
        std::process::exit(4);
    }
    if !expect_type(&(1i16 as i32), &0i32) {
        std::process::exit(5);
    }
    if !expect_type(&(1u16 as i32), &0i32) {
        std::process::exit(6);
    }

    if !expect_type(&(0i32 as u32), &0u32) {
        std::process::exit(7);
    }
    if !expect_type(&(0i64 as u64), &0u64) {
        std::process::exit(8);
    }
    if !expect_type(&(0i128 as u128), &0u128) {
        std::process::exit(9);
    }

    if !expect_type(&(0i32 as i64), &0i64) {
        std::process::exit(10);
    }
    if !expect_type(&(0i64 as i128), &0i128) {
        std::process::exit(11);
    }
    if !expect_type(&(0i32 as i128), &0i128) {
        std::process::exit(12);
    }

    if !expect_type(&(E::EPos as u32), &(0i32 as u32)) {
        std::process::exit(13);
    }
    if !expect_type(&(E::EPos as i32), &0i32) {
        std::process::exit(14);
    }
}

enum E {
    ENeg = -1,
    EPos = 1,
}