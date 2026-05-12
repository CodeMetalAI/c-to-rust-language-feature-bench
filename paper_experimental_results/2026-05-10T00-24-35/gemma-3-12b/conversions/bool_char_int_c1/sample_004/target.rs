#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id((true) as i32 + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(('1' as i32) + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id((1i8 as i32) + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id((1u8 as i32) + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id((1i16 as i32) + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id((1u16 as i32) + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id((0i32 as i32) + (0u32 as i32)), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id((0i64 as i32) + (0u64 as i32)), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id((0i128 as i32) + (0u128 as i32)), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id((0i32 as i32) + (0i64 as i32)), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id((0i64 as i32) + (0i128 as i32)), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id((0i32 as i32) + (0i128 as i32)), 11) {
        std::process::exit(12);
    }
    
    if !expect_type(type_id((E::E_NEG as i32) + 0u32 as i32), type_id((0i32 as i32) + 0u32 as i32)) {
        std::process::exit(13);
    }
    if !expect_type(type_id((E::E_NEG as i32) + 0), type_id((0i32 as i32) + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}

fn type_id(x: i32) -> i32 {
    match x {
        _ if x == true as i32 => 1,
        _ if x == '1' as i32 => 2,
        _ if x == 1i8 as i32 => 3,
        _ if x == 1u8 as i32 => 4,
        _ if x == 1i16 as i32 => 5,
        _ if x == 1u16 as i32 => 6,
        _ if x == 0i32 as i32 => 7,
        _ if x == 0u32 as i32 => 8,
        _ if x == 0i64 as i32 => 9,
        _ if x == 0u64 as i32 => 10,
        _ if x == 0i128 as i32 => 11,
        _ if x == 0u128 as i32 => 12,
        _ => 99,
    }
}