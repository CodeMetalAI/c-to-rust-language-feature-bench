trait TypeId {
    const ID: i32;
}

impl TypeId for bool {
    const ID: i32 = 1;
}

impl TypeId for i8 {
    const ID: i32 = 3;
}

impl TypeId for u8 {
    const ID: i32 = 4;
}

impl TypeId for i16 {
    const ID: i32 = 5;
}

impl TypeId for u16 {
    const ID: i32 = 6;
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

fn type_id_of<T: TypeId>() -> i32 {
    T::ID
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

enum E {
    Neg = -1,
    Pos = 1,
}

fn main() {
    let mut code = 0;
    if !expect_type(type_id_of::<i32>(), 7) {
        code = 1;
    } else if !expect_type(type_id_of::<i32>(), 7) {
        code = 2;
    } else if !expect_type(type_id_of::<i32>(), 7) {
        code = 3;
    } else if !expect_type(type_id_of::<i32>(), 7) {
        code = 4;
    } else if !expect_type(type_id_of::<i32>(), 7) {
        code = 5;
    } else if !expect_type(type_id_of::<i32>(), 7) {
        code = 6;
    } else if !expect_type(type_id_of::<u32>(), 8) {
        code = 7;
    } else if !expect_type(type_id_of::<u64>(), 10) {
        code = 8;
    } else if !expect_type(type_id_of::<u128>(), 12) {
        code = 9;
    } else if !expect_type(type_id_of::<i64>(), 9) {
        code = 10;
    } else if !expect_type(type_id_of::<i128>(), 11) {
        code = 11;
    } else if !expect_type(type_id_of::<i128>(), 11) {
        code = 12;
    } else if !expect_type(type_id_of::<u32>(), type_id_of::<u32>()) {
        code = 13;
    } else if !expect_type(type_id_of::<i32>(), type_id_of::<i32>()) {
        code = 14;
    }
    std::process::exit(code);
}