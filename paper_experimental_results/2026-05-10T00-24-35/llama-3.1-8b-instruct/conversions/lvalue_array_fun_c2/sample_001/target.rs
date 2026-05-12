#[allow(unused)]
fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of::<i32>()!= std::mem::size_of_val(&x) {
        panic!("Size of x is not equal to size of i32");
    }
    if std::mem::align_of::<i32>()!= std::mem::align_of_val(&x) {
        panic!("Alignment of x is not equal to alignment of i32");
    }

    if *(&x)!= 3 {
        panic!("Lvalue reference of x is not 3");
    }

    let type_id = match x {
        _ => 99,
    };
    if type_id!= 1 {
        panic!("Type ID of x is not 1");
    }
    let type_id = match ax {
        std::sync::atomic::AtomicI32::new(val) => 3,
        _ => 99,
    };
    if type_id!= 3 {
        panic!("Type ID of ax is not 3");
    }

    let type_id = match +cx {
        _ => 99,
    };
    if type_id!= 1 {
        panic!("Type ID of cx is not 1");
    }
    let type_id = match +ax {
        _ => 99,
    };
    if type_id!= 1 {
        panic!("Type ID of ax is not 1");
    }

    let y = x;
    if y!= 3 {
        panic!("y is not equal to x");
    }

    x += 1;
    if x!= 4 {
        panic!("x is not incremented correctly");
    }

    struct S {
        m: i32,
    }
    let s = S { m: 7 };
    if s.m!= 7 {
        panic!("m of s is not 7");
    }
    s.m = 8;
    if s.m!= 8 {
        panic!("m of s is not updated correctly");
    }
}