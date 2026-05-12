fn main() {
    let x = 17;
    let p = &x;

    let pc: &i32 = p;
    let pv: &i32 = p;
    let pcv: &i32 = p;

    // In Rust, all these pointers point to the same location,
    // thus no need to explicitly compare addresses.
    if *pc != 17 {
        std::process::exit(4);
    }
    if *pv != 17 {
        std::process::exit(5);
    }
    if *pcv != 17 {
        std::process::exit(6);
    }
}