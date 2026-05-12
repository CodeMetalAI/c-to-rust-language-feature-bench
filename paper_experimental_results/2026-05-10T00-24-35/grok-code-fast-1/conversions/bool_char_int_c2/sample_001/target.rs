use std::any::TypeId;

struct LongDouble(f64);

fn type_id<T: 'static>(_val: &T) -> i32 {
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
    } else if tid == TypeId::of::<LongDouble>() {
        9
    } else {
        99
    }
}

struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    if type_id(&((1i8 as i32) + 0)) != 1 {
        std::process::exit(1);
    }
    if type_id(&((1u8 as i32) + 0)) != 1 {
        std::process::exit(2);
    }
    if type_id(&((1i16 as i32) + 0)) != 1 {
        std::process::exit(3);
    }
    if type_id(&((1u16 as i32) + 0)) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF {
        u1: 0,
        i1: 0,
        b1: false,
    };
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    if type_id(&((bf.u1 as i32) + 0)) != 1 {
        std::process::exit(5);
    }
    if type_id(&((bf.i1 as i32) + 0)) != 1 {
        std::process::exit(6);
    }
    if type_id(&((bf.b1 as i32) + 0)) != 1 {
        std::process::exit(7);
    }

    if type_id(&(1.0f32)) != 7 {
        std::process::exit(8);
    }
    if type_id(&(1.0f64)) != 8 {
        std::process::exit(9);
    }
    let ld = LongDouble(1.0);
    if type_id(&ld) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}