fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;
    if std::any::type_id(&p1)!= std::any::type_id(&f1) {
        panic!("Expected 1");
    }
    if std::any::type_id(&q1)!= std::any::type_id(&f1) {
        panic!("Expected 1");
    }

    if std::ptr::eq(&p1, &q1) {
        panic!("Expected 3");
    }

    if p1(3)!= 4 {
        panic!("Expected 4");
    }
    if f1(3)!= 4 {
        panic!("Expected 5");
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if std::any::type_id(&r1)!= std::any::type_id(&f1) {
        panic!("Expected 6");
    }
    if r1(4)!= 5 {
        panic!("Expected 7");
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;
    if std::any::type_id(&p2)!= std::any::type_id(&f2) {
        panic!("Expected 8");
    }
    if std::any::type_id(&q2)!= std::any::type_id(&f2) {
        panic!("Expected 9");
    }

    if std::ptr::eq(&p2, &q2) {
        panic!("Expected 10");
    }
    if p2(2, 3)!= 5 {
        panic!("Expected 11");
    }
    if f2(2, 3)!= 5 {
        panic!("Expected 12");
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if std::any::type_id(&r2)!= std::any::type_id(&f2) {
        panic!("Expected 13");
    }
    if r2(10, 20)!= 30 {
        panic!("Expected 14");
    }

    println!("All tests passed");
}