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

    if vx != 7 {
        std::process::exit(1);
    }
    if vy != 11 {
        std::process::exit(2);
    }

    let sum = f().x + f().y;
    if sum != 18 {
        std::process::exit(3);
    }
}