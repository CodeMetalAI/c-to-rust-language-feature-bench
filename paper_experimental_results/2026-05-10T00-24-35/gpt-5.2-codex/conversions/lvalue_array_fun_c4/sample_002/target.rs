use std::process::exit;

fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

trait TypeId {
    fn type_id(&self) -> i32;
}

impl TypeId for fn(i32) -> i32 {
    fn type_id(&self) -> i32 {
        1
    }
}

impl TypeId for fn(i32, i32) -> i32 {
    fn type_id(&self) -> i32 {
        2
    }
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if p1.type_id() != 1 {
        exit(1);
    }
    if q1.type_id() != 1 {
        exit(2);
    }

    if p1 != q1 {
        exit(3);
    }

    if p1(3) != 4 {
        exit(4);
    }
    if f1(3) != 4 {
        exit(5);
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if r1.type_id() != 1 {
        exit(6);
    }
    if r1(4) != 5 {
        exit(7);
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if p2.type_id() != 2 {
        exit(8);
    }
    if q2.type_id() != 2 {
        exit(9);
    }

    if p2 != q2 {
        exit(10);
    }
    if p2(2, 3) != 5 {
        exit(11);
    }
    if f2(2, 3) != 5 {
        exit(12);
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if r2.type_id() != 2 {
        exit(13);
    }
    if r2(10, 20) != 30 {
        exit(14);
    }

    exit(0);
}