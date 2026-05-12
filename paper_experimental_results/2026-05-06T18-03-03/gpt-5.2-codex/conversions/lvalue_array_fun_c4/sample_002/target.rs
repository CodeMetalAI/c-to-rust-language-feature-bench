use std::process::exit;

trait TypeId {
    const ID: i32;
}

impl TypeId for fn(i32) -> i32 {
    const ID: i32 = 1;
}

impl TypeId for fn(i32, i32) -> i32 {
    const ID: i32 = 2;
}

fn type_id<T: TypeId>(_: T) -> i32 {
    T::ID
}

fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn run() -> i32 {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if type_id(p1) != 1 {
        return 1;
    }
    if type_id(q1) != 1 {
        return 2;
    }

    if p1 != q1 {
        return 3;
    }

    if p1(3) != 4 {
        return 4;
    }
    if f1(3) != 4 {
        return 5;
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if type_id(r1) != 1 {
        return 6;
    }
    if r1(4) != 5 {
        return 7;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if type_id(p2) != 2 {
        return 8;
    }
    if type_id(q2) != 2 {
        return 9;
    }

    if p2 != q2 {
        return 10;
    }
    if p2(2, 3) != 5 {
        return 11;
    }
    if f2(2, 3) != 5 {
        return 12;
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if type_id(r2) != 2 {
        return 13;
    }
    if r2(10, 20) != 30 {
        return 14;
    }

    0
}

fn main() {
    let code = run();
    exit(code);
}