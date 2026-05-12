fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn type_id<F>(_: F) -> i32 {
    std::any::TypeId::of::<F>()
        .hash(&mut std::collections::hash_map::DefaultHasher::new()) as i32
}

fn main() -> i32 {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if type_id(p1) != type_id(f1 as fn(i32) -> i32) {
        return 1;
    }
    if type_id(q1) != type_id(f1 as fn(i32) -> i32) {
        return 2;
    }

    if p1 as usize != q1 as usize {
        return 3;
    }

    if p1(3) != 4 {
        return 4;
    }
    if f1(3) != 4 {
        return 5;
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if type_id(r1) != type_id(f1 as fn(i32) -> i32) {
        return 6;
    }
    if r1(4) != 5 {
        return 7;
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if type_id(p2) != type_id(f2 as fn(i32, i32) -> i32) {
        return 8;
    }
    if type_id(q2) != type_id(f2 as fn(i32, i32) -> i32) {
        return 9;
    }

    if p2 as usize != q2 as usize {
        return 10;
    }
    if p2(2, 3) != 5 {
        return 11;
    }
    if f2(2, 3) != 5 {
        return 12;
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if type_id(r2) != type_id(f2 as fn(i32, i32) -> i32) {
        return 13;
    }
    if r2(10, 20) != 30 {
        return 14;
    }

    0
}