fn main() -> i32 {
    let x = 1;
    let px: *const i32 = &x;

    let ip = px as usize;
    let p2: *const i32 = ip as *const i32;

    if p2 as usize != ip {
        return 1;
    }

    let z: usize = 1;
    let vp: *const () = z as *const ();
    let z2 = vp as usize;

    if z2 != z {
        return 2;
    }

    0
}