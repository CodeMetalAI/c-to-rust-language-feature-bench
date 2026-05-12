trait TypeId {
    fn type_id(&self) -> i32;
}

impl TypeId for i32 {
    fn type_id(&self) -> i32 { 1 }
}

impl TypeId for u32 {
    fn type_id(&self) -> i32 { 2 }
}

impl TypeId for i64 {
    fn type_id(&self) -> i32 { 3 }
}

impl TypeId for u64 {
    fn type_id(&self) -> i32 { 4 }
}

impl TypeId for i128 {
    fn type_id(&self) -> i32 { 5 }
}

impl TypeId for u128 {
    fn type_id(&self) -> i32 { 6 }
}

impl TypeId for f32 {
    fn type_id(&self) -> i32 { 7 }
}

impl TypeId for f64 {
    fn type_id(&self) -> i32 { 8 }
}

struct LongDouble(f64);

impl TypeId for LongDouble {
    fn type_id(&self) -> i32 { 9 }
}

struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    if ((1i8 as i32) + 0).type_id() != 1 {
        std::process::exit(1);
    }
    if ((1u8 as i32) + 0).type_id() != 1 {
        std::process::exit(2);
    }
    if ((1i16 as i32) + 0).type_id() != 1 {
        std::process::exit(3);
    }
    if ((1u16 as i32) + 0).type_id() != 1 {
        std::process::exit(4);
    }

    let mut bf = BF { u1: 1, i1: -1, b1: true };

    if ((bf.u1 as i32) + 0).type_id() != 1 {
        std::process::exit(5);
    }
    if (bf.i1 + 0).type_id() != 1 {
        std::process::exit(6);
    }
    if ((bf.b1 as i32) + 0).type_id() != 1 {
        std::process::exit(7);
    }

    if (1.0f32).type_id() != 7 {
        std::process::exit(8);
    }
    if (1.0f64).type_id() != 8 {
        std::process::exit(9);
    }
    if LongDouble(1.0f64).type_id() != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}