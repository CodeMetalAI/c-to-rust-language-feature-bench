#[derive(Debug, Copy, Clone)]
struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn type_id<T>(x: T) -> i32 {
    match std::any::type_name::<T>() {
        "signed char" => {
            if x as i32 == 1 {
                1
            } else {
                99
            }
        }
        "unsigned char" => {
            if x as i32 == 1 {
                1
            } else {
                99
            }
        }
        "short" => {
            if x as i32 == 1 {
                1
            } else {
                99
            }
        }
        "unsigned short" => {
            if x as i32 == 1 {
                1
            } else {
                99
            }
        }
        "i32" => 1,
        "u32" => 2,
        "i64" => 3,
        "u64" => 4,
        "i128" => 5,
        "u128" => 6,
        "f32" => 7,
        "f64" => 8,
        "f128" => 9,
        _ => 99,
    }
}

fn main() {
    if type_id((1 as signed char) + 0) != 1 {
        return 1;
    }
    if type_id((1 as unsigned char) + 0) != 1 {
        return 2;
    }
    if type_id((1 as short) + 0) != 1 {
        return 3;
    }
    if type_id((1 as unsigned short) + 0) != 1 {
        return 4;
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 + 0) != 1 {
        return 5;
    }
    if type_id(bf.i1 + 0) != 1 {
        return 6;
    }
    if type_id(bf.b1 + 0) != 1 {
        return 7;
    }

    if type_id(1.0f32) != 7 {
        return 8;
    }
    if type_id(1.0) != 8 {
        return 9;
    }
    if type_id(1.0) != 9 {
        return 10;
    }

    0
}