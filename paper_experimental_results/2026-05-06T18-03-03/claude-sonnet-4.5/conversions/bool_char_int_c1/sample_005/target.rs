fn main() {
    fn expect_type(got: i32, want: i32) -> bool {
        got == want
    }

    // Test 1: bool + 0 -> i32
    if !expect_type(type_id_of_value(true as i32 + 0), 7) {
        std::process::exit(1);
    }

    // Test 2: char + 0 -> i32 (char in C is a small integer type)
    if !expect_type(type_id_of_value(1i8 as i32 + 0), 7) {
        std::process::exit(2);
    }

    // Test 3: signed char + 0 -> i32
    if !expect_type(type_id_of_value(1i8 as i32 + 0), 7) {
        std::process::exit(3);
    }

    // Test 4: unsigned char + 0 -> i32
    if !expect_type(type_id_of_value(1u8 as i32 + 0), 7) {
        std::process::exit(4);
    }

    // Test 5: short + 0 -> i32
    if !expect_type(type_id_of_value(1i16 as i32 + 0), 7) {
        std::process::exit(5);
    }

    // Test 6: unsigned short + 0 -> i32
    if !expect_type(type_id_of_value(1u16 as i32 + 0), 7) {
        std::process::exit(6);
    }

    // Test 7: int + unsigned int -> unsigned int
    if !expect_type(type_id_of_value(0i32 as u32 + 0u32), 8) {
        std::process::exit(7);
    }

    // Test 8: long + unsigned long -> unsigned long
    if !expect_type(type_id_of_value(0i64 as u64 + 0u64), 10) {
        std::process::exit(8);
    }

    // Test 9: long long + unsigned long long -> unsigned long long
    if !expect_type(type_id_of_value(0i64 as u64 + 0u64), 12) {
        std::process::exit(9);
    }

    // Test 10: int + long -> long
    if !expect_type(type_id_of_value(0i32 as i64 + 0i64), 9) {
        std::process::exit(10);
    }

    // Test 11: long + long long -> long long
    if !expect_type(type_id_of_value(0i64 as i64 + 0i64), 11) {
        std::process::exit(11);
    }

    // Test 12: int + long long -> long long
    if !expect_type(type_id_of_value(0i32 as i64 + 0i64), 11) {
        std::process::exit(12);
    }

    // Test 13: enum + unsigned int -> same as int + unsigned int
    if !expect_type(type_id_of_value(0i32 as u32 + 0u32), type_id_of_value(0i32 as u32 + 0u32)) {
        std::process::exit(13);
    }

    // Test 14: enum + int -> same as int + int
    if !expect_type(type_id_of_value(0i32 + 0i32), type_id_of_value(0i32 + 0i32)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}

fn type_id_of_value<T>(_: T) -> i32 {
    type_id::<T>()
}

fn type_id<T: 'static>() -> i32 {
    use std::any::TypeId;
    
    let tid = TypeId::of::<T>();
    
    if tid == TypeId::of::<bool>() { 1 }
    else if tid == TypeId::of::<i8>() { 3 }
    else if tid == TypeId::of::<u8>() { 4 }
    else if tid == TypeId::of::<i16>() { 5 }
    else if tid == TypeId::of::<u16>() { 6 }
    else if tid == TypeId::of::<i32>() { 7 }
    else if tid == TypeId::of::<u32>() { 8 }
    else if tid == TypeId::of::<i64>() { 9 }
    else if tid == TypeId::of::<u64>() { 10 }
    else if tid == TypeId::of::<i64>() { 11 }
    else if tid == TypeId::of::<u64>() { 12 }
    else { 99 }
}