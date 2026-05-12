fn main() {
    let mut x = 1;
    let px = &x;

    let ip = px as usize;
    let p2 = px as *mut i32;

    if ip != p2 as usize {
        return 1;
    }

    let z = 1;
    let vp = z as *mut u8;
    let z2 = vp as usize;

    if z2 != z {
        return 2;
    }

    0
}