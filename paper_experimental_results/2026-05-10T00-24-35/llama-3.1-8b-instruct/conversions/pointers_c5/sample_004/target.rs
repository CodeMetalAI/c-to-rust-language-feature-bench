fn main() {
    let x = 1;
    let px = &x;

    let ip = px as usize;
    let p2 = unsafe { ip as *mut i32 };

    if p2 as usize != ip {
        return 1;
    }

    let z = 1;
    let vp = &z as *const i32;
    let z2 = vp as usize;

    if z2 != z {
        return 2;
    }

    return 0;
}