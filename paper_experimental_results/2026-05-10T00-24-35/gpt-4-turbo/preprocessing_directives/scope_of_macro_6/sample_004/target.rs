fn main() {
    let x = 0;
    let y = func_like(3);

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }
}

fn func_like(a: i32) -> i32 {
    a
}