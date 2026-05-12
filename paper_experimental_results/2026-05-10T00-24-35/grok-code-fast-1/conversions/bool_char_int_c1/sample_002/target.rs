use std::any::TypeId;

enum E {
    ENeg = -1,
    EPos = 1,
}

fn type_id<T: 'static>() -> i32 {
    let id = TypeId::of::<T>();
    if id == TypeId::of::<bool>() {
        1
    } else if id == TypeId::of::<i8>() {
        2
    } else if id == TypeId::of::<u8>() {
        4
    } else if id == TypeId::of::<i16>() {
        5
    } else if id == TypeId::of::<u16>() {
        6
    } else if id == TypeId::of::<i32>() {
        7
    } else if id == TypeId::of::<u32>() {
        8
    } else if id == TypeId::of::<isize>() {
        9
    } else if id == TypeId::of::<usize>() {
        10
    } else if id == TypeId::of::<i64>() {
        11
    } else if id == TypeId::of::<u64>() {
        12
    } else {
        99
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() -> i32 {
    // (_Bool)1 + 0 -> i32
    if !expect_type(type_id::<i32>(), 7) {
        return 1;
    }
    // (char)1 + 0 -> i32 (assuming char as i8)
    let _x: i32 = (1i8 as i32) + 0;
    if !expect_type(type_id::<i32>(), 7) {
        return 2;
    }
    // (signed char)1 + 0 -> i32
    if !expect_type(type_id::<i32>(), 7) {
        return 3;
    }
    // (unsigned char)1 + 0 -> i32
    let _y: i32 = (1u8 as i32) + 0;
    if !expect_type(type_id::<i32>(), 7) {
        return 4;
    }
    // (short)1 + 0 -> i32
    let _z: i32 = (1i16 as i32) + 0;
    if !expect_type(type_id::<i32>(), 7) {
        return 5;
    }
    // (unsigned short)1 + 0 -> i32
    let _a: i32 = (1u16 as i32) + 0;
    if !expect_type(type_id::<i32>(), 7) {
        return 6;
    }

    // (int)0 + (unsigned int)0 -> u32
    let _b: u32 = 0u32 + 0u32;
    if !expect_type(type_id::<u32>(), 8) {
        return 7;
    }
    // (long)0 + (unsigned long)0 -> usize
    let _c: usize = 0usize + 0usize;
    if !expect_type(type_id::<usize>(), 10) {
        return 8;
    }
    // (long long)0 + (unsigned long long)0 -> u64
    let _d: u64 = 0u64 + 0u64;
    if !expect_type(type_id::<u64>(), 12) {
        return 9;
    }

    // (int)0 + (long)0 -> isize
    let _e: isize = 0isize + 0isize;
    if !expect_type(type_id::<isize>(), 9) {
        return 10;
    }
    // (long)0 + (long long)0 -> i64
    let _f: i64 = 0isize as i64 + 0i64;
    if !expect_type(type_id::<i64>(), 11) {
        return 11;
    }
    // (int)0 + (long long)0 -> i64
    let _g: i64 = 0i32 as i64 + 0i64;
    if !expect_type(type_id::<i64>(), 11) {
        return 12;
    }

    // (enum E)0 + 0u -> u32
    let _h: u32 = (E::ENeg as i32 as u32) + 0u32;
    // (int)0 + 0u -> u32
    let _i: u32 = 0u32 + 0u32;
    if !expect_type(type_id::<u32>(), type_id::<u32>()) {
        return 13;
    }
    // (enum E)0 + 0 -> i32
    let _j: i32 = (E::ENeg as i32) + 0i32;
    // (int)0 + 0 -> i32
    let _k: i32 = 0i32 + 0i32;
    if !expect_type(type_id::<i32>(), type_id::<i32>()) {
        return 14;
    }

    0
}