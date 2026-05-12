fn obj_like() -> i32 {
    1 - 1
}

fn func_like(a: i32) -> i32 {
    a
}

fn main() {
    let x = obj_like();
    let y = func_like(3);

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}