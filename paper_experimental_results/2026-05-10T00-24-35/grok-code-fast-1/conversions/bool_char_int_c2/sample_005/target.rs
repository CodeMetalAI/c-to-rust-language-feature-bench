use std::any::Any;

type LongDouble = f64;

fn get_type_id(value: &dyn Any) -> i32 {
    let tid = value.type_id();
    if tid == std::any::TypeId::of::<i32>() {
        1
    } else if tid == std::any::TypeId::of::<u32>() {
        2
    } else if tid == std::any::TypeId::of::<i64>() {
        3
    } else if tid == std::any::TypeId::of::<u64>() {
        4
    } else if tid == std::any::TypeId::of::<i64>() {
        5
    } else if tid == std::any::TypeId::of::<u64>() {
        6
    } else if tid == std::any::TypeId::of::<f32>() {
        7
    } else if tid == std::any::TypeId::of::<f64>() {
        8
    } else if tid == std::any::TypeId::of::<LongDouble>() {
        9
    } else {
        99
    }
}

macro_rules! type_id {
    ($e:expr) => {{
        let v = $e;
        get_type_id(&v as &dyn Any)
    }};
}

struct BF {
    u1: i32,
    i1: i32,
    b1: i32,
}

fn main() {
    if type_id!((1i8 + 0)) != 1 {
        std::process::exit(1);
    }
    if type_id!((1u8 + 0)) != 1 {
        std::process::exit(2);
    }
    if type_id!((1i16 + 0)) != 1 {
        std::process::exit(3);
    }
    if type_id!((1u16 + 0)) != 1 {
        std::process::exit(4);
    }

    let bf = BF {
        u1: 1,
        i1: -1,
        b1: 1,
    };

    if type_id!(bf.u1 + 0) != 1 {
        std::process::exit(5);
    }
    if type_id!(bf.i1 + 0) != 1 {
        std::process::exit(6);
    }
    if type_id!(bf.b1 + 0) != 1 {
        std::process::exit(7);
    }

    if type_id!((1.0f32)) != 7 {
        std::process::exit(8);
    }
    if type_id!((1.0f64)) != 8 {
        std::process::exit(9);
    }
    if type_id!((1.0f64 as LongDouble)) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}