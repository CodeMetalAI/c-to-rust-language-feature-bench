fn type_id<T>(_: T) -> u32 {
    std::any::TypeId::of::<T>().hash() as u32
}

struct BF {
    u1: u8, // Using u8 to represent a 1-bit unsigned int
    i1: i8, // Using i8 to represent a 1-bit signed int
    b1: bool,
}

fn main() -> i32 {
    if type_id(1i8 + 0) != 1 {
        return 1;
    }
    if type_id(1u8 + 0) != 1 {
        return 2;
    }
    if type_id(1i16 + 0) != 1 {
        return 3;
    }
    if type_id(1u16 + 0) != 1 {
        return 4;
    }

    let bf = BF {
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
    if type_id(bf.b1 as u8 + 0) != 1 {
        return 7;
    }

    if type_id(1.0f32) != 7 {
        return 8;
    }
    if type_id(1.0f64) != 8 {
        return 9;
    }
    if type_id(1.0f64) != 9 { // Rust does not have long double, using f64
        return 10;
    }

    return 0;
}