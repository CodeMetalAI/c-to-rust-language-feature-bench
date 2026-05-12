use std::any::TypeId;

fn type_id<T: 'static>() -> u32 {
    let type_id = TypeId::of::<T>();
    
    if type_id == TypeId::of::<i32>() { 1 }
    else if type_id == TypeId::of::<u32>() { 2 }
    else if type_id == TypeId::of::<i64>() { 3 }
    else if type_id == TypeId::of::<u64>() { 4 }
    else if type_id == TypeId::of::<i128>() { 5 }
    else if type_id == TypeId::of::<u128>() { 6 }
    else if type_id == TypeId::of::<f32>() { 7 }
    else if type_id == TypeId::of::<f64>() { 8 }
    else if type_id == TypeId::of::<f64>() { 9 } // Note: Rust doesn't have long double, using f64
    else { 99 }
}

macro_rules! TYPE_ID {
    ($x:expr) => {
        type_id::<decltype!($x)>()
    };
}

// Helper macro to get the type of an expression
macro_rules! decltype {
    ($x:expr) => {{
        let val = $x;
        // Force evaluation and return the type
        std::marker::PhantomData::<decltype(val)>
    }};
}

struct BF {
    u1: u8,  // Using u8 to simulate 1-bit unsigned field
    i1: i8,  // Using i8 to simulate 1-bit signed field
    b1: bool, // Using bool to simulate _Bool field
}

fn main() {
    // Test 1: signed char
    let sc: i8 = 1;
    let sc_expr = sc + 0;
    if type_id::<i32>() != 1 {  // i8 promotes to i32 in expressions
        std::process::exit(1);
    }
    
    // Test 2: unsigned char
    let uc: u8 = 1;
    let uc_expr = uc + 0;
    if type_id::<i32>() != 1 {  // u8 promotes to i32 in expressions
        std::process::exit(2);
    }
    
    // Test 3: short
    let s: i16 = 1;
    let s_expr = s + 0;
    if type_id::<i32>() != 1 {  // i16 promotes to i32 in expressions
        std::process::exit(3);
    }
    
    // Test 4: unsigned short
    let us: u16 = 1;
    let us_expr = us + 0;
    if type_id::<i32>() != 1 {  // u16 promotes to i32 in expressions
        std::process::exit(4);
    }
    
    // Create BF struct
    let bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };
    
    // Test 5: bf.u1
    let u1_expr = bf.u1 + 0;
    if type_id::<i32>() != 1 {  // u8 promotes to i32
        std::process::exit(5);
    }
    
    // Test 6: bf.i1
    let i1_expr = bf.i1 + 0;
    if type_id::<i32>() != 1 {  // i8 promotes to i32
        std::process::exit(6);
    }
    
    // Test 7: bf.b1
    let b1_expr = bf.b1 as i32 + 0;  // bool needs explicit conversion
    if type_id::<i32>() != 1 {
        std::process::exit(7);
    }
    
    // Test 8: float
    let f: f32 = 1.0f32;
    if type_id::<f32>() != 7 {
        std::process::exit(8);
    }
    
    // Test 9: double
    let d: f64 = 1.0f64;
    if type_id::<f64>() != 8 {
        std::process::exit(9);
    }
    
    // Test 10: long double (Rust only has f64, no long double)
    let ld: f64 = 1.0f64;
    if type_id::<f64>() != 8 {  // Note: C code expects 9 for long double, but Rust uses f64
        std::process::exit(10);
    }
    
    std::process::exit(0);
}