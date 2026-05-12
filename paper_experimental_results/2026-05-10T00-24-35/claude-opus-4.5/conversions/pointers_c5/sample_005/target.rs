fn main() {
    let x: i32 = 1;
    let px: &i32 = &x;

    let ip: usize = px as *const i32 as usize;
    let p2: usize = ip;

    if p2 != ip {
        std::process::exit(1);
    }

    let z: usize = 1;
    let vp: usize = z;
    let z2: usize = vp;

    if z2 != z {
        std::process::exit(2);
    }
}