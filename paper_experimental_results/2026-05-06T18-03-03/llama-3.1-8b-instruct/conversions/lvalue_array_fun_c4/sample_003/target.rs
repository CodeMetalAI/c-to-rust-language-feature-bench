fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if p1 != q1 {
        panic!("p1 and q1 should be the same");
    }

    if p1(3) != 4 {
        panic!("p1(3) should be 4");
    }
    if f1(3) != 4 {
        panic!("f1(3) should be 4");
    }

    let r1: fn(i32) -> i32 = f1;
    if r1 != p1 {
        panic!("r1 and p1 should be the same");
    }
    if r1(4) != 5 {
        panic!("r1(4) should be 5");
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if p2 != q2 {
        panic!("p2 and q2 should be the same");
    }
    if p2(2, 3) != 5 {
        panic!("p2(2, 3) should be 5");
    }
    if f2(2, 3) != 5 {
        panic!("f2(2, 3) should be 5");
    }

    let r2: fn(i32, i32) -> i32 = f2;
    if r2 != p2 {
        panic!("r2 and p2 should be the same");
    }
    if r2(10, 20) != 30 {
        panic!("r2(10, 20) should be 30");
    }
}