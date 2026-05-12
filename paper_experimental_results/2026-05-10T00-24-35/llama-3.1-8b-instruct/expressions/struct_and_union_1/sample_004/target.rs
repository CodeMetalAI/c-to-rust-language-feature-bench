struct S {
    x: i32,
    y: i32,
}

fn f() -> S {
    S { x: 7, y: 11 }
}

fn main() {
    let result = f();
    let vx = result.x;
    let vy = result.y;

    if vx != 7 {
        return 1;
    }
    if vy != 11 {
        return 2;
    }

    if result.x + result.y != 18 {
        return 3;
    }

    0
}