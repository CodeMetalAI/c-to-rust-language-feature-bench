fn main() {
    #[derive(Debug)]
    struct BF {
        u1: u32,
        i1: i32,
        b1: bool,
    }

    let mut bf = BF { u1: 0, i1: 0, b1: false };
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    if type_id(1i8 as i32) != 1 {
        std::process::exit(1);
    }
    if type_id(1u8 as u32) != 1 {
        std::process::exit(2);
    }
    if type_id(1i16 as i32) != 1 {
        std::process::exit(3);
    }
    if type_id(1u16 as u32) != 1 {
        std::process::exit(4);
    }

    if type_id(bf.u1) != 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1) != 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 as u8) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }
    if type_id(1.0f64) != 8 { // Rust does not have `long double`, using `f64` as closest match.
        std::process::exit(10);
    }
}

fn type_id<T: 'static>(_: T) -> i32 {
    match std::any::TypeId::of::<T>() {
        _ if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() => 1,
        _ if std::any::TypeId::of::<u32>() == std::any::TypeId::of::<T>() => 1,
        _ if std::any::TypeId::of::<i64>() == std::any::TypeId::of::<T>() => 3,
        _ if std::any::TypeId::of::<u64>() == std::any::TypeId::of::<T>() => 4,
        _ if std::any::TypeId::of::<f32>() == std::any::TypeId::of::<T>() => 7,
        _ if std::any::TypeId::of::<f64>() == std::any::TypeId::of::<T>() => 8,
        _ => 99,
    }
}