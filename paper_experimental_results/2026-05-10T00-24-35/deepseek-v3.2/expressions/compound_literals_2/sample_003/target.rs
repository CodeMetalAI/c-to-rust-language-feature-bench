fn f(init: i32) -> i32 {
    let x = init;
    let mut p = &x as *const i32;

    // Simulate the compound literal (int[2]){*p}
    let arr: [i32; 2] = [unsafe { *p }, 0];
    p = arr.as_ptr();

    if unsafe { *p } != init {
        return 1;
    }
    if unsafe { *p.add(1) } != 0 {
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