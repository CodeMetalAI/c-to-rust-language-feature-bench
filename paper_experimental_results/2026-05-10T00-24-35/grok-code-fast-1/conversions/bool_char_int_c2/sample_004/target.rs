trait TypeId {
    fn id() -> i32;
}

impl TypeId for i32 {
    fn id() -> i32 { 1 }
}

impl TypeId for u32 {
    fn id() -> i32 { 2 }
}

impl TypeId for i64 {
    fn id() -> i32 { 3 }
}

impl TypeId for u64 {
    fn id() -> i32 { 4 }
}

impl TypeId for i128 {
    fn id() -> i32 { 5 }
}

impl TypeId for u128 {
    fn id() -> i32 { 6 }
}

impl TypeId for f32 {
    fn id() -> i32 { 7 }
}

impl TypeId for f64 {
    fn id() -> i32 { 8 }
}

type LongDouble = f64;

impl TypeId for LongDouble {
    fn id() -> i32 { 9 }
}

struct BF {
    u1: i32,
    i1: i32,
    b1: i32,
}

fn main() {
    if i32::id() != 1 {
        std::process::exit(1);
    }
    if i32::id() != 1 {
        std::process::exit(2);
    }
    if i32::id() != 1 {
        std::process::exit(3);
    }
    if i32::id() != 1 {
        std::process::exit(4);
    }

    let mut bf = BF { u1: 1, i1: -1, b1: 1 };

    if i32::id() != 1 {
        std::process::exit(5);
    }
    if i32::id() != 1 {
        std::process::exit(6);
    }
    if i32::id() != 1 {
        std::process::exit(7);
    }

    if f32::id() != 7 {
        std::process::exit(8);
    }
    if f64::id() != 8 {
        std::process::exit(9);
    }
    if LongDouble::id() != 9 {
        std::process::exit(10);
    }
}