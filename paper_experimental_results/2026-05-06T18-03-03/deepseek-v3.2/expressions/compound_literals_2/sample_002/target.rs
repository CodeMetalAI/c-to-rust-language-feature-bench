fn f(init: i32) -> i32 {
    let x = init;
    let mut p = &x;

    // Simulate compound literal: create a new array with p[0] = *p, p[1] = 0
    let arr = [*p, 0];
    p = &arr[0];

    if p[0] != init {
        return 1;
    }
    if arr[1] != 0 {
        return 2;
    }

    return 0;
}

fn main() {
    if f(7) != 0 {
        return std::process::exit(10);
    }
    if f(-3) != 0 {
        return std::process::exit(11);
    }
    std::process::exit(0);
}