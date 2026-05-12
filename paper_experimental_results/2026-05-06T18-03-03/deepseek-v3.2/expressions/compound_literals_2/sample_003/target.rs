fn f(init: i32) -> i32 {
    let x = init;
    let mut p = &x;

    // Simulate compound literal (int[2]){*p} as a fixed-size array on the stack
    let arr: [i32; 2] = [*p, 0];
    p = &arr[0];

    if p[0] != init {
        return 1;
    }
    // Use pointer arithmetic via slice indexing
    if p[1] != 0 {
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