fn type_id<T: TypeId>(_: T) -> u8 {
    T::ID
}

trait TypeId {
    const ID: u8;
}

macro_rules! impl_type_id {
    ($($t:ty => $id:expr),*) => {
        $(impl TypeId for $t {
            const ID: u8 = $id;
        })*
    };
}

impl_type_id! {
    i32 => 1,
    u32 => 2,
    i64 => 3,
    u64 => 4,
    i128 => 5,
    u128 => 6,
    f32 => 7,
    f64 => 8,
    // Rust doesn't have a direct equivalent to long double, so we'll use f64 for this case
    f64 => 9
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if type_id((1i8 + 0)) != 1 {
        std::process::exit(1);
    }
    if type_id((1u8 + 0)) != 1 {
        std::process::exit(2);
    }
    if type_id((1i16 + 0)) != 1 {
        std::process::exit(3);
    }
    if type_id((1u16 + 0)) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 + 0) != 1 {
        std::process::exit(5);
    }
    if type_id(bf.i1 + 0) != 1 {
        std::process::exit(6);
    }
    if type_id(bf.b1 as i8 + 0) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }
    if type_id(1.0f64) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}