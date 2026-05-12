fn main() {
    let mut x = 1;
    let px = &mut x as *mut i32;

    let ip = px as usize;
    let p2 = ip as *mut i32;

    if p2 as usize != ip {
        std::process::exit(1);
    }

    let z = 1usize;
    let vp = z as *const u8;
    let z2 = vp as usize;

    if z2 != z {
        std::process::exit(2);
    }

    std::process::exit(0);
}