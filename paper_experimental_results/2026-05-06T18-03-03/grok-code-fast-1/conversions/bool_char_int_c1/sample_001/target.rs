use std::process;

#[derive(Clone, Copy)]
enum E {
    Neg = -1,
    Pos = 1,
}

trait TypeId {
    fn id(&self) -> i32;
}

impl TypeId for i8 {
    fn id(&self) -> i32 {
        3
    }
}

impl TypeId for u8 {
    fn id(&self) -> i32 {
        4
    }
}

impl TypeId for i16 {
    fn id(&self) -> i32 {
        5
    }
}

impl TypeId for u16 {
    fn id(&self) -> i32 {
        6
    }
}

impl TypeId for i32 {
    fn id(&self) -> i32 {
        7
    }
}

impl TypeId for u32 {
    fn id(&self) -> i32 {
        8
    }
}

impl TypeId for i64 {
    fn id(&self) -> i32 {
        9
    }
}

impl TypeId for u64 {
    fn id(&self) -> i32 {
        10
    }
}

impl TypeId for i128 {
    fn id(&self) -> i32 {
        11
    }
}

impl TypeId for u128 {
    fn id(&self) -> i32 {
        12
    }
}

fn main() {
    if (true as i32 + 0).id() != 7 {
        process::exit(1);
    }
    if (1i8 + 0).id() != 7 {
        process::exit(2);
    }
    if (1i8 + 0).id() != 7 {
        process::exit(3);
    }
    if (1u8 + 0).id() != 7 {
        process::exit(4);
    }
    if (1i16 + 0).id() != 7 {
        process::exit(5);
    }
    if (1u16 + 0).id() != 7 {
        process::exit(6);
    }
    if (0i32 as u32 + 0u32).id() != 8 {
        process::exit(7);
    }
    if (0i64 as u64 + 0u64).id() != 10 {
        process::exit(8);
    }
    if (0i128 as u128 + 0u128).id() != 12 {
        process::exit(9);
    }
    if (0i32 + 0i64).id() != 9 {
        process::exit(10);
    }
    if (0i64 + 0i128).id() != 11 {
        process::exit(11);
    }
    if (0i32 + 0i128).id() != 11 {
        process::exit(12);
    }
    if ((E::Pos as i32) + 0u32).id() != (0i32 + 0u32).id() {
        process::exit(13);
    }
    if ((E::Pos as i32) + 0).id() != (0i32 + 0).id() {
        process::exit(14);
    }
}