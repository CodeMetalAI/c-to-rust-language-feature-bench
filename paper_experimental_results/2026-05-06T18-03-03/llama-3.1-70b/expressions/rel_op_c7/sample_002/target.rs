fn main() {
    let x = 42;
    let p = &x as *const i32;

    if p!= &x as *const i32 {
        return 1;
    }

    let p_end1 = p.offset(1);
    let p_end2 = &x as *const i32;

    if p_end1!= p_end2.offset(1) {
        return 2;
    }

    if p_end1 == p {
        return 3;
    }

    return 0;
}