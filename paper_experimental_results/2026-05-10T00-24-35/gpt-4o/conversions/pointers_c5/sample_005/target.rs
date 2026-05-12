fn main() {
    let x: i32 = 1;
    let px: *const i32 = &x;

    let ip = px as usize;
    let p2 = ip as *const i32;

    if (p2 as usize) != ip {
        std::process::exit(1);
    }

    let z: usize = 1;
    let vp = z as *const ();
    let z2 = vp as usize;

    if z2 != z {
        std::process::exit(2);
    }
}