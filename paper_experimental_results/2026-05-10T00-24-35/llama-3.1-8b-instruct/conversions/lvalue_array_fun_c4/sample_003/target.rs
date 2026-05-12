fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = &f1;

    assert_eq!(f1_type_id(p1), 1);
    assert_eq!(f1_type_id(q1), 1);

    assert_ne!(p1, q1);

    assert_eq!(p1(3), 4);
    assert_eq!(f1(3), 4);

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    assert_eq!(f1_type_id(r1), 1);
    assert_eq!(r1(4), 5);

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = &f2;

    assert_eq!(f2_type_id(p2), 2);
    assert_eq!(f2_type_id(q2), 2);

    assert_ne!(p2, q2);
    assert_eq!(p2(2, 3), 5);
    assert_eq!(f2(2, 3), 5);

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    assert_eq!(f2_type_id(r2), 2);
    assert_eq!(r2(10, 20), 30);
}

fn f1_type_id(f: fn(i32) -> i32) -> u32 {
    match f {
        f1 => 1,
        _ => 99,
    }
}

fn f2_type_id(f: fn(i32, i32) -> i32) -> u32 {
    match f {
        f2 => 2,
        _ => 99,
    }
}