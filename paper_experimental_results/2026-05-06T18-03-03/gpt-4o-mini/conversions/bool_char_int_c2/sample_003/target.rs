fn type_id<T>(_: T) -> u32 {
    std::any::TypeId::of::<T>().hash() as u32
}

struct BF {
    u1: u8, // Using u8 to represent a 1-bit unsigned integer
    i1: i8, // Using i8 to represent a 1-bit signed integer
    b1: bool,
}

fn main() -> i32 {
    if type_id(1i8) != type_id(1i8 + 0) {
        return 1;
    }
    if type_id(1u8) != type_id(1u8 + 0) {
        return 2;
    }
    if type_id(1i16) != type_id(1i16 + 0) {
        return 3;
    }
    if type_id(1u16) != type_id(1u16 + 0) {
        return 4;
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1) != type_id(1u8 + 0) {
        return 5;
    }
    if type_id(bf.i1) != type_id(-1i8 + 0) {
        return 6;
    }
    if type_id(bf.b1) != type_id(true) {
        return 7;
    }

    if type_id(1.0f32) != type_id(1.0f32) {
        return 8;
    }
    if type_id(1.0f64) != type_id(1.0f64) {
        return 9;
    }
    if type_id(1.0f64) != type_id(1.0f64) {
        return 10;
    }

    return 0;
}