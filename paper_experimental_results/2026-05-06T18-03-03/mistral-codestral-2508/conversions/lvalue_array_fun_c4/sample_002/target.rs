fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if p1(3) != 4 {
        std::process::exit(4);
    }
    if f1(3) != 4 {
        std::process::exit(5);
    }

    let r1: fn(i32) -> i32 = f1;
    if r1(4) != 5 {
        std::process::exit(7);
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if p2(2, 3) != 5 {
        std::process::exit(11);
    }
    if f2(2, 3) != 5 {
        std::process::exit(12);
    }

    let r2: fn(i32, i32) -> i32 = f2;
    if r2(10, 20) != 30 {
        std::process::exit(14);
    }
}