struct S {
    x: i32,
    y: i32,
}

impl S {
    fn new(x: i32, y: i32) -> Self {
        S { x, y }
    }
}

fn f() -> S {
    S::new(7, 11)
}

fn main() {
    let vx = f().x;
    let vy = f().y;

    if vx != 7 {
        return std::process::exit(1);
    }
    if vy != 11 {
        return std::process::exit(2);
    }

    if (f().x + f().y) != 18 {
        return std::process::exit(3);
    }

    std::process::exit(0);
}