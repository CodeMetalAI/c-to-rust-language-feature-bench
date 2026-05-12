fn f(init: i32) -> i32 {
    let mut x = init;
    let mut p = &mut x;

    let arr = [x, 0];
    *p = arr[0];

    if p != &init {
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
    std::process::exit(0);
}