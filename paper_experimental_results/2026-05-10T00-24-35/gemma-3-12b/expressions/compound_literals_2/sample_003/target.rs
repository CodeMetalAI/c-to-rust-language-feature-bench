fn f(init: i32) -> i32 {
    let mut x = init;
    let mut p = &mut x as *mut i32;

    unsafe {
        *p = [init, 0];
    }

    if p.read_unchecked(0) != init {
        return 1;
    }
    if p.read_unchecked(1) != 0 {
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