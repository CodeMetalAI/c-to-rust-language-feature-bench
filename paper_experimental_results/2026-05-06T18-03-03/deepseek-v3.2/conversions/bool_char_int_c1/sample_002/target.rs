fn type_id<T>(_x: T) -> u32 {
    use std::any::TypeId;
    let type_id = TypeId::of::<T>();
    
    if type_id == TypeId::of::<bool>() { 1 }
    else if type_id == TypeId::of::<i8>() { 2 }
    else if type_id == TypeId::of::<u8>() { 4 }
    else if type_id == TypeId::of::<i16>() { 5 }
    else if type_id == TypeId::of::<u16>() { 6 }
    else if type_id == TypeId::of::<i32>() { 7 }
    else if type_id == TypeId::of::<u32>() { 8 }
    else if type_id == TypeId::of::<i64>() { 9 }
    else if type_id == TypeId::of::<u64>() { 10 }
    else if type_id == TypeId::of::<i128>() { 11 }
    else if type_id == TypeId::of::<u128>() { 12 }
    else { 99 }
}

fn expect_type(got: u32, want: u32) -> bool {
    got == want
}

enum E {
    Neg = -1,
    Pos = 1,
}

fn main() {
    if !expect_type(type_id((true as bool) as i32 + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(('a' as i8) as i32 + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id((1i8) as i32 + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id((1u8) as i32 + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id((1i16) as i32 + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id((1u16) as i32 + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id(0i32 + 0u32), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id(0i64 + 0u64), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id(0i128 + 0u128), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id(0i32 + 0i64), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id(0i64 + 0i128), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id(0i32 + 0i128), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id((E::Pos as i32) + 0u32), type_id(0i32 + 0u32)) {
        std::process::exit(13);
    }
    if !expect_type(type_id((E::Pos as i32) + 0), type_id(0i32 + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}