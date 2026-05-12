fn type_id<T>(x: T) -> i32 {
    match x {
        _ if std::is_integral::<T>() => 1,
        _ if std::is_unsigned::<T>() => 2,
        _ if std::is_signed::<T>() => 3,
        _ if std::is_floating_point::<T>() => 7,
        _ => 99,
    }
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if type_id((1 as i8 + 0) as i32) != 1 {
        return 1;
    }
    if type_id((1 as u8 + 0) as i32) != 1 {
        return 2;
    }
    if type_id((1 as i16 + 0) as i32) != 1 {
        return 3;
    }
    if type_id((1 as u16 + 0) as i32) != 1 {
        return 4;
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 as i32) != 1 {
        return 5;
    }
    if type_id(bf.i1 as i32) != 1 {
        return 6;
    }
    if type_id(bf.b1 as i32) != 1 {
        return 7;
    }

    if type_id(1.0f32 as f32) != 7 {
        return 8;
    }
    if type_id(1.0f64 as f64) != 8 {
        return 9;
    }
    if type_id(1.0f80 as f80) != 9 {
        return 10;
    }

    0
}