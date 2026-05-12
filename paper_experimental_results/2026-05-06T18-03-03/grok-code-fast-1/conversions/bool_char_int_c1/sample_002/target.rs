trait TypeId {
    const ID: i32;
}

impl TypeId for bool {
    const ID: i32 = 1;
}

impl TypeId for i32 {
    const ID: i32 = 7;
}

impl TypeId for u32 {
    const ID: i32 = 8;
}

impl TypeId for i64 {
    const ID: i32 = 9;
}

impl TypeId for u64 {
    const ID: i32 = 10;
}

impl TypeId for i128 {
    const ID: i32 = 11;
}

impl TypeId for u128 {
    const ID: i32 = 12;
}

fn get_id<T: TypeId>(_ : T) -> i32 {
    T::ID
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

#[derive(Clone, Copy)]
enum E {
    ENeg = -1,
    EPos = 1,
}

fn main() {
    if !expect_type(get_id((true as i32) + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(get_id((1i8 as i32) + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(get_id((1i8 as i32) + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(get_id((1u8 as i32) + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(get_id((1i16 as i32) + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(get_id((1u16 as i32) + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(get_id(0i32 + 0u32), 8) {
        std::process::exit(7);
    }
    if !expect_type(get_id(0i64 + 0u64), 10) {
        std::process::exit(8);
    }
    if !expect_type(get_id(0i128 + 0u128), 12) {
        std::process::exit(9);
    }

    if !expect_type(get_id(0i32 + 0i64), 9) {
        std::process::exit(10);
    }
    if !expect_type(get_id(0i64 + 0i128), 11) {
        std::process::exit(11);
    }
    if !expect_type(get_id(0i32 + 0i128), 11) {
        std::process::exit(12);
    }

    if !expect_type(get_id((E::ENeg as i32) + 0u32), get_id(0i32 + 0u32)) {
        std::process::exit(13);
    }
    if !expect_type(get_id((E::ENeg as i32) + 0i32), get_id(0i32 + 0i32)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}