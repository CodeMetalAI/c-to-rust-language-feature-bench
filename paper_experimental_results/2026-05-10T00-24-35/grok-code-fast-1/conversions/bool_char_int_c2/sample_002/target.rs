trait TypeId {
    const ID: i32;
}

impl TypeId for i32 {
    const ID: i32 = 1;
}

impl TypeId for f32 {
    const ID: i32 = 7;
}

impl TypeId for f64 {
    const ID: i32 = 8;
}

#[derive(Clone, Copy)]
struct LongDouble(f64);

impl TypeId for LongDouble {
    const ID: i32 = 9;
}

struct BF {
    u1: i32,
    i1: i32,
    b1: i32,
}

fn main() -> i32 {
    if i32::ID != 1 {
        return 1;
    }
    if i32::ID != 1 {
        return 2;
    }
    if i32::ID != 1 {
        return 3;
    }
    if i32::ID != 1 {
        return 4;
    }

    let mut bf = BF {
        u1: 0,
        i1: 0,
        b1: 0,
    };
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = 1;

    if i32::ID != 1 {
        return 5;
    }
    if i32::ID != 1 {
        return 6;
    }
    if i32::ID != 1 {
        return 7;
    }

    if f32::ID != 7 {
        return 8;
    }
    if f64::ID != 8 {
        return 9;
    }
    if LongDouble::ID != 9 {
        return 10;
    }

    0
}