fn main() {
    let x = 42;
    let p = &x as *const i32;

    if p as usize!= &x as *const i32 as usize {
        return;
    }

    let p_end1 = p.offset(1);
    let p_end2 = (&x as *const i32).offset(1);

    if p_end1 as usize!= p_end2 as usize {
        std::process::exit(2);
    }

    if p_end1 as usize == p as usize {
        std::process::exit(3);
    }

    std::process::exit(0);
}