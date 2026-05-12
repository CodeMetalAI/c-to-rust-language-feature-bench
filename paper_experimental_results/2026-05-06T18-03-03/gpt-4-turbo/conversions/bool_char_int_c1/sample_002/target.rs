fn main() {
    fn expect_type<T: 'static>(_: &T, want: usize) -> bool {
        std::any::TypeId::of::<T>() == std::any::TypeId::of::<want>()
    }

    macro_rules! type_id {
        ($expr:expr) => {
            std::any::TypeId::of_val(&$expr)
        };
    }

    let int_type_id = type_id!(0i32);
    let uint_type_id = type_id!(0u32);
    let long_type_id = type_id!(0i64);
    let ulong_type_id = type_id!(0u64);
    let longlong_type_id = type_id!(0i128);
    let ulonglong_type_id = type_id!(0u128);

    if !expect_type(&type_id!((true as i32)), int_type_id) {
        std::process::exit(1);
    }
    if !expect_type(&type_id!((1i8 as i32)), int_type_id) {
        std::process::exit(2);
    }
    if !expect_type(&type_id!((1i8 as i32)), int_type_id) {
        std::process::exit(3);
    }
    if !expect_type(&type_id!((1u8 as i32)), int_type_id) {
        std::process::exit(4);
    }
    if !expect_type(&type_id!((1i16 as i32)), int_type_id) {
        std::process::exit(5);
    }
    if !expect_type(&type_id!((1u16 as i32)), int_type_id) {
        std::process::exit(6);
    }

    if !expect_type(&type_id!((0i32 + 0u32)), uint_type_id) {
        std::process::exit(7);
    }
    if !expect_type(&type_id!((0i64 + 0u64)), ulong_type_id) {
        std::process::exit(8);
    }
    if !expect_type(&type_id!((0i128 + 0u128)), ulonglong_type_id) {
        std::process::exit(9);
    }

    if !expect_type(&type_id!((0i32 + 0i64)), long_type_id) {
        std::process::exit(10);
    }
    if !expect_type(&type_id!((0i64 + 0i128)), longlong_type_id) {
        std::process::exit(11);
    }
    if !expect_type(&type_id!((0i32 + 0i128)), longlong_type_id) {
        std::process::exit(12);
    }

    enum E {
        Neg = -1,
        Pos = 1,
    }

    if !expect_type(&type_id!((E::Pos as u32)), type_id!((0i32 as u32))) {
        std::process::exit(13);
    }
    if !expect_type(&type_id!((E::Pos as i32)), type_id!(0i32)) {
        std::process::exit(14);
    }
}