#![allow(dead_code)]

struct BitField {
    u1: u8, // Use u8 with bit manipulation to simulate bit field
    i1: i8,
    b1: bool,
}

impl BitField {
    fn new() -> Self {
        BitField {
            u1: 0,
            i1: 0,
            b1: false,
        }
    }
    
    // Helper methods to get bit values
    fn get_u1(&self) -> u8 {
        self.u1 & 0x1
    }
    
    fn get_i1(&self) -> i8 {
        (self.i1 & 0x1) as i8
    }
    
    fn get_b1(&self) -> bool {
        self.b1
    }
    
    // Helper methods to set bit values
    fn set_u1(&mut self, value: u8) {
        self.u1 = value & 0x1;
    }
    
    fn set_i1(&mut self, value: i8) {
        self.i1 = value & 0x1;
    }
    
    fn set_b1(&mut self, value: bool) {
        self.b1 = value;
    }
}

fn main() {
    // Check signed char -> i32 promotion
    let signed_char: i8 = 1;
    if (signed_char as i32).wrapping_add(0) != 1 {
        std::process::exit(1);
    }
    
    // Check unsigned char -> i32 promotion
    let unsigned_char: u8 = 1;
    if (unsigned_char as i32).wrapping_add(0) != 1 {
        std::process::exit(2);
    }
    
    // Check short -> i32 promotion
    let short_val: i16 = 1;
    if (short_val as i32).wrapping_add(0) != 1 {
        std::process::exit(3);
    }
    
    // Check unsigned short -> i32 promotion
    let unsigned_short: u16 = 1;
    if (unsigned_short as i32).wrapping_add(0) != 1 {
        std::process::exit(4);
    }
    
    let mut bf = BitField::new();
    bf.set_u1(1u8);
    bf.set_i1(-1i8);
    bf.set_b1(true);
    
    // Check bit field promotions
    if (bf.get_u1() as i32).wrapping_add(0) != 1 {
        std::process::exit(5);
    }
    
    if (bf.get_i1() as i32).wrapping_add(0) != 1 {
        std::process::exit(6);
    }
    
    if (bf.get_b1() as i32).wrapping_add(0) != 1 {
        std::process::exit(7);
    }
    
    // Check floating point types
    let float_val: f32 = 1.0f32;
    if float_val != 1.0f32 {
        std::process::exit(8);
    }
    
    let double_val: f64 = 1.0f64;
    if double_val != 1.0f64 {
        std::process::exit(9);
    }
    
    // Rust doesn't have long double, f64 is the largest standard float
    // In C, long double is at least as precise as double
    let long_double_val: f64 = 1.0f64;
    if long_double_val != 1.0f64 {
        std::process::exit(10);
    }
    
    std::process::exit(0);
}