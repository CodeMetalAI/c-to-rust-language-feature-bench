fn type_id<T: TypeId>(value: T) -> u8 {
    value.type_id()
}

trait TypeId {
    fn type_id(&self) -> u8;
}

impl TypeId for i8 {
    fn type_id(&self) -> u8 { 1 }
}

impl TypeId for u8 {
    fn type_id(&self) -> u8 { 1 }
}

impl TypeId for i16 {
    fn type_id(&self) -> u8 { 1 }
}

impl TypeId for u16 {
    fn type_id(&self) -> u8 { 1 }
}

impl TypeId for i32 {
    fn type_id(&self) -> u8 { 1 }
}

impl TypeId for u32 {
    fn type_id(&self) -> u8 { 2 }
}

impl TypeId for i64 {
    fn type_id(&self) -> u8 { 3 }
}

impl TypeId for u64 {
    fn type_id(&self) -> u8 { 4 }
}

impl TypeId for i128 {
    fn type_id(&self) -> u8 { 5 }
}

impl TypeId for u128 {
    fn type_id(&self) -> u8 { 6 }
}

impl TypeId for f32 {
    fn type_id(&self) -> u8 { 7 }
}

impl TypeId for f64 {
    fn type_id(&self) -> u8 { 8 }
}

struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    if type_id(1i8 + 0) != 1 {
        std::process::exit(1);
    }
    if type_id(1u8 + 0) != 1 {
        std::process::exit(2);
    }
    if type_id(1i16 + 0) != 1 {
        std::process::exit(3);
    }
    if type_id(1u16 + 0) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF {
        u1: 1u32,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 + 0) != 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1 + 0) != 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 as i32 + 0) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }

    std::process::exit(0);
}