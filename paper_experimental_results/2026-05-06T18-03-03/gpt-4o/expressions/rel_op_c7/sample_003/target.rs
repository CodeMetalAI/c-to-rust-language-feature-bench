fn main() {
    let x = 42;
    let p = &x;

    if p as *const i32 != &x as *const i32 {
        std::process::exit(1);
    }

    let p_end1 = unsafe { p.offset(1) };
    let p_end2 = unsafe { (&x as *const i32).offset(1) };

    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    if p_end1 == p {
        std::process::exit(3);
    }

    std::process::exit(0);
}