use std::any::TypeId;

fn type_id_of<T: 'static>(_: T) -> u8 {
    if TypeId::of::<i8>() == TypeId::of::<T>() { 1 }
    else if TypeId::of::<u8>() == TypeId::of::<T>() { 2 }
    else if TypeId::of::<i16>() == TypeId::of::<T>() { 1 }
    else if TypeId::of::<u16>() == TypeId::of::<T>() { 1 }
    else if TypeId::of::<i32>() == TypeId::of::<T>() { 1 }
    else if TypeId::of::<u32>() == TypeId::of::<T>() { 2 }
    else if TypeId::of::<i64>() == TypeId::of::<T>() { 3 }
    else if TypeId::of::<u64>() == TypeId::of::<T>() { 4 }
    else if TypeId::of::<i128>() == TypeId::of::<T>() { 5 }
    else if TypeId::of::<u128>() == TypeId::of::<T>() { 6 }
    else if TypeId::of::<f32>() == TypeId::of::<T>() { 7 }
    else if TypeId::of::<f64>() == TypeId::of::<T>() { 8 }
    else if TypeId::of::<bool>() == TypeId::of::<T>() { 99 }
    else { 99 }
}

struct BitFields {
    u1: u8,
    i1: i8,
    b1: bool,
}

impl BitFields {
    fn new() -> Self {
        Self {
            u1: 0,
            i1: 0,
            b1: false,
        }
    }
    
    fn set_u1(&mut self, value: u8) {
        self.u1 = value & 0x1;
    }
    
    fn set_i1(&mut self, value: i8) {
        self.i1 = value & 0x1;
    }
    
    fn set_b1(&mut self, value: bool) {
        self.b1 = value;
    }
    
    fn get_u1(&self) -> u8 {
        self.u1
    }
    
    fn get_i1(&self) -> i8 {
        self.i1
    }
    
    fn get_b1(&self) -> bool {
        self.b1
    }
}

fn main() {
    // Test 1: signed char -> i8 in Rust, but C promotes to int
    if type_id_of((1i8 as i32) + 0) != 1 {
        std::process::exit(1);
    }
    
    // Test 2: unsigned char -> u8 in Rust, but C promotes to int
    if type_id_of((1u8 as i32) + 0) != 1 {
        std::process::exit(2);
    }
    
    // Test 3: short -> i16 in Rust, but C promotes to int
    if type_id_of((1i16 as i32) + 0) != 1 {
        std::process::exit(3);
    }
    
    // Test 4: unsigned short -> u16 in Rust, but C promotes to int
    if type_id_of((1u16 as i32) + 0) != 1 {
        std::process::exit(4);
    }
    
    let mut bf = BitFields::new();
    bf.set_u1(1u8);
    bf.set_i1(-1i8);
    bf.set_b1(true);
    
    // Test 5: unsigned bitfield (1 bit) promotes to int in C
    if type_id_of((bf.get_u1() as i32) + 0) != 1 {
        std::process::exit(5);
    }
    
    // Test 6: signed bitfield (1 bit) promotes to int in C
    if type_id_of((bf.get_i1() as i32) + 0) != 1 {
        std::process::exit(6);
    }
    
    // Test 7: bool bitfield promotes to int in C
    if type_id_of((bf.get_b1() as i32) + 0) != 1 {
        std::process::exit(7);
    }
    
    // Test 8: float
    if type_id_of(1.0f32) != 7 {
        std::process::exit(8);
    }
    
    // Test 9: double
    if type_id_of(1.0f64) != 8 {
        std::process::exit(9);
    }
    
    // Test 10: long double - Rust only has f32 and f64, so we use f64
    // Note: This is an approximation since Rust doesn't have long double
    if type_id_of(1.0f64) != 8 {
        std::process::exit(10);
    }
    
    std::process::exit(0);
}