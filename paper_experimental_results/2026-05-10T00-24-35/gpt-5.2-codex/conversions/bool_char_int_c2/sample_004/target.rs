use std::process::exit;

trait TypeIdConst {
    const ID: i32;
}

impl TypeIdConst for i32 {
    const ID: i32 = 1;
}
impl TypeIdConst for u32 {
    const ID: i32 = 2;
}
impl TypeIdConst for i64 {
    const ID: i32 = 3;
}
impl TypeIdConst for u64 {
    const ID: i32 = 4;
}
impl TypeIdConst for f32 {
    const ID: i32 = 7;
}
impl TypeIdConst for f64 {
    const ID: i32 = 8;
}

#[derive(Copy, Clone)]
struct LongDouble(f64);

impl TypeIdConst for LongDouble {
    const ID: i32 = 9;
}

fn type_id<T: TypeIdConst>(_: T) -> i32 {
    T::ID
}

struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    if type_id((1i8 as i32) + 0) != 1 {
        exit(1);
    }
    if type_id((1u8 as i32) + 0) != 1 {
        exit(2);
    }
    if type_id((1i16 as i32) + 0) != 1 {
        exit(3);
    }
    if type_id((1u16 as i32) + 0) != 1 {
        exit(4);
    }

    let bf = BF {
        u1: 1u32,
        i1: -1,
        b1: true,
    };

    if type_id((bf.u1 as i32) + 0) != 1 {
        exit(5);
    }
    if type_id((bf.i1 as i32) + 0) != 1 {
        exit(6);
    }
    let b1_val: i32 = if bf.b1 { 1 } else { 0 };
    if type_id(b1_val + 0) != 1 {
        exit(7);
    }

    if type_id(1.0f32) != 7 {
        exit(8);
    }
    if type_id(1.0f64) != 8 {
        exit(9);
    }
    if type_id(LongDouble(1.0f64)) != 9 {
        exit(10);
    }

    // Exit 0 by default
}