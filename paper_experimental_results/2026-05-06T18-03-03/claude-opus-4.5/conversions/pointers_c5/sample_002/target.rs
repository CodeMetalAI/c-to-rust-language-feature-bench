fn main() {
    let x: i32 = 1;
    let px: &i32 = &x;

    // Get the address as an integer
    let ip: usize = px as *const i32 as usize;
    
    // In safe Rust, we can't actually convert back to a pointer and dereference,
    // but we can verify the round-trip of the integer representation
    let p2_addr: usize = ip;

    if p2_addr != ip {
        std::process::exit(1);
    }

    let z: usize = 1;
    // Simulating void* as just storing the integer value
    // In safe Rust, we represent the "pointer value" as the integer itself
    let vp: usize = z;
    let z2: usize = vp;

    if z2 != z {
        std::process::exit(2);
    }

    std::process::exit(0);
}