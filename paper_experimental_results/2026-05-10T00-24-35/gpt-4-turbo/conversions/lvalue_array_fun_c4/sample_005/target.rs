fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if !is_type_id_1(p1) {
        return std::process::exit(1);
    }
    if !is_type_id_1(q1) {
        return std::process::exit(2);
    }

    if std::ptr::eq(p1 as *const (), q1 as *const ()) == false {
        return std::process::exit(3);
    }

    if p1(3) != 4 {
        return std::process::exit(4);
    }
    if f1(3) != 4 {
        return std::process::exit(5);
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if !is_type_id_1(r1) {
        return std::process::exit(6);
    }
    if r1(4) != 5 {
        return std::process::exit(7);
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if !is_type_id_2(p2) {
        return std::process::exit(8);
    }
    if !is_type_id_2(q2) {
        return std::process::exit(9);
    }

    if std::ptr::eq(p2 as *const (), q2 as *const ()) == false {
        return std::process::exit(10);
    }

    if p2(2, 3) != 5 {
        return std::process::exit(11);
    }
    if f2(2, 3) != 5 {
        return std::process::exit(12);
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if !is_type_id_2(r2) {
        return std::process::exit(13);
    }
    if r2(10, 20) != 30 {
        return std::process::exit(14);
    }
}

fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn is_type_id_1(f: fn(i32) -> i32) -> bool {
    std::mem::size_of_val(&f) == std::mem::size_of::<fn(i32) -> i32>()
}

fn is_type_id_2(f: fn(i32, i32) -> i32) -> bool {
    std::mem::size_of_val(&f) == std::mem::size_of::<fn(i32, i32) -> i32>()
}