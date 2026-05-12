fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = &f1;

    if TYPE_ID(&p1) != 1 {
        return 1;
    }
    if TYPE_ID(&q1) != 1 {
        return 2;
    }

    if p1 as *const fn(i32) -> i32 != q1 as *const fn(i32) -> i32 {
        return 3;
    }

    if p1(3) != 4 {
        return 4;
    }
    if f1(3) != 4 {
        return 5;
    }

    let r1: fn(i32) -> i32 = (if true { f1 } else { f1 });
    if TYPE_ID(&r1) != 1 {
        return 6;
    }
    if r1(4) != 5 {
        return 7;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = &f2;

    if TYPE_ID(&p2) != 2 {
        return 8;
    }
    if TYPE_ID(&q2) != 2 {
        return 9;
    }

    if p2 as *const fn(i32, i32) -> i32 != q2 as *const fn(i32, i32) -> i32 {
        return 10;
    }
    if p2(2, 3) != 5 {
        return 11;
    }
    if f2(2, 3) != 5 {
        return 12;
    }

    let r2: fn(i32, i32) -> i32 = (if false { f2 } else { f2 });
    if TYPE_ID(&r2) != 2 {
        return 13;
    }
    if r2(10, 20) != 30 {
        return 14;
    }

    0
}

#[allow(dead_code)]
fn TYPE_ID(func: &fn(i32) -> i32) -> i32 {
    if func == &f1 {
        1
    } else if func == &f2 {
        2
    } else {
        99
    }
}

#[allow(dead_code)]
fn TYPE_ID(func: &fn(i32, i32) -> i32) -> i32 {
    if func == &f1 {
        1
    } else if func == &f2 {
        2
    } else {
        99
    }
}