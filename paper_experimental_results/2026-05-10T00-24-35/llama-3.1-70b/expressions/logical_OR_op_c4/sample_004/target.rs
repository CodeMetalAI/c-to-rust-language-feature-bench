fn main() -> i32 {
    let mut g = 0;

    fn set_g(gval: i32) -> bool {
        g = gval;
        false
    }

    g = 0;
    if !set_g(1) || g != 1 {
        return 1;
    }

    g = 0;
    let mut y = 0;
    if set_g(0) || { y += 1; true } {
        return 2;
    }
    if y != 0 {
        return 3;
    }

    0
}