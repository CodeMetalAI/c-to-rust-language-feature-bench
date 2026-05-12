struct S {
    x: i32,
    y: i32,
}

fn f() -> S {
    S { x: 7, y: 11 }
}

fn main() {
    let s = f();
    let vx = s.x;
    let vy = s.y;

    if vx!= 7 {
        return 1;
    }
    if vy!= 11 {
        return 2;
    }

    if s.x + s.y!= 18 {
        return 3;
    }

    return 0;
}