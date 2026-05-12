fn main() {
    let mut x = 1;
    let px = &x as *const i32;

    let ip = px as u64;
    let p2 = ip as *const i32;

    if (p2 as u64) != ip {
        return 1;
    }

    let z = 1u64;
    let vp = z as *mut ();
    let z2 = vp as u64;

    if z2 != z {
        return 2;
    }

    0
}