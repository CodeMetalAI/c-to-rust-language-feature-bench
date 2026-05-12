use std::any::TypeId;

#[derive(Clone, Copy)]
struct Float(f32);

#[derive(Clone, Copy)]
struct Double(f64);

#[derive(Clone, Copy)]
struct LongDouble(f64);

fn type_id<T: 'static>() -> i32 {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        1
    } else if tid == TypeId::of::<Float>() {
        7
    } else if tid == TypeId::of::<Double>() {
        8
    } else if tid == TypeId::of::<LongDouble>() {
        9
    } else {
        99
    }
}

struct BF {
    u1: i32,
    i1: i32,
    b1: i32,
}

fn main() {
    if type_id::<i32>() != 1 {
        std::process::exit(1);
    }
    if type_id::<i32>() != 1 {
        std::process::exit(2);
    }
    if type_id::<i32>() != 1 {
        std::process::exit(3);
    }
    if type_id::<i32>() != 1 {
        std::process::exit(4);
    }

    let mut bf = BF {
        u1: 0,
        i1: 0,
        b1: 0,
    };
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = 1;

    if type_id::<i32>() != 1 {
        std::process::exit(5);
    }
    if type_id::<i32>() != 1 {
        std::process::exit(6);
    }
    if type_id::<i32>() != 1 {
        std::process::exit(7);
    }

    if type_id::<Float>() != 7 {
        std::process::exit(8);
    }
    if type_id::<Double>() != 8 {
        std::process::exit(9);
    }
    if type_id::<LongDouble>() != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}