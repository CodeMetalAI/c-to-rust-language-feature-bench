fn type_id<T>(_value: T) -> u8 {
    std::any::TypeId::of::<T>()
        .hash(&mut std::collections::hash_map::DefaultHasher::new())
        .to_le_bytes()[0]
}

fn main() -> std::process::ExitCode {
    if type_id(1i8 + 0) != type_id(1i32) {
        return std::process::ExitCode::from(1);
    }
    if type_id(1u8 + 0) != type_id(1i32) {
        return std::process::ExitCode::from(2);
    }
    if type_id(1i16 + 0) != type_id(1i32) {
        return std::process::ExitCode::from(3);
    }
    if type_id(1u16 + 0) != type_id(1i32) {
        return std::process::ExitCode::from(4);
    }

    struct BF {
        u1: u8,
        i1: i8,
        b1: bool,
    }

    let bf = BF { u1: 1, i1: -1, b1: true };

    if type_id(bf.u1 + 0) != type_id(1i32) {
        return std::process::ExitCode::from(5);
    }
    if type_id(bf.i1 + 0) != type_id(1i32) {
        return std::process::ExitCode::from(6);
    }
    if type_id(bf.b1 as i32 + 0) != type_id(1i32) {
        return std::process::ExitCode::from(7);
    }

    if type_id(1.0f32) != type_id(1.0f32) {
        return std::process::ExitCode::from(8);
    }
    if type_id(1.0f64) != type_id(1.0f64) {
        return std::process::ExitCode::from(9);
    }
    if type_id(1.0f64) != type_id(1.0f64) {
        return std::process::ExitCode::from(10);
    }

    std::process::ExitCode::SUCCESS
}