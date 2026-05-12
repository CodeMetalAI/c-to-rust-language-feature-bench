fn main() -> i32 {
    let x = 42;
    let p = &x;

    if p!= &x {
        1
    } else {
        let p_end1 = p.offset(1);
        let p_end2 = &x.offset(1);

        if p_end1!= p_end2 {
            2
        } else if p_end1 == p {
            3
        } else {
            0
        }
    }
}