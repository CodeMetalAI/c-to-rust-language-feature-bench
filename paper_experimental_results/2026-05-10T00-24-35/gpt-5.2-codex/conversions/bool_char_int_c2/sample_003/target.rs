struct LongDouble(f64);

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
impl TypeId for f32 {
    const ID: i32 = 7;
}
impl TypeId for f64 {
    const ID: i32 = 8;
}
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
    let code = if type_id((1i8 as i32) + 0) != 1 {
        1
    } else if type_id((1u8 as i32) + 0) != 1 {
        2
    } else if type_id((1i16 as i32) + 0) != 1 {
        3
    } else if type_id((1u16 as i32) + 0) != 1 {
        4
    } else {
        let mut bf = BF {
            u1: 0,
            i1: 0,
            b1: false,
        };
        bf.u1 = 1u32;
        bf.i1 = -1;
        bf.b1 = true;

        if type_id((bf.u1 as i32) + 0) != 1 {
            5
        } else if type_id((bf.i1 as i32) + 0) != 1 {
            6
        } else if type_id((bf.b1 as i32) + 0) != 1 {
            7
        } else if type_id(1.0f32) != 7 {
            8
        } else if type_id(1.0f64) != 8 {
            9
        } else if type_id(LongDouble(1.0f64)) != 9 {
            10
        } else {
            0
        }
    };

    std::process::exit(code);
}