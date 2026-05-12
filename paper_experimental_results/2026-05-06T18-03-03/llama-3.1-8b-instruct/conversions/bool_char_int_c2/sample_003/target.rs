fn type_id<T>(_: T) -> i32 {
    match std::mem::size_of::<T>() {
        1 => 1,
        2 => 2,
        4 => 3,
        8 => 4,
        16 => 5,
        4..=8 => 6,
        8..=16 => 7,
        16..=32 => 8,
        32..=64 => 9,
        _ => 99,
    }
}

struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    if type_id((1 as i8) + 0) != 1 {
        return 1;
    }
    if type_id((1 as u8) + 0) != 1 {
        return 2;
    }
    if type_id((1 as i16) + 0) != 1 {
        return 3;
    }
    if type_id((1 as u16) + 0) != 1 {
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
    if type_id(1.0f64) != 8 {
        return 9;
    }
    if type_id(1.0f80) != 9 {
        return 10;
    }

    println!("All tests passed");
}