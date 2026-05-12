fn main() {
    let x = 42;
    let p = &x;

    if p as *const _ != &x as *const _ {
        std::process::exit(1);
    }

    let p_end1 = (p as *const i32).wrapping_add(1);
    let p_end2 = (&x as *const i32).wrapping_add(1);

    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    if p_end1 == p as *const _ {
        std::process::exit(3);
    }

    std::process::exit(0);
}