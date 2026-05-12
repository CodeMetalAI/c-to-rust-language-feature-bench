use std::any::TypeId;

struct BitField {
    u1: u8,
    i1: u8,
    b1: u8,
}

impl BitField {
    fn new() -> Self {
        BitField { u1: 0, i1: 0, b1: 0 }
    }
    
    fn set_u1(&mut self, value: u8) {
        self.u1 = value & 0x1;
    }
    
    fn set_i1(&mut self, value: i8) {
        self.i1 = (value as u8) & 0x1;
    }
    
    fn set_b1(&mut self, value: bool) {
        self.b1 = value as u8;
    }
    
    fn get_u1(&self) -> u8 {
        self.u1
    }
    
    fn get_i1(&self) -> i8 {
        (self.i1 as i8) | (!0 << 1)
    }
    
    fn get_b1(&self) -> bool {
        self.b1 != 0
    }
}

fn type_id<T: 'static>(_: T) -> u32 {
    let type_id = TypeId::of::<T>();
    
    if type_id == TypeId::of::<i8>() || type_id == TypeId::of::<i16>() || 
       type_id == TypeId::of::<i32>() || type_id == TypeId::of::<i64>() || 
       type_id == TypeId::of::<i128>() || type_id == TypeId::of::<isize>() {
        return 1;
    }
    if type_id == TypeId::of::<u8>() || type_id == TypeId::of::<u16>() || 
       type_id == TypeId::of::<u32>() || type_id == TypeId::of::<u64>() || 
       type_id == TypeId::of::<u128>() || type_id == TypeId::of::<usize>() {
        return 2;
    }
    if type_id == TypeId::of::<i64>() {
        return 3;
    }
    if type_id == TypeId::of::<u64>() {
        return 4;
    }
    if type_id == TypeId::of::<i128>() {
        return 5;
    }
    if type_id == TypeId::of::<u128>() {
        return 6;
    }
    if type_id == TypeId::of::<f32>() {
        return 7;
    }
    if type_id == TypeId::of::<f64>() {
        return 8;
    }
    if type_id == TypeId::of::<f64>() {
        return 9;
    }
    
    99
}

fn main() {
    if type_id::<i32>((1i8 as i32) + 0) != 1 {
        std::process::exit(1);
    }
    if type_id::<i32>((1u8 as i32) + 0) != 1 {
        std::process::exit(2);
    }
    if type_id::<i32>((1i16 as i32) + 0) != 1 {
        std::process::exit(3);
    }
    if type_id::<i32>((1u16 as i32) + 0) != 1 {
        std::process::exit(4);
    }

    let mut bf = BitField::new();
    bf.set_u1(1);
    bf.set_i1(-1);
    bf.set_b1(true);

    if type_id::<i32>(bf.get_u1() as i32 + 0) != 1 {
        std::process::exit(5);
    }
    if type_id::<i32>(bf.get_i1() as i32 + 0) != 1 {
        std::process::exit(6);
    }
    if type_id::<i32>(bf.get_b1() as i32 + 0) != 1 {
        std::process::exit(7);
    }

    if type_id(1.0f32) != 7 {
        std::process::exit(8);
    }
    if type_id(1.0f64) != 8 {
        std::process::exit(9);
    }
    if type_id(1.0f64) != 9 {
        std::process::exit(10);
    }

    std::process::exit(0);
}