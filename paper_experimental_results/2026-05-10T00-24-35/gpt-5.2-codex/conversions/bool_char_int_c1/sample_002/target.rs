use std::process::exit;

trait TypeId {
    const ID: i32;
}

struct CBool;
struct CChar;
struct CSChar;
struct CUChar;
struct CShort;
struct CUShort;
struct CInt;
struct CUInt;
struct CLong;
struct CULong;
struct CLongLong;
struct CULongLong;

impl TypeId for CBool {
    const ID: i32 = 1;
}
impl TypeId for CChar {
    const ID: i32 = 2;
}
impl TypeId for CSChar {
    const ID: i32 = 3;
}
impl TypeId for CUChar {
    const ID: i32 = 4;
}
impl TypeId for CShort {
    const ID: i32 = 5;
}
impl TypeId for CUShort {
    const ID: i32 = 6;
}
impl TypeId for CInt {
    const ID: i32 = 7;
}
impl TypeId for CUInt {
    const ID: i32 = 8;
}
impl TypeId for CLong {
    const ID: i32 = 9;
}
impl TypeId for CULong {
    const ID: i32 = 10;
}
impl TypeId for CLongLong {
    const ID: i32 = 11;
}
impl TypeId for CULongLong {
    const ID: i32 = 12;
}

fn type_id<T: TypeId>() -> i32 {
    T::ID
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

#[repr(i32)]
enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn main() {
    if !expect_type(type_id::<CInt>(), 7) {
        exit(1);
    }
    if !expect_type(type_id::<CInt>(), 7) {
        exit(2);
    }
    if !expect_type(type_id::<CInt>(), 7) {
        exit(3);
    }
    if !expect_type(type_id::<CInt>(), 7) {
        exit(4);
    }
    if !expect_type(type_id::<CInt>(), 7) {
        exit(5);
    }
    if !expect_type(type_id::<CInt>(), 7) {
        exit(6);
    }

    if !expect_type(type_id::<CUInt>(), 8) {
        exit(7);
    }
    if !expect_type(type_id::<CULong>(), 10) {
        exit(8);
    }
    if !expect_type(type_id::<CULongLong>(), 12) {
        exit(9);
    }

    if !expect_type(type_id::<CLong>(), 9) {
        exit(10);
    }
    if !expect_type(type_id::<CLongLong>(), 11) {
        exit(11);
    }
    if !expect_type(type_id::<CLongLong>(), 11) {
        exit(12);
    }

    if !expect_type(type_id::<CUInt>(), type_id::<CUInt>()) {
        exit(13);
    }
    if !expect_type(type_id::<CInt>(), type_id::<CInt>()) {
        exit(14);
    }

    exit(0);
}