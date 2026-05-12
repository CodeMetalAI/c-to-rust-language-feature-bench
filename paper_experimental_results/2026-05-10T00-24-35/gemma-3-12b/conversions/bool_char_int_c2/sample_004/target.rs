#[derive(Debug, PartialEq)]
struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

macro_rules! type_id {
    ($x:expr) => {
        match $x {
            i32 | u32 => 1,
            i64 | u64 => 1,
            i16 | u16 => 1,
            i8 | u8 => 1,
            f32 => 7,
            f64 => 8,
            f128 => 9,
            _ => 99,
        }
    };
}

fn main() {
    if type_id((1i8 as signed char) + 0) != 1 {
        return 1;
    }
    if type_id((1u8 as unsigned char) + 0) != 1 {
        return 2;
    }
    if type_id((1i16 as short) + 0) != 1 {
        return 3;
    }
    if type_id((1u16 as unsigned short) + 0) != 1 {
        return 4;
    }

    let mut bf = BF {
        u1: 1u32,
        i1: -1i32,
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

    if type_id(1.0f32) != 7 {
        return 8;
    }
    if type_id(1.0f64) != 8 {
        return 9;
    }
    if type_id(1.0f128) != 9 {
        return 10;
    }

    0
}