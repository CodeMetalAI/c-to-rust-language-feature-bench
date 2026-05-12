trait TypeId {
    const ID: i32;
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

fn type_id_val<T: TypeId>(_: &T) -> i32 {
    T::ID
}

macro_rules! type_id {
    ($e:expr) => {{
        let x = $e;
        type_id_val(&x)
    }};
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

#[repr(i32)]
enum E {
    Neg = -1,
    Pos = 1,
}

fn main() {
    if !expect_type(type_id!((true as i32) + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id!((1i8) + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id!((1i8) + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id!((1u8) + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id!((1i16) + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id!((1u16) + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id!((0i32 as u32) + 0u32), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id!((0i64 as u64) + 0u64), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id!((0i128 as u128) + 0u128), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id!(0i32 + 0i64), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id!(0i64 + 0i128), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id!(0i32 + 0i128), 11) {
        std::process::exit(12);
    }

    let id1 = type_id!((0i32 as u32) + 0u32);
    let id2 = type_id!((0i32 as u32) + 0u32);
    if !expect_type(id1, id2) {
        std::process::exit(13);
    }
    let id3 = type_id!(0i32 + 0i32);
    let id4 = type_id!(0i32 + 0i32);
    if !expect_type(id3, id4) {
        std::process::exit(14);
    }

    std::process::exit(0);
}