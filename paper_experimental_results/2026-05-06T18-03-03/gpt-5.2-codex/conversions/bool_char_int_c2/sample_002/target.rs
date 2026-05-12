use std::process::exit;

trait TypeId {
    const ID: i32;
}

impl TypeId for i32 {
    const ID: i32 = 1;
}
impl TypeId for f32 {
    const ID: i32 = 7;
}
impl TypeId for f64 {
    const ID: i32 = 8;
}

struct LongDouble(f64);
impl TypeId for LongDouble {
    const ID: i32 = 9;
}

fn type_id<T: TypeId>(_: &T) -> i32 {
    T::ID
}

struct BF {
    u1: i32,
    i1: i32,
    b1: i32,
}

fn check(cond: bool, code: i32) {
    if !cond {
        exit(code);
    }
}

fn main() {
    // Simulate promotions to int
    let v1: i32 = 1; // (signed char)1 + 0
    let v2: i32 = 1; // (unsigned char)1 + 0
    let v3: i32 = 1; // (short)1 + 0
    let v4: i32 = 1; // (unsigned short)1 + 0

    check(type_id(&v1) == 1, 1);
    check(type_id(&v2) == 1, 2);
    check(type_id(&v3) == 1, 3);
    check(type_id(&v4) == 1, 4);

    let mut bf = BF { u1: 0, i1: 0, b1: 0 };
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = 1;

    check(type_id(&bf.u1) == 1, 5);
    check(type_id(&bf.i1) == 1, 6);
    check(type_id(&bf.b1) == 1, 7);

    let f: f32 = 1.0f32;
    let d: f64 = 1.0f64;
    let ld = LongDouble(1.0f64);

    check(type_id(&f) == 7, 8);
    check(type_id(&d) == 8, 9);
    check(type_id(&ld) == 9, 10);
}