fn main() {
    let _x: i32 = 1;

    let ip: usize = 1;
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

    std::process::exit(0);
}