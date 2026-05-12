#[derive(Debug)]
struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn type_id<T: std::fmt::Debug>(x: T) -> i32 {
    match x {
        x if std::any::TypeId::of::<i32>() == std::any::TypeId::of_val(&x) => 1,
        x if std::any::TypeId::of::<u32>() == std::any::TypeId::of_val(&x) => 2,
        x if std::any::TypeId::of::<i64>() == std::any::TypeId::of_val(&x) => 3,
        x if std::any::TypeId::of::<u64>() == std::any::TypeId::of_val(&x) => 4,
        x if std::any::TypeId::of::<i128>() == std::any::TypeId::of_val(&x) => 5,
        x if std::any::TypeId::of::<u128>() == std::any::TypeId::of_val(&x) => 6,
        x if std::any::TypeId::of::<f32>() == std::any::TypeId::of_val(&x) => 7,
        x if std::any::TypeId::of::<f64>() == std::any::TypeId::of_val(&x) => 8,
        x if std::any::TypeId::of::<i8>() == std::any::TypeId::of_val(&x) => 1,
        x if std::any::TypeId::of::<u8>() == std::any::TypeId::of_val(&x) => 1,
        x if std::any::TypeId::of::<i16>() == std::any::TypeId::of_val(&x) => 1,
        x if std::any::TypeId::of::<u16>() == std::any::TypeId::of_val(&x) => 1,
        x if std::any::TypeId::of::<bool>() == std::any::TypeId::of_val(&x) => 1,
        _ => 99,
    }
}

fn main() {
    if type_id((1i8 + 0) as i32) != 1 {
        std::process::exit(1);
    }
    if type_id((1u8 + 0) as i32) != 1 {
        std::process::exit(2);
    }
    if type_id((1i16 + 0) as i32) != 1 {
        std::process::exit(3);
    }
    if type_id((1u16 + 0) as i32) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF {
        u1: 1u8,
        i1: -1i8,
        b1: true,
    };

    if type_id((bf.u1 + 0) as i32) != 1 {
        std::process::exit(5);
    }
    if type_id((bf.i1 + 0) as i32) != 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }

    std::process::exit(0);
}