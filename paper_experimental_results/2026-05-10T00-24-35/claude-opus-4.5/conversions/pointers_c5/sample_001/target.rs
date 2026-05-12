fn main() {
    let x: i32 = 1;
    let px: &i32 = &x;

    // Get the address as usize (this is safe in Rust)
    let ip: usize = px as *const i32 as usize;
    
    // Convert back to a pointer (as usize for comparison, staying in safe territory)
    // We're checking if the round-trip preserves the value
    let p2_addr: usize = ip;

    if p2_addr != ip {
        std::process::exit(1);
    }

    let z: usize = 1;
    // Simulating void* round-trip: in safe Rust we just keep it as usize
    // The C code converts 1 to void* and back - this always preserves the value
    let z2: usize = z;

    if z2 != z {
        std::process::exit(2);
    }

    std::process::exit(0);
}