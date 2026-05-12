fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, integer promotion rules differ from C.
    // pc + 0, sc + 0, uc + 0 will be i32 due to default integer promotion.
    // We'll check the type by using a helper that returns a type id.
    // Since Rust doesn't have _Generic, we'll emulate with match on type.
    // We'll create a macro that returns a type id for integer types.
    macro_rules! type_id {
        ($x:expr) => {
            match $x {
                _ if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<decltype($x)>() => 1,
                _ if std::any::TypeId::of::<u32>() == std::any::TypeId::of::<decltype($x)>() => 2,
                _ => 99,
            }
        };
    }

    // However, Rust's type inference makes this tricky. Instead, we'll rely on known behavior.
    // In Rust, adding 0 to an integer doesn't change its type, but when used in expressions,
    // the type is inferred. We'll check the type of (pc + 0) by using a function.
    // Let's define a function that returns the type id.
    fn get_type_id<T: 'static>(_: T) -> i32 {
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
            1
        } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
            2
        } else {
            99
        }
    }

    // Now check the types of pc+0, sc+0, uc+0.
    // In Rust, adding 0 to an integer will keep it as the same type unless there's a type inference.
    // But we want to mimic C's integer promotion. In C, char and short are promoted to int.
    // In Rust, there is no such promotion; operations on i8/u8 stay as i8/u8 unless cast.
    // However, when comparing or using in expressions, Rust may infer i32 for literals.
    // To mimic C, we'll explicitly cast to i32 when adding 0.
    let pc_plus_zero: i32 = pc as i32 + 0;
    let sc_plus_zero: i32 = sc as i32 +我们发现，Rust中没有C那样的整数提升规则。在C中，char和short在表达式中会自动提升为int。
    // 在Rust中，我们需要显式转换。所以我们将检查转换后的类型。
    if get_type_id(pc_plus_zero) != 1 {
        std::process::exit(1);
    }
    if get_type_id(sc_plus_zero) != 1 {
        std::process::exit(2);
    }
    let uc_plus_zero: i32 = uc as i32 + 0;
    if get_type_id(uc_plus_zero) != 1 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    // Check if char is signed (in Rust, i8 is always signed).
    // In C, char may be signed or unsigned. Here we assume it's signed.
    // The original code checks (char)-1 < 0. In Rust, i8::from(-1) is -1, which is < 0.
    if (pc as i8) < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(6);
        }
    } else {
        if (ipc as u32) != 255u32 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}