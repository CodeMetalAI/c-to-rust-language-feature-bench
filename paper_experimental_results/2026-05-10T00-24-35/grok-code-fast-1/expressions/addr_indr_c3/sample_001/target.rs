fn main() -> i32 {
    let x = 7i32;
    let p = &x;

    if !std::ptr::eq(&x as *const _, p as *const _) {
        return 1;
    }

    if !std::ptr::eq(&(*p) as *const _, p as *const _) {
        return 2;
    }

    let a = [10i32, 20, 30];

    if !std::ptr::eq(&a[0] as *const _, a.as_ptr()) {
        return 3;
    }

    unsafe {
        if !std::ptr::eq(&a[1] as *const _, a.as_ptr().add(1)) {
            return 4;
        }

        if !std::ptr::eq(&a[2] as *const _, a.as_ptr().add(2)) {
            return 5;
        }

        if *a.as_ptr().add(1) != 20 {
            return 6;
        }
    }

    return 0;
}