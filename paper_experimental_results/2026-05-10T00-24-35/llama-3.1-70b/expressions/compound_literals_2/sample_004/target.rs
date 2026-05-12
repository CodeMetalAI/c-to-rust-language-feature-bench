fn f(init: i32) -> i32 {
    let mut x: i32 = init;
    let p: &mut [i32; 2] = unsafe { &mut *(x as *mut i32 as *mut [i32; 2]) };

    if p[0] != init {
        return 1;
    }
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