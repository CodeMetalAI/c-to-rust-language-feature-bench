struct S {
    x: i32,
    y: i32,
}

fn f() -> S {
    S { x: 7, y: 11 }
}

fn main() -> i32 {
    let vx = f().x;
    let vy = f().y;

    if vx != 7 {
        return 1;
    }
    if vy != 11 {
        return 2;
    }

    if (f().x + f().y) != 18 {
        return 3;
    }

    0
}