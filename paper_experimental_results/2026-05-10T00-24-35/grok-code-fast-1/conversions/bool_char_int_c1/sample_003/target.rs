use std::any::{Any, type_name_of_val};

#[derive(Clone, Copy)]
enum E {
    Neg = -1,
    Pos = 1,
}

fn type_id(val: &dyn Any) -> i32 {
    match type_name_of_val(val) {
        "i32" => 7,
        "u32" => 8,
        "i64" => 9,
        "u64" => 10,
        "i128" => 11,
        "u128" => 12,
        _ => 99,
    }
}

fn main() {
    // (_Bool)1 + 0
    let val = Box::new((true as i32) + 0) as Box<dyn Any>;
    if type_id(&*val) != 7 {
        std::process::exit(1);
    }

    // (char)1 + 0
    let val = Box::new(('\x01' as i8) + 0) as Box<dyn Any>;
    if type_id(&*val) != 7 {
        std::process::exit(2);
    }

    // (signed char)1 + 0
    let val = Box::new((1i8) + 0) as Box<dyn Any>;
    if type_id(&*val) != 7 {
        std::process::exit(3);
    }

    // (unsigned char)1 + 0
    let val = Box::new((1u8) + 0) as Box<dyn Any>;
    if type_id(&*val) != 7 {
        std::process::exit(4);
    }

    // (short)1 + 0
    let val = Box::new((1i16) + 0) as Box<dyn Any>;
    if type_id(&*val) != 7 {
        std::process::exit(5);
    }

    // (unsigned short)1 + 0
    let val = Box::new((1u16) + 0) as Box<dyn Any>;
    if type_id(&*val) != 7 {
        std::process::exit(6);
    }

    // (int)0 + (unsigned int)0
    let val = Box::new((0i32 as u32) + 0u32) as Box<dyn Any>;
    if type_id(&*val) != 8 {
        std::process::exit(7);
    }

    // (long)0 + (unsigned long)0
    let val = Box::new((0i64 as u64) + 0u64) as Box<dyn Any>;
    if type_id(&*val) != 10 {
        std::process::exit(8);
    }

    // (long long)0 + (unsigned long long)0
    let val = Box::new((0i128 as u128) + 0u128) as Box<dyn Any>;
    if type_id(&*val) != 12 {
        std::process::exit(9);
    }

    // (int)0 + (long)0
    let val = Box::new(0i32 + 0i64) as Box<dyn Any>;
    if type_id(&*val) != 9 {
        std::process::exit(10);
    }

    // (long)0 + (long long)0
    let val = Box::new(0i64 + 0i128) as Box<dyn Any>;
    if type_id(&*val) != 11 {
        std::process::exit(11);
    }

    // (int)0 + (long long)0
    let val = Box::new(0i32 + 0i128) as Box<dyn Any>;
    if type_id(&*val) != 11 {
        std::process::exit(12);
    }

    // (enum E)0 + 0u
    let val1 = Box::new((E::Pos as i32 as u32) + 0u32) as Box<dyn Any>;
    let val2 = Box::new((0i32 as u32) + 0u32) as Box<dyn Any>;
    if type_id(&*val1) != type_id(&*val2) {
        std::process::exit(13);
    }

    // (enum E)0 + 0
    let val1 = Box::new((E::Pos as i32) + 0) as Box<dyn Any>;
    let val2 = Box::new(0i32 + 0) as Box<dyn Any>;
    if type_id(&*val1) != type_id(&*val2) {
        std::process::exit(14);
    }
}