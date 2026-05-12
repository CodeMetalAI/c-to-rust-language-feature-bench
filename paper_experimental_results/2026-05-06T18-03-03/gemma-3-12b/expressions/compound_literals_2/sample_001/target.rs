fn f(init: i32) -> i32 {
    let mut x = init;
    let mut p = &mut x;

    *p = [*p];

    if p.to_raw() as *mut i32 != &mut x as *mut i32 {
        return 1;
    }

    let arr = [x, 0];
    unsafe {
        *p = arr[0];
        let p_ptr = p.to_raw();
        let p_mut = p.to_mut();
        *p_mut = arr[1];
    }

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
    std::process::exit(0);
}