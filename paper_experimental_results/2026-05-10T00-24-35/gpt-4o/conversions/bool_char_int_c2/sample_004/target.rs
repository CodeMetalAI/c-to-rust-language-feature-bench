fn type_id<T>(_value: T) -> i32 {
    std::any::TypeId::of::<T>()
}

fn main() -> i32 {
    if type_id(1i8).hash() != type_id(1i32).hash() {
        return 1;
    }
    if type_id(1u8).hash() != type_id(1i32).hash() {
        return 2;
    }
    if type_id(1i16).hash() != type_id(1i32).hash() {
        return 3;
    }
    if type_id(1u16).hash() != type_id(1i32).hash() {
        return 4;
    }

    struct BF {
        u1: i32,
        i1: i32,
        b1: i32,
    }

    let bf = BF {
        u1: 1,
        i1: -1,
        b1: 1,
    };

    if type_id(bf.u1).hash() != type_id(1i32).hash() {
        return 5;
    }
    if type_id(bf.i1).hash() != type_id(1i32).hash() {
        return 6;
    }
    if type_id(bf.b1).hash() != type_id(1i32).hash() {
        return 7;
    }

    if type_id(1.0f32).hash() != type_id(1.0f32).hash() {
        return 8;
    }
    if type_id(1.0f64).hash() != type_id(1.0f64).hash() {
        return 9;
    }
    if type_id(1.0f64).hash() != type_id(1.0f64).hash() {
        return 10;
    }

    0
}