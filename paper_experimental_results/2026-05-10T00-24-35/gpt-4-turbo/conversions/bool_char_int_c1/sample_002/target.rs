fn main() {
    if !expect_type(type_id(1u8 as i32), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(1i8 as i32), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id(1u16 as i32), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id(1i16 as i32), 7) {
        std::process::exit(4);
    }

    if !expect_type(type_id(0i32 + 0u32), 8) {
        std::process::exit(5);
    }
    if !expect_type(type_id(0i64 + 0u64), 10) {
        std::process::exit(6);
    }

    if !expect_type(type_id(0i32 + 0i64), 11) {
        std::process::exit(7);
    }

    if !expect_type(type_id(E::EPos as i32 + 0), 8) {
        std::process::exit(8);
    }
}

fn expect_type<T: PartialEq>(got: T, want: T) -> bool {
    got == want
}

fn type_id<T>(_: T) -> usize {
    std::any::TypeId::of::<T>().into()
}

enum E {
    ENeg = -1,
    EPos = 1,
}