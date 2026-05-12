fn main() {
    if type_id(1i8) != 1 {
        std::process::exit(1);
    }
    if type_id(1u8) != 1 {
        std::process::exit(2);
    }
    if type_id(1i16) != 1 {
        std::process::exit(3);
    }
    if type_id(1u16) != 1 {
        std::process::exit(4);
    }

    let bf = BitFields {
        u1: 1u32,
        i1: -1i32,
        b1: true,
    };

    if type_id(bf.u1 as i32) != 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1) != 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 as i32) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }
    if type_id(1.0) != 8 {
        std::process::exit(10);
    }

    std::process::exit(0);
}

struct BitFields {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn type_id<T: TypeId>(value: T) -> i32 {
    T::TYPE_ID
}

trait TypeId {
    const TYPE_ID: i32;
}

impl TypeId for i8 {
    const TYPE_ID: i32 = 1;
}

impl TypeId for u8 {
    const TYPE_ID: i32 = 1;
}

impl TypeId for i16 {
    const TYPE_ID: i32 = 1;
}

impl TypeId for u16 {
    const TYPE_ID: i32 = 1;
}

impl TypeId for i32 {
    const TYPE_ID: i32 = 1;
}

impl TypeId for u32 {
    const TYPE_ID: i32 = 1;
}

impl TypeId for i64 {
    const TYPE_ID: i32 = 1;
}

impl TypeId for u64 {
    const TYPE_ID: i32 = 1;
}

impl TypeId for f32 {
    const TYPE_ID: i32 = 7;
}

impl TypeId for f64 {
    const TYPE_ID: i32 = 8;
}

impl TypeId for bool {
    // Interpretation as `i32` for true=1
    const TYPE_ID: i32 = 1;
}