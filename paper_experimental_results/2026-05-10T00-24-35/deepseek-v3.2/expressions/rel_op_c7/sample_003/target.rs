fn main() {
    let x = 42;
    let p: *const i32 = &x;

    if p != (&x as *const i32) {
        std::process::exit(1);
    }

    unsafe {
        let p_end1 = p.add(1);
        let p_end2 = (&x as *const i32).add(1);

        if p_end1 != p_end2 {
            std::process::exit(2);
        }

        if p_end1 == p {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}