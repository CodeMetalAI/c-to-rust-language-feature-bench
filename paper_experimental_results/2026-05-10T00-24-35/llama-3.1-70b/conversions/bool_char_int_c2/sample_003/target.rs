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
    if type_id((1_i8 + 0) as i8) != 1 {
        return;
    }
    if type_id((1_u8 + 0) as u8) != 1 {
        return;
    }
    if type_id((1_i16 + 0) as i16) != 1 {
        return;
    }
    if type_id((1_u16 + 0) as u16) != 1 {
        return;
    }

    let mut bf = BF { u1: 1, i1: -1, b1: true };

    if type_id(bf.u1 as i32) != 1 {
        return;
    }
    if type_id(bf.i1 as i32) != 1 {
        return;
    }
    if type_id(bf.b1 as u8) != 1 {
        return;
    }

    if type_id(1.0f32) != 7 {
        return;
    }
    if type_id(1.0f64) != 8 {
        return;
    }
    if type_id(1.0f128) != 9 {
        return;
    }

    println!();
}