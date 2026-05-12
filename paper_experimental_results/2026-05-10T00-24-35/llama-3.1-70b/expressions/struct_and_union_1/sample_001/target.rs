// struct_and_union_1

#[derive(Debug, Clone, Copy)]
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
        std::process::exit(1);
    }
    if vy != 11 {
        std::process::exit(2);
    }

    if (f().x + f().y) != 18 {
        std::process::exit(3);
    }

    println!("");
}