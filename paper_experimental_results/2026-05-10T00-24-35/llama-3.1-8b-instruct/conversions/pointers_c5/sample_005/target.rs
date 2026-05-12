fn main() {
    let mut x = 1;
    let px = &x;

    let ip: usize = px as usize;
    let p2: *const i32 = ip as *const i32;

    if p2 as usize != ip {
        return 1;
    }

    let mut z = 1;
    let vp = z;
    let z2 = vp as usize;

    if z2 != z {
        return 2;
    }

    return 0;
}