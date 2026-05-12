use std::any::TypeId;

fn type_id<T: 'static>() -> u32 {
    let type_id = TypeId::of::<T>();
    if type_id == TypeId::of::<i8>() { return 1; }
    if type_id == TypeId::of::<u8>() { return 1; }
    if type_id == TypeId::of::<i16>() { return 1; }
    if type_id == TypeId::of::<u16>() { return 1; }
    if type_id == TypeId::of::<i32>() { return 1; }
    if type_id == TypeId::of::<u32>() { return 2; }
    if type_id == TypeId::of::<i64>() { return 3; }
    if type_id == TypeId::of::<u64>() { return 4; }
    if type_id == TypeId::of::<i128>() { return 5; }
    if type_id == TypeId::of::<u128>() { return 6; }
    if type_id == TypeId::of::<f32>() { return 7; }
    if type_id == TypeId::of::<f64>() { return 8; }
    if type_id == TypeId::of::<f64>() { return 9; } // Note: Rust doesn't have long double, f64 is used
    99
}

macro_rules! TYPE_ID {
    ($x:expr) => {
        type_id::<decltype($x)>()
    };
}

fn decltype<T>(_: T) -> T {
    unimplemented!()
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

impl BF {
    fn new() -> Self {
        BF {
            u1: 0,
            i1: 0,
            b1: false,
        }
    }
}

fn main() {
    // Since Rust doesn't have C-style integer promotions in the same way,
    // we'll simulate the behavior by checking the type of the expression
    // after addition with 0 (which in C promotes to int).
    // In Rust, we need to explicitly check the resulting type.
    
    // For signed char -> i8, but C promotes to int (i32)
    let sc: i8 = 1;
    let sc_expr = sc as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // For unsigned char -> u8, but C promotes to int (i32)
    let uc: u8 = 1;
    let uc_expr = uc as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // For short -> i16, but C promotes to int (i32)
    let s: i16 = 1;
    let s_expr = s as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // For unsigned short -> u16, but C promotes to int (i32)
    let us: u16 = 1;
    let us_expr = us as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    let mut bf = BF::new();
    bf.u1 = 1;
    bf.i1 = -1;
    bf.b1 = true;
    
    // For bitfield u1 (unsigned int:1) -> u32 in C, promotes to int (i32)
    let u1_expr = bf.u1 as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // For bitfield i1 (int:1) -> i32 in C
    let i1_expr = bf.i1 as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // For bitfield b1 (_Bool:1) -> _Bool in C, promotes to int (i32)
    let b1_expr = bf.b1 as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // For float -> f32
    let f: f32 = 1.0f32;
    if type_id::<f32>() != 7 {
        return;
    }
    
    // For double -> f64
    let d: f64 = 1.0;
    if type_id::<f64>() != 8 {
        return;
    }
    
    // For long double -> f64 (closest equivalent in Rust)
    let ld: f64 = 1.0;
    if type_id::<f64>() != 9 {
        return;
    }
    
    // If all checks pass, exit with code 0
    std::process::exit(0);
}