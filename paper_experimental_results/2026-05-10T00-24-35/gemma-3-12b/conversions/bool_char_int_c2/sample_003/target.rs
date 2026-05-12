#[derive(Debug, PartialEq, Eq)]
struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

macro_rules! type_id {
    ($x:expr) => {
        match $x {
            i32 => 1,
            u32 => 2,
            i64 => 3,
            u64 => 4,
            i128 => 5,
            u128 => 6,
            f32 => 7,
            f64 => 8,
            f128 => 9,
            _ => 99,
        }
    };
}

fn main() {
    if type_id((1 as i8) as i32 + 0) != 1 {
        return 1;
    }
    if type_id((1 as u8) as u32 + 0) != 1 {
        return 2;
    }
    if type_id((1 as i16) as i32 + 0) != 1 {
        return 3;
    }
    if type_id((1 as u16) as u32 + 0) != 1 {
        return 4;
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 + 0) != 1 {
        return 5;
    }
    if type_id(bf.i1 + 0) != 1 {
        return 6;
    }
    if type_id(bf.b1 + 0) != 1 {
        return 7;
    }

    if type_id((1.0f32) as f32) != 7 {
        return 8;
    }
    if type_id((1.0 as f64) as f64) != 8 {
        return 9;
    }
    if type_id((1.0 as f128) as f128) != 9 {
        return 10;
    }

    0
}