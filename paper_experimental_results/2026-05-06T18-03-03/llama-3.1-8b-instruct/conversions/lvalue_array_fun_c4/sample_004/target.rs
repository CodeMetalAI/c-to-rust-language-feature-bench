fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if std::any::type_id(&p1) != std::any::type_id(&f1) {
        panic!("TYPE_ID(p1) != 1");
    }
    if std::any::type_id(&q1) != std::any::type_id(&f1) {
        panic!("TYPE_ID(q1) != 1");
    }

    if std::ptr::eq(&p1, &q1) {
        panic!("p1 == q1");
    }

    if p1(3) != 4 {
        panic!("p1(3) != 4");
    }
    if f1(3) != 4 {
        panic!("f1(3) != 4");
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if std::any::type_id(&r1) != std::any::type_id(&f1) {
        panic!("TYPE_ID(r1) != 1");
    }
    if r1(4) != 5 {
        panic!("r1(4) != 5");
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if std::any::type_id(&p2) != std::any::type_id(&f2) {
        panic!("TYPE_ID(p2) != 2");
    }
    if std::any::type_id(&q2) != std::any::type_id(&f2) {
        panic!("TYPE_ID(q2) != 2");
    }

    if std::ptr::eq(&p2, &q2) {
        panic!("p2 == q2");
    }
    if p2(2, 3) != 5 {
        panic!("p2(2, 3) != 5");
    }
    if f2(2, 3) != 5 {
        panic!("f2(2, 3) != 5");
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if std::any::type_id(&r2) != std::any::type_id(&f2) {
        panic!("TYPE_ID(r2) != 2");
    }
    if r2(10, 20) != 30 {
        panic!("r2(10, 20) != 30");
    }
}