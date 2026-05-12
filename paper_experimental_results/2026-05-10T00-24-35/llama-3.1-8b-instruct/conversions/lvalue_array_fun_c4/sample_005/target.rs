// lvalue_array_fun.rs

fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if p1(3)!= 4 {
        return;
    }
    if f1(3)!= 4 {
        return;
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if r1(4)!= 5 {
        return;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if p2(2, 3)!= 5 {
        return;
    }
    if f2(2, 3)!= 5 {
        return;
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if r2(10, 20)!= 30 {
        return;
    }

    println!("All tests passed.");
}