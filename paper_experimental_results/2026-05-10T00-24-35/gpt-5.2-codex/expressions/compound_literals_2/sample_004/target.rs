fn f(init: i32) -> i32 {
    let x = init;
    let arr = [x, 0];

    if arr[0] != init {
        return 1;
    }
    if arr[1] != 0 {
        return 2;
    }

    0
}

fn main() {
    let rc = if f(7) != 0 {
        10
    } else if f(-3) != 0 {
        11
    } else {
        0
    };
    std::process::exit(rc);
}