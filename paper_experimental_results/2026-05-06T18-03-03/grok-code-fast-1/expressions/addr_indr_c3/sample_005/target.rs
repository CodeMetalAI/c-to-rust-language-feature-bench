fn main() {
    let x = 7;
    let p = &x as *const i32;

    if (&x as *const i32) != p {
        std::process::exit(1);
    }

    unsafe {
        if (&*p as *const i32) != p {
            std::process::exit(2);
        }
    }

    let a = [10, 20, 30];
    let aptr = a.as_ptr();

    if (&a[0] as *const i32) != aptr {
        std::process::exit(3);
    }
    if (&a[1] as *const i32) != aptr.wrapping_add(1) {
        std::process::exit(4);
    }
    if (&a[2] as *const i32) != aptr.wrapping_add(2) {
        std::process::exit(5);
    }

    unsafe {
        if *aptr.add(1) != 20 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}