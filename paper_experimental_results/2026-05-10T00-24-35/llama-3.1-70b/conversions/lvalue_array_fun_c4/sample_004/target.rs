#[allow(unused_macros)]
macro_rules! type_id {
    ($x:expr) => {{
        match (&$x).type_id() {
            s if s == std::ptr::null::<fn(i32) -> i32>().type_id() => 1,
            s if s == std::ptr::null::<fn(i32, i32) -> i32>().type_id() => 2,
            _ => 99,
        }
    }};
}

fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1 = f1;
    let q1 = f1;

    if type_id!(p1) != 1 {
        return;
    }
    if type_id!(q1) != 1 {
        return;
    }

    if (p1 as usize) != (q1 as usize) {
        return;
    }

    if p1(3) != 4 {
        return;
    }
    if f1(3) != 4 {
        return;
    }

    let r1 = if true { f1 } else { f1 };
    if type_id!(r1) != 1 {
        return;
    }
    if r1(4) != 5 {
        return;
    }

    let p2 = f2;
    let q2 = f2;

    if type_id!(p2) != 2 {
        return;
    }
    if type_id!(q2) != 2 {
        return;
    }

    if (p2 as usize) != (q2 as usize) {
        return;
    }
    if p2(2, 3) != 5 {
        return;
    }
    if f2(2, 3) != 5 {
        return;
    }

    let r2 = if false { f2 } else { f2 };
    if type_id!(r2) != 2 {
        return;
    }
    if r2(10, 20) != 30 {
        return;
    }

    println!("Success");
}