fn type_id<T>(_: T) -> i32 {
    match std::any::type_id_of::<T>() {
        std::any::TypeId::Int => 1,
        std::any::TypeId::Uint => 2,
        std::any::TypeId::Long => 3,
        std::any::TypeId::Ulong => 4,
        std::any::TypeId::Longlong => 5,
        std::any::TypeId::Ulonglong => 6,
        std::any::TypeId::Float => 7,
        std::any::TypeId::Double => 8,
        std::any::TypeId::Longdouble => 9,
        _ => 99,
    }
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if type_id((1i8 + 0) as u8) != 1 {
        return 1;
    }
    if type_id((1u8 + 0) as u8) != 1 {
        return 2;
    }
    if type_id((1i16 + 0) as u8) != 1 {
        return 3;
    }
    if type_id((1u16 + 0) as u8) != 1 {
        return 4;
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 as u8) != 1 {
        return 5;
    }
    if type_id(bf.i1 as u8) != 1 {
        return 6;
    }
    if type_id(bf.b1 as u8) != 1 {
        return 7;
    }

    if type_id(1.0f32 as f32) != 7 {
        return 8;
    }
    if type_id(1.0f64 as f64) != 8 {
        return 9;
    }
    if type_id(1.0e3f80 as f80) != 9 {
        return 10;
    }

    println!("Success");
}