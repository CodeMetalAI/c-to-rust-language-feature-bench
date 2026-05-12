use std::any::TypeId;

#[derive(Clone, Copy)]
struct Double(f64);

#[derive(Clone, Copy)]
struct LongDouble(f64);

trait MyTypeId {
    fn my_type_id() -> i32;
}

impl MyTypeId for i32 {
    fn my_type_id() -> i32 { 1 }
}

impl MyTypeId for u32 {
    fn my_type_id() -> i32 { 2 }
}

impl MyTypeId for i64 {
    fn my_type_id() -> i32 { 3 }
}

impl MyTypeId for u64 {
    fn my_type_id() -> i32 { 4 }
}

impl MyTypeId for f32 {
    fn my_type_id() -> i32 { 7 }
}

impl MyTypeId for Double {
    fn my_type_id() -> i32 { 8 }
}

impl MyTypeId for LongDouble {
    fn my_type_id() -> i32 { 9 }
}

fn type_id<T: 'static>(_: &T) -> i32 {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        1
    } else if tid == TypeId::of::<u32>() {
        2
    } else if tid == TypeId::of::<i64>() {
        3
    } else if tid == TypeId::of::<u64>() {
        4
    } else if tid == TypeId::of::<f32>() {
        7
    } else if tid == TypeId::of::<Double>() {
        8
    } else if tid == TypeId::of::<LongDouble>() {
        9
    } else {
        99
    }
}

#[derive(Clone, Copy)]
struct BF {
    u1: i32,
    i1: i32,
    b1: bool,
}

fn main() {
    if type_id(&(1i8 + 0)) != 1 {
        std::process::exit(1);
    }
    if type_id(&(1u8 + 0)) != 1 {
        std::process::exit(2);
    }
    if type_id(&(1i16 + 0)) != 1 {
        std::process::exit(3);
    }
    if type_id(&(1u16 + 0)) != 1 {
        std::process::exit(4);
    }

    let mut bf = BF { u1: 0, i1: 0, b1: false };
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;

    if type_id(&(bf.u1 + 0)) != 1 {
        std::process::exit(5);
    }
    if type_id(&(bf.i1 + 0)) != 1 {
        std::process::exit(6);
    }
    if type_id(&((bf.b1 as i32) + 0)) != 1 {
        std::process::exit(7);
    }

    if type_id(&1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(&Double(1.0)) != 8 {
        std::process::exit(9);
    }
    if type_id(&LongDouble(1.0)) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}