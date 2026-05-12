fn type_id<T>(_: T) -> u8 {
    std::any::TypeId::of::<T>().as_u64() as u8
}

#[repr(C, packed)]
struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if type_id((i8::from(1) + 0) as i8)!= type_id(1i8) {
        return 1;
    }
    if type_id((u8::from(1) + 0) as u8)!= type_id(1u8) {
        return 2;
    }
    if type_id((i16::from(1) + 0) as i16)!= type_id(1i16) {
        return 3;
    }
    if type_id((u16::from(1) + 0) as u16)!= type_id(1u16) {
        return 4;
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 as u8)!= type_id(1u8) {
        return 5;
    }
    if type_id(bf.i1 as i8)!= type_id(1i8) {
        return 6;
    }
    if type_id(bf.b1 as bool)!= type_id(true) {
        return 7;
    }

    if type_id(1.0f32)!= type_id(1.0f32) {
        return 8;
    }
    if type_id(1.0f64)!= type_id(1.0f64) {
        return 9;
    }
    if type_id(1.0f64)!= type_id(1.0f64) {
        return 10;
    }

    return 0;
}