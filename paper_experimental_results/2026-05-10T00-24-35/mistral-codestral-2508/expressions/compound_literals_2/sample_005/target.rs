fn f(init: i32) -> i32 {
    let mut x = init;
    let mut p = &mut x as *mut i32;

    let arr: [i32; 2] = [x, 0];
    p = &arr[0] as *const i32 as *mut i32;

    if unsafe { *p } != init {
        return 1;
    }
    if unsafe { *p.offset(1) } != 0 {
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