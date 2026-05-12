fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn type_id<T>(_: fn(T) -> i32) -> usize {
    1
}

fn type_id2<T, U>(_: fn(T, U) -> i32) -> usize {
    2
}

fn main() -> Result<(), i32> {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if type_id(p1) != 1 {
        return Err(1);
    }
    if type_id(q1) != 1 {
        return Err(2);
    }

    if p1 as *const _ != q1 as *const _ {
        return Err(3);
    }

    if p1(3) != 4 {
        return Err(4);
    }
    if f1(3) != 4 {
        return Err(5);
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if type_id(r1) != 1 {
        return Err(6);
    }
    if r1(4) != 5 {
        return Err(7);
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if type_id2(p2) != 2 {
        return Err(8);
    }
    if type_id2(q2) != 2 {
        return Err(9);
    }

    if p2 as *const _ != q2 as *const _ {
        return Err(10);
    }
    if p2(2, 3) != 5 {
        return Err(11);
    }
    if f2(2, 3) != 5 {
        return Err(12);
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if type_id2(r2) != 2 {
        return Err(13);
    }
    if r2(10, 20) != 30 {
        return Err(14);
    }

    Ok(())
}