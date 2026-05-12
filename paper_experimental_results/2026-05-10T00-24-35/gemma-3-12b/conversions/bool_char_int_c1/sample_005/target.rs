#[derive(Debug, Copy, Clone)]
enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(
        type_id(&(1 as _Bool) + 0),
        7,
    ) {
        std::process::exit(1);
    }
    if !expect_type(
        type_id(&(1 as char) + 0),
        7,
    ) {
        std::process::exit(2);
    }
    if !expect_type(
        type_id(&(1 as i8) + 0),
        7,
    ) {
        std::process::exit(3);
    }
    if !expect_type(
        type_id(&(1 as u8) + 0),
        7,
    ) {
        std::process::exit(4);
    }
    if !expect_type(
        type_id(&(1 as i16) + 0),
        7,
    ) {
        std::process::exit(5);
    }
    if !expect_type(
        type_id(&(1 as u16) + 0),
        7,
    ) {
        std::process::exit(6);
    }

    if !expect_type(
        type_id(0 as i32 + 0 as u32),
        8,
    ) {
        std::process::exit(7);
    }
    if !expect_type(
        type_id(0 as i64 + 0 as u64),
        10,
    ) {
        std::process::exit(8);
    }
    if !expect_type(
        type_id(0 as i128 + 0 as u128),
        12,
    ) {
        std::process::exit(9);
    }

    if !expect_type(
        type_id(0 as i32 + 0 as i64),
        9,
    ) {
        std::process::exit(10);
    }
    if !expect_type(
        type_id(0 as i64 + 0 as i128),
        11,
    ) {
        std::process::exit(11);
    }
    if !expect_type(
        type_id(0 as i32 + 0 as i128),
        11,
    ) {
        std::process::exit(12);
    }

    if !expect_type(
        type_id(&(0 as E) + 0u32),
        type_id(0 as i32 + 0u32),
    ) {
        std::process::exit(13);
    }
    if !expect_type(
        type_id(&(0 as E) + 0),
        type_id(0 as i32 + 0),
    ) {
        std::process::exit(14);
    }

    0
}

fn type_id<T>(_: &T) -> i32 {
    let t = std::any::type_name::<T>();
    match t {
        "_Bool" => 1,
        "char" => 2,
        "signed char" => 3,
        "unsigned char" => 4,
        "short" => 5,
        "unsigned short" => 6,
        "i32" => 7,
        "u32" => 8,
        "i64" => 9,
        "u64" => 10,
        "i128" => 11,
        "u128" => 12,
        _ => 99,
    }
}