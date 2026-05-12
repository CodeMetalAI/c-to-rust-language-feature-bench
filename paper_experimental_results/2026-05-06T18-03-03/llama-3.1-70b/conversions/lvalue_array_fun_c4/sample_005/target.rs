fn f1(x: i32) -> i32 { x + 1 }
fn f2(x: i32, y: i32) -> i32 { x + y }

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if std::any::type_name::<fn(i32) -> i32>()!= std::any::type_name::<fn(i32) -> i32>() {
        return 1;
    }

    if p1 as *const ()!= q1 as *const () {
        return 3;
    }

    if p1(3)!= 4 {
        return 4;
    }
    if f1(3)!= 4 {
        return 5;
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if r1(4)!= 5 {
        return 7;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if std::any::type_name::<fn(i32, i32) -> i32>()!= std::any::type_name::<fn(i32, i32) -> i32>() {
        return 8;
    }

    if p2 as *const ()!= q2 as *const () {
        return 10;
    }
    if p2(2, 3)!= 5 {
        return 11;
    }
    if f2(2, 3)!= 5 {
        return 12;
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if r2(10, 20)!= 30 {
        return 14;
    }

    return 0;
}