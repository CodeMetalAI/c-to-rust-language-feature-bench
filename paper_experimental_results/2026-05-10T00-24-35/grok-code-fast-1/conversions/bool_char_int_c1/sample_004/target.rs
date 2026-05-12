use std::any::TypeId;

enum E {
    Neg = -1,
    Pos = 1,
}

fn get_type_id<T: 'static>(_val: T) -> TypeId {
    TypeId::of::<T>()
}

fn expect_type(got: TypeId, want: TypeId) -> bool {
    got == want
}

fn main() {
    if !expect_type(get_type_id((true as i32) + 0i32), TypeId::of::<i32>()) {
        std::process::exit(1);
    }
    if !expect_type(get_type_id((1i8) + 0i32), TypeId::of::<i32>()) {
        std::process::exit(2);
    }
    if !expect_type(get_type_id((1i8) + 0i32), TypeId::of::<i32>()) {
        std::process::exit(3);
    }
    if !expect_type(get_type_id((1u8) + 0i32), TypeId::of::<i32>()) {
        std::process::exit(4);
    }
    if !expect_type(get_type_id((1i16) + 0i32), TypeId::of::<i32>()) {
        std::process::exit(5);
    }
    if !expect_type(get_type_id((1u16) + 0i32), TypeId::of::<i32>()) {
        std::process::exit(6);
    }

    if !expect_type(get_type_id((0i32 as u32) + 0u32), TypeId::of::<u32>()) {
        std::process::exit(7);
    }
    if !expect_type(get_type_id((0i32 as u32) + 0u32), TypeId::of::<u32>()) {
        std::process::exit(8);
    }
    if !expect_type(get_type_id((0i64 as u64) + 0u64), TypeId::of::<u64>()) {
        std::process::exit(9);
    }

    if !expect_type(get_type_id((0i32 as i64) + 0i64), TypeId::of::<i64>()) {
        std::process::exit(10);
    }
    if !expect_type(get_type_id((0i64) + 0i64), TypeId::of::<i64>()) {
        std::process::exit(11);
    }
    if !expect_type(get_type_id((0i32 as i64) + 0i64), TypeId::of::<i64>()) {
        std::process::exit(12);
    }

    if !expect_type(get_type_id((E::Neg as i32 as u32) + 0u32), get_type_id((0i32 as u32) + 0u32)) {
        std::process::exit(13);
    }
    if !expect_type(get_type_id((E::Neg as i32) + 0i32), get_type_id(0i32 + 0i32)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}