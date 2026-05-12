fn type_id<T: std::fmt::Debug>(x: T) -> u8 {
    match x {
        _ if is_integral(x) => 1,
        _ if is_unsigned_integral(x) => 2,
        _ if is_floating_point(x) => 7,
        _ => 99,
    }
}

fn is_integral<T: std::fmt::Debug>(x: T) -> bool {
    std::mem::size_of::<T>() <= std::mem::size_of::<i32>()
}

fn is_unsigned_integral<T: std::fmt::Debug>(x: T) -> bool {
    std::mem::size_of::<T>() <= std::mem::size_of::<u32>()
}

fn is_floating_point<T: std::fmt::Debug>(x: T) -> bool {
    std::mem::size_of::<T>() >= std::mem::size_of::<f64>()
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if type_id(1_i8 + 0)!= 1 {
        panic!("Failed test 1");
    }
    if type_id(1_u8 + 0)!= 1 {
        panic!("Failed test 2");
    }
    if type_id(1_i16 + 0)!= 1 {
        panic!("Failed test 3");
    }
    if type_id(1_u16 + 0)!= 1 {
        panic!("Failed test 4");
    }

    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 as u8 + 0)!= 1 {
        panic!("Failed test 5");
    }
    if type_id(bf.i1 as i8 + 0)!= 1 {
        panic!("Failed test 6");
    }
    if type_id(bf.b1 + 0)!= 99 {
        panic!("Failed test 7");
    }

    if type_id(1.0f32)!= 7 {
        panic!("Failed test 8");
    }
    if type_id(1.0f64)!= 8 {
        panic!("Failed test 9");
    }
    if type_id(1.0f64)!= 8 {
        panic!("Failed test 10");
    }
}