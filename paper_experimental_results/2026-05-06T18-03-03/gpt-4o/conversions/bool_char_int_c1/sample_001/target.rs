fn expect_type(got: usize, want: usize) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id(&(true as i32 + 0)), type_id(&0)) {
        std::process::exit(1);
    }
    if !expect_type(type_id(&(1i8 + 0)), type_id(&0)) {
        std::process::exit(2);
    }
    if !expect_type(type_id(&(1i8 + 0)), type_id(&0)) {
        std::process::exit(3);
    }
    if !expect_type(type_id(&(1u8 + 0)), type_id(&0)) {
        std::process::exit(4);
    }
    if !expect_type(type_id(&(1i16 + 0)), type_id(&0)) {
        std::process::exit(5);
    }
    if !expect_type(type_id(&(1u16 + 0)), type_id(&0)) {
        std::process::exit(6);
    }

    if !expect_type(type_id(&(0i32 + 0u32)), type_id(&0u32)) {
        std::process::exit(7);
    }
    if !expect_type(type_id(&(0i64 + 0u64)), type_id(&0u64)) {
        std::process::exit(8);
    }
    if !expect_type(type_id(&(0i64 + 0u64)), type_id(&0u64)) {
        std::process::exit(9);
    }

    if !expect_type(type_id(&(0i32 + 0i64)), type_id(&0i64)) {
        std::process::exit(10);
    }
    if !expect_type(type_id(&(0i64 + 0i128)), type_id(&0i128)) {
        std::process::exit(11);
    }
    if !expect_type(type_id(&(0i32 + 0i128)), type_id(&0i128)) {
        std::process::exit(12);
    }

    if !expect_type(type_id(&(0 as i32 + 0u32)), type_id(&(0i32 + 0u32))) {
        std::process::exit(13);
    }
    if !expect_type(type_id(&(0 as i32 + 0)), type_id(&(0i32 + 0))) {
        std::process::exit(14);
    }
}

fn type_id<T: 'static>(_: &T) -> usize {
    std::any::TypeId::of::<T>().hash()
}