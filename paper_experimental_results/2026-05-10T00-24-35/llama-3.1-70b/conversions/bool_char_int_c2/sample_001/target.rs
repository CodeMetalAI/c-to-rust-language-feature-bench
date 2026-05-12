fn type_id<T>(_: T) -> u8 {
    std::any::TypeId::of::<T>().as_u64() as u8
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if type_id((i8::from(1) + 0))!= type_id(1)
        || type_id((u8::from(1) + 0))!= type_id(1)
        || type_id((i16::from(1) + 0))!= type_id(1)
        || type_id((u16::from(1) + 0))!= type_id(1)
    {
        return 1;
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 as u32)!= type_id(1)
        || type_id(bf.i1 as i32)!= type_id(1)
        || type_id(bf.b1 as u8)!= type_id(1)
    {
        return 2;
    }

    if type_id(1.0f32)!= type_id(1.0f32)
        || type_id(1.0f64)!= type_id(1.0f64)
        || type_id(1.0f64)!= type_id(1.0f64)
    {
        return 3;
    }

    std::process::exit(0);
}