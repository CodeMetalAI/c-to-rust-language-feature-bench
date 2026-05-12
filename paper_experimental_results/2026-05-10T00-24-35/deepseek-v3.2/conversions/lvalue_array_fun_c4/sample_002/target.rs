fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1;

    if std::any::TypeId::of::<fn(i32) -> i32>() != std::any::TypeId::of_val(&p1) {
        std::process::exit(1);
    }
    if std::any::TypeId::of::<fn(i32) -> i32>() != std::any::TypeId::of_val(&q1) {
        std::process::exit(2);
    }

    if p1 as usize != q1 as usize {
        std::process::exit(3);
    }

    if p1(3) != 4 {
        std::process::exit(4);
    }
    if f1(3) != 4 {
        std::process::exit(5);
    }

    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if std::any::TypeId::of::<fn(i32) -> i32>() != std::any::TypeId::of_val(&r1) {
        std::process::exit(6);
    }
    if r1(4) != 5 {
        std::process::exit(7);
    }

    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;

    if std::any::TypeId::of::<fn(i32, i32) -> i32>() != std::any::TypeId::of_val(&p2) {
        std::process::exit(8);
    }
    if std::any::TypeId::of::<fn(i32, i32) -> i32>() != std::any::TypeId::of_val(&q2) {
        std::process::exit(9);
    }

    if p2 as usize != q2 as usize {
        std::process::exit(10);
    }
    if p2(2, 3) != 5 {
        std::process::exit(11);
    }
    if f2(2, 3) != 5 {
        std::process::exit(12);
    }

    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if std::any::TypeId::of::<fn(i32, i32) -> i32>() != std::any::TypeId::of_val(&r2) {
        std::process::exit(13);
    }
    if r2(10, 20) != 30 {
        std::process::exit(14);
    }

    std::process::exit(0);
}