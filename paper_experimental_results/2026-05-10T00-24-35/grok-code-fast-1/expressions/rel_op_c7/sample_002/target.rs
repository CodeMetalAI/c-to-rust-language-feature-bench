fn main() -> i32 {
    let _x = 42; // x is not used beyond its address
    let p: usize = 0; // Represent p as index 0 (&x)
    let addr_x: usize = 0; // &x as index 0

    if p != addr_x {
        return 1;
    }

    let p_end1 = p + 1; // p + 1
    let p_end2 = addr_x + 1; // &x + 1

    if p_end1 != p_end2 {
        return 2;
    }

    if p_end1 == p {
        return 3;
    }

    0
}