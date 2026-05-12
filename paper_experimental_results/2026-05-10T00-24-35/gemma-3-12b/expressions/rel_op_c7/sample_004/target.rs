fn main() {
    let x = 42;
    let p = &x;

    if p != &x {
        return 1;
    }

    let p_end1 = unsafe { p.add(1) };
    let p_end2 = unsafe { &x.add(1) };

    if p_end1 != p_end2 {
        return 2;
    }

    if p_end1 == p {
        return 3;
    }

    0
}