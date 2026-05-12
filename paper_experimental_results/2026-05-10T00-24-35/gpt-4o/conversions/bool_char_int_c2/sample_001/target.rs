fn type_id<T>(_: T) -> u8 {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>()
}

struct BF {
    u1: u8, // Bitfields are not directly supported in Rust, but this will suffice for this example
    i1: i8,
    b1: bool,
}

fn main() -> i32 {
    if !type_id(1i8 + 0) {
        return 1;
    }
    if !type_id(1u8 + 0) {
        return 2;
    }
    if !type_id(1i16 + 0) {
        return 3;
    }
    if !type_id(1u16 + 0) {
        return 4;
    }

    let mut bf = BF { u1: 0, i1: 0, b1: false };
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    if !type_id(bf.u1 + 0) {
        return 5;
    }
    if !type_id(bf.i1 + 0) {
        return 6;
    }
    if !type_id(bf.b1 as i32 + 0) {
        return 7;
    }

    if type_id(1.0f32) != 7 {
        return 8;
    }
    if type_id(1.0f64) != 8 {
        return 9;
    }
    // Rust does not have a separate long double type
    if type_id(1.0f64) != 9 {
        return 10;
    }

    0
}