fn main() {
    let x: i32 = 1;
    let px: *const i32 = &x;

    let ip: usize = px as usize;
    let p2: *const i32 = ip as *const i32;

    if ip!= p2 as usize {
        return 1;
    }

    let z: usize = 1;
    let vp: *const usize = &z;
    let z2: usize = vp as usize;

    if z2!= z {
        return 2;
    }

    return 0;
}