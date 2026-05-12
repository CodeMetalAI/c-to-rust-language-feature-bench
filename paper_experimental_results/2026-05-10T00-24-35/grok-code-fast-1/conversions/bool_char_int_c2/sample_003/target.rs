use std::any::TypeId;

fn get_type_id<T: 'static>(_: &T) -> i32 {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        1
    } else if tid == TypeId::of::<u32>() {
        2
    } else if tid == TypeId::of::<i64>() {
        3
    } else if tid == TypeId::of::<u64>() {
        4
    } else if tid == TypeId::of::<i64>() {
        5
    } else if tid == TypeId::of::<u64>() {
        6
    } else if tid == TypeId::of::<f32>() {
        7
    } else if tid == TypeId::of::<f64>() {
        8
    } else if tid == TypeId::of::<f64>() {
        9
    } else {
        99
    }
}

struct Bf {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    let temp = (1i8 as i32) + 0;
    if get_type_id(&temp) != 1 {
        std::process::exit(1);
    }
    let temp = (1u8 as i32) + 0;
    if get_type_id(&temp) != 1 {
        std::process::exit(2);
    }
    let temp = (1i16 as i32) + 0;
    if get_type_id(&temp) != 1 {
        std::process::exit(3);
    }
    let temp = (1u16 as i32) + 0;
    if get_type_id(&temp) != 1 {
        std::process::exit(4);
    }

    let mut bf = Bf {
        u1: 1,
        i1: -1,
        b1: true,
    };

    let temp = (bf.u1 as i32) + 0;
    if get_type_id(&temp) != 1 {
        std::process::exit(5);
    }
    let temp = bf.i1 + 0;
    if get_type_id(&temp) != 1 {
        std::process::exit(6);
    }
    let temp = (bf.b1 as i32) + 0;
    if get_type_id(&temp) != 1 {
        std::process::exit(7);
    }

    let temp = 1.0f32;
    if get_type_id(&temp) != 7 {
        std::process::exit(8);
    }
    let temp = 1.0f64;
    if get_type_id(&temp) != 8 {
        std::process::exit(9);
    }
    let temp = 1.0f64;
    if get_type_id(&temp) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}