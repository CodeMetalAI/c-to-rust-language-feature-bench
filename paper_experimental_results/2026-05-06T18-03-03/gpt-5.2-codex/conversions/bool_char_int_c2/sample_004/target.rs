use std::process;

trait TypeId {
    const ID: i32;
}

impl TypeId for i32 {
    const ID: i32 = 1;
}
impl TypeId for u32 {
    const ID: i32 = 2;
}
impl TypeId for i64 {
    const ID: i32 = 3;
}
impl TypeId for u64 {
    const ID: i32 = 4;
}
impl TypeId for i128 {
    const ID: i32 = 5;
}
impl TypeId for u128 {
    const ID: i32 = 6;
}
impl TypeId for f32 {
    const ID: i32 = 7;
}
impl TypeId for f64 {
    const ID: i32 = 8;
}

struct LongDouble(f64);
impl TypeId for LongDouble {
    const ID: i32 = 9;
}

fn type_id<T: TypeId>(_: T) -> i32 {
    T::ID
}

struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    if type_id((1i8 as i32) + 0) != 1 {
        process::exit(1);
    }
    if type_id((1u8 as i32) + 0) != 1 {
        process::exit(2);
    }
    if type_id((1i16 as i32) + 0) != 1 {
        process::exit(3);
    }
    if type_id((1u16 as i32) + 0) != 1 {
        process::exit(4);
    }

    let bf = BF {
        u1: 1u32,
        i1: -1i32,
        b1: true,
    };

    if type_id((bf.u1 as i32) + 0) != 1 {
        process::exit(5);
    }
    if type_id((bf.i1 as i32) + 0) != 1 {
        process::exit(6);
    }
    if type_id((bf.b1 as i32) + 0) != 1 {
        process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        process::exit(9);
    }
    if type_id(LongDouble(1.0f64)) != 9 {
        process::exit(10);
    }

    process::exit(0);
}