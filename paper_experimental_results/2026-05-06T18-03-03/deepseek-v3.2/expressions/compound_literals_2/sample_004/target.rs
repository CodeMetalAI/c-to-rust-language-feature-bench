fn f(init: i32) -> i32 {
    let x = init;
    let mut p = &x;

    // Simulate compound literal: create a two-element array on the stack
    let arr = [*p, 0];
    p = &arr[0];

    if p[0] != init {
        return 1;
    }
    // Check second element (index 1) of the array
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