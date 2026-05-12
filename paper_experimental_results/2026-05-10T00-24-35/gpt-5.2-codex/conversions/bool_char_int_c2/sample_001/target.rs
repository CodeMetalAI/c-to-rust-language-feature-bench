use std::process::exit;

trait TypeId {
    fn type_id(&self) -> i32;
}

impl TypeId for i32 {
    fn type_id(&self) -> i32 { 1 }
}
impl TypeId for u32 {
    fn type_id(&self) -> i32 { 2 }
}
impl TypeId for i64 {
    fn type_id(&self) -> i32 { 3 }
}
impl TypeId for u64 {
    fn type_id(&self) -> i32 { 4 }
}
impl TypeId for f32 {
    fn type_id(&self) -> i32 { 7 }
}
impl TypeId for f64 {
    fn type_id(&self) -> i32 { 8 }
}

struct LongDouble(f64);
impl TypeId for LongDouble {
    fn type_id(&self) -> i32 { 9 }
}

fn type_id<T: TypeId>(v: &T) -> i32 {
    v.type_id()
}

struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    if type_id(&((1i8 as i32) + 0)) != 1 { exit(1); }
    if type_id(&((1u8 as i32) + 0)) != 1 { exit(2); }
    if type_id(&((1i16 as i32) + 0)) != 1 { exit(3); }
    if type_id(&((1u16 as i32) + 0)) != 1 { exit(4); }

    let mut bf = BF { u1: 0, i1: 0, b1: false };
    bf.u1 = 1u32;
    bf.i1 = -1;
    bf.b1 = true;

    if type_id(&((bf.u1 as i32) + 0)) != 1 { exit(5); }
    if type_id(&(bf.i1 + 0)) != 1 { exit(6); }
    if type_id(&((bf.b1 as i32) + 0)) != 1 { exit(7); }

    let f: f32 = 1.0;
    if type_id(&f) != 7 { exit(8); }

    let d: f64 = 1.0;
    if type_id(&d) != 8 { exit(9); }

    let ld = LongDouble(1.0);
    if type_id(&ld) != 9 { exit(10); }

    exit(0);
}