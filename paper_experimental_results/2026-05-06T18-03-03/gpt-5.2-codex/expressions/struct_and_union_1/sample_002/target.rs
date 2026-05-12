use std::process::exit;

#[derive(Copy, Clone)]
struct S {
    x: i32,
    y: i32,
}

fn f() -> S {
    S { x: 7, y: 11 }
}

fn main() {
    let vx = f().x;
    let vy = f().y;

    if vx != 7 {
        exit(1);
    }
    if vy != 11 {
        exit(2);
    }

    if (f().x + f().y) != 18 {
        exit(3);
    }

    exit(0);
}