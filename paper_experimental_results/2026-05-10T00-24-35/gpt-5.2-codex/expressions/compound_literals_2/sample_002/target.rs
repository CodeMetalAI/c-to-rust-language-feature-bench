fn f(init: i32) -> i32 {
    let _x = init;
    let arr = [init, 0];

    if arr[0] != init {
        return 1;
    }
    if arr[1] != 0 {
        return 2;
    }
    0
}

fn main() {
    if f(7) != 0 {
        std::process::exit(10);
    }
    if f(-3) != 0 {
        std::process::exit(11);
    }
}