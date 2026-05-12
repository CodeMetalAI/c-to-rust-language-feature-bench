fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() -> i32 {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if type_id(p1) != 1 {
        return 1;
    }
    if type_id(q1) != 1 {
        return 2;
    }

    if p1 as *const _ != q1 as *const _ {
        return 3;
    }

    if p1(3) != 4 {
        return 4;
    }
    if f1(3) != 4 {
        return 5;
    }

    let r1: fn(i32) -> i32 = if 1 { f1 } else { f1 };
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

    if p2 as *const _ != q2 as *const _ {
        return 10;
    }
    if p2(2, 3) != 5 {
        return 11;
    }
    if f2(2, 3) != 5 {
        return 12;
    }

    let r2: fn(i32, i32) -> i32 = if 0 { f2 } else { f2 };
    if type_id(r2) != 2 {
        return 13;
    }
    if r2(10, 20) != 30 {
        return 14;
    }

    0
}

fn type_id<F: Fn(i32) -> i32>() -> i32 {
    1
}

fn type_id<F: Fn(i32, i32) -> i32>() -> i32 {
    2
}