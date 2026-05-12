enum E {
    NEG = -1,
    POS = 1,
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id(&_Bool::from(true) + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(&(1 as char) + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id(&(1 as i8) + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id(&(1 as u8) + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id(&(1 as i16) + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id(&(1 as u16) + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id(&(0 as i32) + 0 as u32), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id(&(0 as i64) + 0 as u64), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id(&(0 as i64) + 0 as u64), 10) {
        std::process::exit(9);
    }

    if !expect_type(type_id(&(0 as i32) + 0 as i64), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id(&(0 as i64) + 0 as i64), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id(&(0 as i32) + 0 as i64), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id(&(E::NEG as i32) + 0 as u32), type_id(&(0 as i32) + 0 as u32)) {
        std::process::exit(13);
    }
    if !expect_type(type_id(&(E::NEG as i32) + 0), type_id(&(0 as i32) + 0)) {
        std::process::exit(14);
    }
}

fn type_id<T: std::any::Any + Copy>(_: &T) -> i32 {
    match std::any::Any::type_id(&_) {
        _ if std::any::type_name::<bool>() == std::any::type_name::<T>() => 1,
        _ if std::any::type_name::<char>() == std::any::type_name::<T>() => 2,
        _ if std::any::type_name::<i8>() == std::any::type_name::<T>() => 3,
        _ if std::any::type_name::<u8>() == std::any::type_name::<T>() => 4,
        _ if std::any::type_name::<i16>() == std::any::type_name::<T>() => 5,
        _ if std::any::type_name::<u16>() == std::any::type_name::<T>() => 6,
        _ if std::any::type_name::<i32>() == std::any::type_name::<T>() => 7,
        _ if std::any::type_name::<u32>() == std::any::type_name::<T>() => 8,
        _ if std::any::type_name::<i64>() == std::any::type_name::<T>() => 9,
        _ if std::any::type_name::<u64>() == std::any::type_name::<T>() => 10,
        _ if std::any::type_name::<i64>() == std::any::type_name::<T>() => 11,
        _ if std::any::type_name::<u64>() == std::any::type_name::<T>() => 12,
        _ => 99,
    }
}