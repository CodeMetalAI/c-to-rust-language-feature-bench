fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if p1 == q1 {
        eprintln!("error: p1 and q1 are the same");
        return 1;
    }

    if p1(3) != 4 {
        eprintln!("error: p1(3) != 4");
        return 2;
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if r1(4) != 5 {
        eprintln!("error: r1(4) != 5");
        return 3;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if p2 == q2 {
        eprintln!("error: p2 and q2 are the same");
        return 4;
    }

    if p2(2, 3) != 5 {
        eprintln!("error: p2(2, 3) != 5");
        return 5;
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if r2(10, 20) != 30 {
        eprintln!("error: r2(10, 20) != 30");
        return 6;
    }

    println!("tests passed");
}

fn type_id<T>(x: T) -> u8
where
    T: Fn(i32) -> i32,
{
    1
}

fn type_id2<T>(x: T) -> u8
where
    T: Fn(i32, i32) -> i32,
{
    2
}

fn test_suite() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if type_id(p1) != 1 {
        eprintln!("error: type_id(p1) != 1");
        return 7;
    }
    if type_id(q1) != 1 {
        eprintln!("error: type_id(q1) != 1");
        return 8;
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if type_id(r1) != 1 {
        eprintln!("error: type_id(r1) != 1");
        return 9;
    }
    if r1(4) != 5 {
        eprintln!("error: r1(4) != 5");
        return 10;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if type_id2(p2) != 2 {
        eprintln!("error: type_id2(p2) != 2");
        return 11;
    }
    if type_id2(q2) != 2 {
        eprintln!("error: type_id2(q2) != 2");
        return 12;
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if type_id2(r2) != 2 {
        eprintln!("error: type_id2(r2) != 2");
        return 13;
    }
    if r2(10, 20) != 30 {
        eprintln!("error: r2(10, 20) != 30");
        return 14;
    }
}

fn main() {
    test_suite();
    println!("tests passed");
}