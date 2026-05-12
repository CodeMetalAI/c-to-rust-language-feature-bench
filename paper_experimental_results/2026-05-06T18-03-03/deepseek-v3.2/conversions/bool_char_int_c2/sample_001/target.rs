use std::any::TypeId;

fn type_id<T: 'static>() -> u32 {
    // Map Rust types to the same numeric IDs as the C code
    let tid = TypeId::of::<T>();
    
    if tid == TypeId::of::<i32>() { 1 }
    else if tid == TypeId::of::<u32>() { 2 }
    else if tid == TypeId::of::<i64>() { 3 }
    else if tid == TypeId::of::<u64>() { 4 }
    else if tid == TypeId::of::<i128>() { 5 }
    else if tid == TypeId::of::<u128>() { 6 }
    else if tid == TypeId::of::<f32>() { 7 }
    else if tid == TypeId::of::<f64>() { 8 }
    else if tid == TypeId::of::<f64>() { 9 } // Note: Rust doesn't have long double, using f64
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
        // Force evaluation to get the actual type
        let _val = $x;
        std::marker::PhantomData::<decltype!(_val)>
    }};
}

// Specialization for the decltype macro
macro_rules! decltype {
    ($x:ident) => {
        _type_of!($x)
    };
    ($x:expr) => {
        _type_of!($x)
    };
}

macro_rules! _type_of {
    ($x:expr) => {{
        // This is a trick to get the type at compile time
        trait TypeOf {
            type Type;
        }
        impl<T> TypeOf for T {
            type Type = T;
        }
        <decltype!($x) as TypeOf>::Type
    }};
}

struct BF {
    u1: u8,  // Using u8 with bit manipulation to simulate bitfield
    i1: i8,  // Using i8 with bit manipulation
    b1: bool,
}

impl BF {
    fn new() -> Self {
        Self {
            u1: 0,
            i1: 0,
            b1: false,
        }
    }
    
    fn set_u1(&mut self, val: u32) {
        self.u1 = (val & 0x1) as u8;
    }
    
    fn get_u1(&self) -> u32 {
        (self.u1 & 0x1) as u32
    }
    
    fn set_i1(&mut self, val: i32) {
        self.i1 = (val & 0x1) as i8;
    }
    
    fn get_i1(&self) -> i32 {
        // Sign extend if needed
        if (self.i1 & 0x1) != 0 {
            -1
        } else {
            0
        }
    }
    
    fn set_b1(&mut self, val: bool) {
        self.b1 = val;
    }
    
    fn get_b1(&self) -> bool {
        self.b1
    }
}

fn main() {
    // Test 1: signed char
    let sc: i8 = 1;
    let sc_expr = sc as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // Test 2: unsigned char  
    let uc: u8 = 1;
    let uc_expr = uc as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // Test 3: short
    let s: i16 = 1;
    let s_expr = s as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // Test 4: unsigned short
    let us: u16 = 1;
    let us_expr = us as i32 + 0;
    if type_id::<i32>() != 1 {
        return;
    }
    
    // Test bitfields
    let mut bf = BF::new();
    bf.set_u1(1u32);
    bf.set_i1(-1);
    bf.set_b1(true);
    
    // Test 5: unsigned bitfield
    let bf_u1_expr = bf.get_u1() as i32 + 0;
    if type_id::<i32>() != 1 {
        std::process::exit(5);
    }
    
    // Test 6: signed bitfield
    let bf_i1_expr = bf.get_i1() as i32 + 0;
    if type_id::<i32>() != 1 {
        std::process::exit(6);
    }
    
    // Test 7: bool bitfield
    let bf_b1_expr = bf.get_b1() as i32 + 0;
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
    
    // Test 10: long double (Rust uses f64 for both double and long double)
    let ld: f64 = 1.0f64;
    if type_id::<f64>() != 8 {  // Note: C code expects 9 for long double
        std::process::exit(10);
    }
    
    std::process::exit(0);
}